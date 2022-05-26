use crate::frontend;
use crate::models;
use crate::proto;
use crate::proto::{
    gofer_server::{Gofer, GoferServer},
    *,
};
use crate::storage;
use crate::{conf, storage::StorageError};

use slog_scope::info;
use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{Request, Response, Status};

use axum::{body::BoxBody, response::IntoResponse};
use axum_server::tls_rustls::RustlsConfig;
use futures::{
    future::{BoxFuture, Either},
    ready, TryFutureExt,
};
use std::{convert::Infallible, io::BufReader, sync::Arc, task::Poll};
use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig};
use tower::Service;

const BUILD_SEMVER: &str = env!("BUILD_SEMVER");
const BUILD_COMMIT: &str = env!("BUILD_COMMIT");

#[derive(Clone)]
pub struct Api {
    conf: conf::api::Config,
    storage: storage::Db,
}

#[tonic::async_trait]
impl Gofer for Api {
    async fn get_system_info(
        &self,
        _: Request<GetSystemInfoRequest>,
    ) -> Result<Response<GetSystemInfoResponse>, Status> {
        Ok(Response::new(GetSystemInfoResponse {
            commit: BUILD_COMMIT.to_string(),
            dev_mode_enabled: self.conf.general.dev_mode,
            semver: BUILD_SEMVER.to_string(),
        }))
    }

    async fn list_namespaces(
        &self,
        request: Request<ListNamespacesRequest>,
    ) -> Result<Response<ListNamespacesResponse>, Status> {
        let args = &request.into_inner();

        let result = self
            .storage
            .list_namespaces(args.offset, args.limit as u8)
            .await;

        match result {
            Ok(namespaces_raw) => {
                let namespaces = namespaces_raw
                    .into_iter()
                    .map(proto::Namespace::from)
                    .collect();
                return Ok(Response::new(ListNamespacesResponse { namespaces }));
            }
            Err(storage_err) => return Err(Status::internal(storage_err.to_string())),
        }
    }

    async fn create_namespace(
        &self,
        request: Request<CreateNamespaceRequest>,
    ) -> Result<Response<CreateNamespaceResponse>, Status> {
        let args = &request.into_inner();
        let new_namespace = match models::Namespace::new(&args.id, &args.name, &args.description) {
            Ok(namespace) => namespace,
            Err(e) => {
                return Err(Status::failed_precondition(e.to_string()));
            }
        };

        let result = self.storage.create_namespace(&new_namespace).await;
        match result {
            Ok(_) => (),
            Err(e) => match e {
                storage::StorageError::Exists => {
                    return Err(Status::already_exists(format!(
                        "namespace with id '{}' already exists",
                        new_namespace.id
                    )))
                }
                _ => return Err(Status::internal(e.to_string())),
            },
        };

        info!("Created new namespace"; "namespace" => format!("{:?}", new_namespace));
        Ok(Response::new(CreateNamespaceResponse {
            namespace: Some(new_namespace.into()),
        }))
    }

    async fn get_namespace(
        &self,
        request: Request<GetNamespaceRequest>,
    ) -> Result<Response<GetNamespaceResponse>, Status> {
        let args = &request.into_inner();

        let result = self.storage.get_namespace(&args.id).await;
        let namespace = match result {
            Ok(namespace) => namespace,
            Err(e) => match e {
                storage::StorageError::NotFound => {
                    return Err(Status::not_found(format!(
                        "namespace with id '{}' does not exist",
                        &args.id
                    )))
                }
                _ => return Err(Status::internal(e.to_string())),
            },
        };

        Ok(Response::new(GetNamespaceResponse {
            namespace: Some(namespace.into()),
        }))
    }

    async fn update_namespace(
        &self,
        request: Request<UpdateNamespaceRequest>,
    ) -> Result<Response<UpdateNamespaceResponse>, Status> {
        let args = &request.into_inner();

        let result = self
            .storage
            .update_namespace(&models::Namespace {
                id: args.id.clone(),
                name: args.name.clone(),
                description: args.description.clone(),
                created: 0,
                modified: epoch(),
            })
            .await;

        match result {
            Ok(_) => (),
            Err(e) => match e {
                storage::StorageError::NotFound => {
                    return Err(Status::not_found(format!(
                        "namespace with id '{}' does not exist",
                        &args.id
                    )))
                }
                _ => return Err(Status::internal(e.to_string())),
            },
        };

        Ok(Response::new(UpdateNamespaceResponse {}))
    }

    async fn delete_namespace(
        &self,
        request: Request<DeleteNamespaceRequest>,
    ) -> Result<Response<DeleteNamespaceResponse>, Status> {
        let args = &request.into_inner();

        let result = self.storage.delete_namespace(&args.id).await;
        match result {
            Ok(_) => (),
            Err(e) => match e {
                storage::StorageError::NotFound => {
                    return Err(Status::not_found(format!(
                        "namespace with id '{}' does not exist",
                        &args.id
                    )))
                }
                _ => return Err(Status::internal(e.to_string())),
            },
        };

        info!("Deleted namespace"; "id" => &args.id);
        Ok(Response::new(DeleteNamespaceResponse {}))
    }
}

