use crate::conf;
use crate::proto::{
    gofer_server::{Gofer, GoferServer},
    *,
};
use crate::storage;
use slog_scope::info;
use tonic::{Request, Response, Status};

const BUILD_SEMVER: &str = env!("BUILD_SEMVER");
const BUILD_COMMIT: &str = env!("BUILD_COMMIT");

#[derive(Default, Clone)]
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
            debug_enabled: false,
            semver: BUILD_SEMVER.to_string(),
        }))
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
