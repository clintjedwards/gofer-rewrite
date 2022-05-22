use crate::conf;
use crate::models;
use crate::proto;
use crate::proto::{
    gofer_server::{Gofer, GoferServer},
    *,
};
use crate::storage;
use slog_scope::info;
use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{Request, Response, Status};

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
            debug_enabled: self.conf.general.debug,
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
        let new_namespace = models::Namespace::new(&args.id, &args.name, &args.description);

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

        let current_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let result = self
            .storage
            .update_namespace(&models::Namespace {
                id: args.id.clone(),
                name: args.name.clone(),
                description: args.description.clone(),
                created: 0,
                modified: current_epoch,
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
    pub async fn new(conf: conf::api::Config) -> Self {
        let storage = storage::Db::new(&conf.server.storage_path).await.unwrap();

        Api { conf, storage }
    }

    // Return new instance of the Gofer GRPC server.
    pub fn init_grpc_server(&self) -> GoferServer<Api> {
        GoferServer::new(self.clone())
    }
}

//   // ListNamespaces returns all registered namespaces.
//   rpc ListNamespaces(ListNamespacesRequest) returns (ListNamespacesResponse);

//   // CreateNamespace creates a new namespace that separates pipelines.
//   rpc CreateNamespace(CreateNamespaceRequest) returns (CreateNamespaceResponse);

//   // GetNamespace returns a single namespace by id.
//   rpc GetNamespace(GetNamespaceRequest) returns (GetNamespaceResponse);

//   // UpdateNamespace updates the details of a particular namespace by id.
//   rpc UpdateNamespace(UpdateNamespaceRequest) returns (UpdateNamespaceResponse);

//   // DeleteNamespace removes a namespace by id.
//   rpc DeleteNamespace(DeleteNamespaceRequest) returns (DeleteNamespaceResponse);