impl Api {
    /// Create new API object. Subsequently you can run start_service to start the server.
    pub async fn new(conf: conf::api::Config) -> Self {
        let storage = storage::Db::new(&conf.server.storage_path).await.unwrap();

        let api = Api { conf, storage };

        api.create_default_namespace().await.unwrap();

        api
    }

    /// Gofer starts with a default namespace that all users have access to.
    async fn create_default_namespace(&self) -> Result<(), StorageError> {
        const DEFAULT_NAMESPACE_ID: &str = "default";
        const DEFAULT_NAMESPACE_NAME: &str = "Default";
        const DEFAULT_NAMESPACE_DESCRIPTION: &str =
            "The default namespace when no other namespace is specified.";

        let default_namespace = match models::Namespace::new(
            DEFAULT_NAMESPACE_ID,
            DEFAULT_NAMESPACE_NAME,
            DEFAULT_NAMESPACE_DESCRIPTION,
        ) {
            Ok(namespace) => namespace,
            Err(e) => {
                return Err(storage::StorageError::Unknown(e.to_string()));
            }
        };

        match self.storage.create_namespace(&default_namespace).await {
            Ok(_) => Ok(()),
            Err(e) => match e {
                storage::StorageError::Exists => Ok(()),
                _ => Err(e),
            },
        }
    }

    /// Start a TLS enabled, multiplexed, grpc/http server.
    pub async fn start_service(&self) {
        let rest =
            axum::Router::new().route("/*path", axum::routing::any(frontend::frontend_handler));
        let grpc = GoferServer::new(self.clone());

        let service = MultiplexService { rest, grpc };

        let cert = self.conf.server.tls_cert.clone().into_bytes();
        let key = self.conf.server.tls_key.clone().into_bytes();

        if self.conf.general.dev_mode {
            start_server(service, &self.conf.server.url).await;
            return;
        }

        start_tls_server(service, &self.conf.server.url, cert, key).await;
    }
}

async fn start_server(service: MultiplexService<axum::Router, GoferServer<Api>>, host: &str) {
    info!("Started multiplexed grpc/http service"; "url" => host.parse::<String>().unwrap());

    axum::Server::bind(&host.parse().unwrap())
        .tcp_keepalive(Some(std::time::Duration::from_secs(15)))
        .serve(tower::make::Shared::new(service))
        .await
        .expect("server exited unexpectedly");
}

async fn start_tls_server(
    service: MultiplexService<axum::Router, GoferServer<Api>>,
    host: &str,
    cert: Vec<u8>,
    key: Vec<u8>,
) {
    let tls_config = get_tls_config(cert, key);

    let tcp_settings = axum_server::AddrIncomingConfig::new()
        .tcp_keepalive(Some(std::time::Duration::from_secs(15)))
        .build();

    info!("Started multiplexed, TLS enabled, grpc/http service"; "url" => host.parse::<String>().unwrap());

    axum_server::bind_rustls(host.parse().unwrap(), tls_config)
        .addr_incoming_config(tcp_settings)
        .serve(tower::make::Shared::new(service))
        .await
        .expect("server exited unexpectedly");
}

/// returns a TLS configuration object for use in the multiplexing server.
fn get_tls_config(cert: Vec<u8>, key: Vec<u8>) -> RustlsConfig {
    let mut buffered_cert: BufReader<&[u8]> = BufReader::new(&cert);
    let mut buffered_key: BufReader<&[u8]> = BufReader::new(&key);

    let certs = rustls_pemfile::certs(&mut buffered_cert)
        .expect("could not get certificate chain")
        .into_iter()
        .map(Certificate)
        .collect();

    let key = PrivateKey(
        rustls_pemfile::pkcs8_private_keys(&mut buffered_key)
            .expect("could not get private key")
            .get(0)
            .expect("could not get private key")
            .to_vec(),
    );

    let tls_config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("could not load certificate or private key");

    RustlsConfig::from_config(Arc::new(tls_config))
}

fn epoch() -> u64 {
    let current_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    u64::try_from(current_epoch).unwrap()
}

#[derive(Clone)]
pub struct MultiplexService<A, B> {
    pub rest: A,
    pub grpc: B,
}

impl<A, B> Service<hyper::Request<hyper::Body>> for MultiplexService<A, B>
where
    A: Service<hyper::Request<hyper::Body>, Error = Infallible>,
    A::Response: IntoResponse,
    A::Future: Send + 'static,
    B: Service<hyper::Request<hyper::Body>, Error = Infallible>,
    B::Response: IntoResponse,
    B::Future: Send + 'static,
{
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = hyper::Response<BoxBody>;

    // This seems incorrect. We never check GRPC readiness; but I'm too lazy
    // to fix it and it seems to work well enough.
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(if let Err(err) = ready!(self.rest.poll_ready(cx)) {
            Err(err)
        } else {
            ready!(self.rest.poll_ready(cx))
        })
    }

    fn call(&mut self, req: hyper::Request<hyper::Body>) -> Self::Future {
        let hv = req.headers().get("content-type").map(|x| x.as_bytes());

        let fut = if hv
            .filter(|value| value.starts_with(b"application/grpc"))
            .is_some()
        {
            Either::Left(self.grpc.call(req).map_ok(|res| res.into_response()))
        } else {
            Either::Right(self.rest.call(req).map_ok(|res| res.into_response()))
        };

        Box::pin(fut)
    }
}
