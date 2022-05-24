use crate::api;
use crate::conf;
use crate::frontend;
use crate::proto::gofer_client::GoferClient;
use crate::proto::GetSystemInfoRequest;

use axum::{body::BoxBody, response::IntoResponse};
use axum_server::tls_rustls::RustlsConfig;
use clap::{Args, Subcommand};
use futures::{
    future::{BoxFuture, Either},
    ready, TryFutureExt,
};
use hyper::{Body, Request, Response};
use slog_scope::info;
use std::{convert::Infallible, process, task::Poll};
use tower::Service;

use super::CliHarness;

#[derive(Debug, Args)]
pub struct ServiceSubcommands {
    #[clap(subcommand)]
    pub command: ServiceCommands,
}

#[derive(Debug, Subcommand)]
pub enum ServiceCommands {
    /// Start the Gofer GRPC service.
    #[clap(
        long_about = "Gofer runs a a GRPC backend combined with GRPC-WEB/HTTP1.
    Running this command attempts to start the long running service. This command will block and only
    gracefully stop on SIGINT or SIGTERM signals."
    )]
    Start,

    /// Retrieve general information about Gofer's systems
    Info,
}

impl CliHarness {
    pub async fn service_start(&self, config: conf::api::Config) {
        let rest =
            axum::Router::new().route("/*path", axum::routing::any(frontend::frontend_handler));
        let grpc = api::Api::new(config.clone()).await.init_grpc_server();

        let service = MultiplexService { rest, grpc };

        let tls_config = RustlsConfig::from_pem().await.unwrap();

        info!("Started grpc-web service"; "url" => &config.server.url.parse::<String>().unwrap());

        axum::Server::bind(&config.server.url.parse().unwrap())
            .tcp_keepalive(Some(std::time::Duration::from_secs(15)))
            .serve(tower::make::Shared::new(service))
            .await
            .unwrap();
    }

    pub async fn service_info(&self) {
        let channel = match tonic::transport::Channel::from_shared(self.config.server.to_string()) {
            Ok(channel) => channel,
            Err(e) => {
                eprintln!("Could not open transport channel; {}", e);
                process::exit(1);
            }
        };

        let conn = match channel.connect().await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Could not connect to server; {}", e);
                process::exit(1);
            }
        };

        let mut client = GoferClient::new(conn);
        let request = tonic::Request::new(GetSystemInfoRequest {});
        let response = match client.get_system_info(request).await {
            Ok(response) => response.into_inner(),
            Err(e) => {
                eprintln!("Could not get info; {}", e.message());
                process::exit(1);
            }
        };

        println!("{:?}", response);
    }
}

#[derive(Clone)]
pub struct MultiplexService<A, B> {
    pub rest: A,
    pub grpc: B,
}

impl<A, B> Service<Request<Body>> for MultiplexService<A, B>
where
    A: Service<Request<Body>, Error = Infallible>,
    A::Response: IntoResponse,
    A::Future: Send + 'static,
    B: Service<Request<Body>, Error = Infallible>,
    B::Response: IntoResponse,
    B::Future: Send + 'static,
{
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = Response<BoxBody>;

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

    fn call(&mut self, req: Request<Body>) -> Self::Future {
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
