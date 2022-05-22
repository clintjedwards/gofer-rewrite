use crate::conf;
use crate::proto;
use crate::proto::{
    gofer_server::{Gofer, GoferServer},
    *,
};
use crate::storage;
use slog_scope::info;
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

        let namespaces: Vec<Namespace>;

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
