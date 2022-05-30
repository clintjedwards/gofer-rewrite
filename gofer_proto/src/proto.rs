// Documentation for these models can be found in the "models" package as these
// are just protobuf representations of those models.
//
// Why represent what amounts to the same model twice in protobuf AND a struct
// you ask?
//
//   Essentially, because the separation of the transport layer and the
//   application layer is a good thing. There are probably many reasons, but
//   the most straightforward is that we might want to represent something in
//   the database or within the application that might not be easily
//   representable in protobuf, a structure mainly made for transport.
//
//   There might also be things that we don't want to expose outside and so
//   the separation gives us a chance to not mess that up by simply forgetting a
//   json:"-".

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Namespace {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub created: u64,
    #[prost(uint64, tag="5")]
    pub modified: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Variable {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
    #[prost(enumeration="VariableOwner", tag="3")]
    pub owner: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pipeline {
    #[prost(string, tag="1")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub last_run_id: u64,
    #[prost(uint64, tag="6")]
    pub last_run_time: u64,
    #[prost(uint64, tag="7")]
    pub parallelism: u64,
    #[prost(uint64, tag="8")]
    pub created: u64,
    #[prost(uint64, tag="9")]
    pub modified: u64,
    #[prost(enumeration="pipeline::PipelineState", tag="10")]
    pub state: i32,
    #[prost(map="string, message", tag="11")]
    pub tasks: ::std::collections::HashMap<::prost::alloc::string::String, Task>,
    #[prost(map="string, message", tag="12")]
    pub triggers: ::std::collections::HashMap<::prost::alloc::string::String, PipelineTriggerSettings>,
    #[prost(map="string, message", tag="13")]
    pub notifiers: ::std::collections::HashMap<::prost::alloc::string::String, PipelineNotifierSettings>,
    #[prost(string, repeated, tag="14")]
    pub store_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Pipeline`.
pub mod pipeline {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PipelineState {
        Unknown = 0,
        Active = 1,
        Disabled = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineConfig {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub parallelism: u64,
    #[prost(message, repeated, tag="5")]
    pub tasks: ::prost::alloc::vec::Vec<TaskConfig>,
    #[prost(message, repeated, tag="6")]
    pub triggers: ::prost::alloc::vec::Vec<PipelineTriggerConfig>,
    #[prost(message, repeated, tag="7")]
    pub notifiers: ::prost::alloc::vec::Vec<PipelineNotifierConfig>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Run {
    #[prost(string, tag="1")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pipeline: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub id: u64,
    #[prost(uint64, tag="4")]
    pub started: u64,
    #[prost(uint64, tag="5")]
    pub ended: u64,
    #[prost(enumeration="run::RunState", tag="6")]
    pub state: i32,
    #[prost(enumeration="run::RunStatus", tag="7")]
    pub status: i32,
    #[prost(message, optional, tag="8")]
    pub failure_info: ::core::option::Option<RunFailureInfo>,
    #[prost(string, repeated, tag="9")]
    pub task_runs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="10")]
    pub trigger: ::core::option::Option<RunTriggerInfo>,
    #[prost(message, repeated, tag="11")]
    pub variables: ::prost::alloc::vec::Vec<Variable>,
    #[prost(message, optional, tag="12")]
    pub store_info: ::core::option::Option<RunStoreInfo>,
}
/// Nested message and enum types in `Run`.
pub mod run {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RunState {
        Unknown = 0,
        Pending = 1,
        Running = 2,
        Complete = 3,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RunStatus {
        Unknown = 0,
        Successful = 1,
        Failed = 2,
        Cancelled = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunFailureInfo {
    #[prost(enumeration="run_failure_info::RunFailureReason", tag="1")]
    pub reason: i32,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RunFailureInfo`.
pub mod run_failure_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RunFailureReason {
        Unknown = 0,
        AbnormalExit = 1,
        SchedulerError = 2,
        FailedPrecondition = 3,
        UserCancelled = 4,
        AdminCancelled = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTriggerInfo {
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub label: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunStoreInfo {
    #[prost(bool, tag="1")]
    pub is_expired: bool,
    #[prost(string, repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistryAuth {
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pass: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exec {
    #[prost(string, tag="1")]
    pub shell: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub script: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub image: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub registry_auth: ::core::option::Option<RegistryAuth>,
    #[prost(map="string, enumeration(task::RequiredParentStatus)", tag="5")]
    pub depends_on: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    #[prost(message, repeated, tag="6")]
    pub variables: ::prost::alloc::vec::Vec<Variable>,
    #[prost(message, optional, tag="7")]
    pub exec: ::core::option::Option<Exec>,
}
/// Nested message and enum types in `Task`.
pub mod task {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RequiredParentStatus {
        Unknown = 0,
        Any = 1,
        Success = 2,
        Failure = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineTriggerSettings {
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub label: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="3")]
    pub settings: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub error: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineNotifierSettings {
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub label: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="3")]
    pub settings: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub error: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskConfig {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub image: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub registry_auth: ::core::option::Option<RegistryAuth>,
    #[prost(map="string, enumeration(task_config::RequiredParentStatus)", tag="5")]
    pub depends_on: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    #[prost(map="string, string", tag="6")]
    pub variables: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(message, optional, tag="7")]
    pub exec: ::core::option::Option<Exec>,
}
/// Nested message and enum types in `TaskConfig`.
pub mod task_config {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RequiredParentStatus {
        Unknown = 0,
        Any = 1,
        Success = 2,
        Failure = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineTriggerConfig {
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub label: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="3")]
    pub settings: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineNotifierConfig {
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub label: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="3")]
    pub settings: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VariableOwner {
    Unknown = 0,
    User = 1,
    System = 2,
}
////////////// System Transport Models //////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSystemInfoRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSystemInfoResponse {
    #[prost(string, tag="1")]
    pub commit: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub dev_mode_enabled: bool,
    #[prost(string, tag="3")]
    pub semver: ::prost::alloc::string::String,
}
////////////// Namespace Transport Models //////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNamespaceRequest {
    /// Unique identifier
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNamespaceResponse {
    #[prost(message, optional, tag="1")]
    pub namespace: ::core::option::Option<Namespace>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNamespacesRequest {
    /// offset is a pagination parameter that defines where to start when counting
    /// the list of objects to return.
    #[prost(uint64, tag="1")]
    pub offset: u64,
    /// limit is a pagination parameter that defines how many objects to return
    /// per result.
    #[prost(uint64, tag="2")]
    pub limit: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNamespacesResponse {
    #[prost(message, repeated, tag="1")]
    pub namespaces: ::prost::alloc::vec::Vec<Namespace>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNamespaceRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNamespaceResponse {
    #[prost(message, optional, tag="1")]
    pub namespace: ::core::option::Option<Namespace>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNamespaceRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNamespaceResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNamespaceRequest {
    /// Unique identifier
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNamespaceResponse {
}
////////////// Pipeline Transport Models //////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPipelineRequest {
    /// Unique namespace identifier
    #[prost(string, tag="1")]
    pub namespace_id: ::prost::alloc::string::String,
    /// Unique identifier
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPipelineResponse {
    #[prost(message, optional, tag="1")]
    pub pipeline: ::core::option::Option<Pipeline>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPipelinesRequest {
    /// offset is a pagination parameter that defines where to start when counting
    /// the list of pipelines to return.
    #[prost(int64, tag="1")]
    pub offset: i64,
    /// limit is a pagination parameter that defines how many pipelines to return
    /// per result.
    #[prost(int64, tag="2")]
    pub limit: i64,
    /// Unique namespace identifier
    #[prost(string, tag="3")]
    pub namespace_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPipelinesResponse {
    #[prost(message, repeated, tag="1")]
    pub pipelines: ::prost::alloc::vec::Vec<Pipeline>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisablePipelineRequest {
    /// Unique namespace identifier
    #[prost(string, tag="1")]
    pub namespace_id: ::prost::alloc::string::String,
    /// Unique namespace identifier
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisablePipelineResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnablePipelineRequest {
    /// Unique namespace identifier
    #[prost(string, tag="1")]
    pub namespace_id: ::prost::alloc::string::String,
    /// Unique identifier
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnablePipelineResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePipelineRequest {
    /// Unique namespace identifier
    #[prost(string, tag="1")]
    pub namespace_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub pipeline_config: ::core::option::Option<PipelineConfig>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePipelineResponse {
    #[prost(message, optional, tag="1")]
    pub pipeline: ::core::option::Option<Pipeline>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePipelineRequest {
    /// Unique namespace identifier
    #[prost(string, tag="1")]
    pub namespace_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub pipeline_config: ::core::option::Option<PipelineConfig>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePipelineResponse {
    #[prost(message, optional, tag="1")]
    pub pipeline: ::core::option::Option<Pipeline>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePipelineRequest {
    /// Unique namespace identifier
    #[prost(string, tag="1")]
    pub namespace_id: ::prost::alloc::string::String,
    /// Pipeline ID
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePipelineResponse {
}
/// Generated client implementations.
pub mod gofer_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct GoferClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GoferClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GoferClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GoferClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            GoferClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /////////////// System RPCs //////////////
        pub async fn get_system_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSystemInfoRequest>,
        ) -> Result<tonic::Response<super::GetSystemInfoResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Gofer/GetSystemInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ListNamespaces returns all registered namespaces.
        pub async fn list_namespaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNamespacesRequest>,
        ) -> Result<tonic::Response<super::ListNamespacesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Gofer/ListNamespaces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// CreateNamespace creates a new namespace that separates pipelines.
        pub async fn create_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNamespaceRequest>,
        ) -> Result<tonic::Response<super::CreateNamespaceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Gofer/CreateNamespace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetNamespace returns a single namespace by id.
        pub async fn get_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNamespaceRequest>,
        ) -> Result<tonic::Response<super::GetNamespaceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Gofer/GetNamespace");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpdateNamespace updates the details of a particular namespace by id.
        pub async fn update_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNamespaceRequest>,
        ) -> Result<tonic::Response<super::UpdateNamespaceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Gofer/UpdateNamespace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteNamespace removes a namespace by id.
        pub async fn delete_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNamespaceRequest>,
        ) -> Result<tonic::Response<super::DeleteNamespaceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Gofer/DeleteNamespace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetPipeline returns a single pipeline by ID.
        pub async fn get_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPipelineRequest>,
        ) -> Result<tonic::Response<super::GetPipelineResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Gofer/GetPipeline");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ListPipelines returns all registered pipelines. Can control pagination by
        /// offset && limit request parameters.
        /// By default ListPipelines will return the first 100 pipelines ordered by
        /// creation.
        pub async fn list_pipelines(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPipelinesRequest>,
        ) -> Result<tonic::Response<super::ListPipelinesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Gofer/ListPipelines",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// EnablePipeline allows a pipeline to execute runs by allowing it to receive
        /// trigger events. See DisablePipeline to prevent a pipeline from executing
        /// any more runs.
        pub async fn enable_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::EnablePipelineRequest>,
        ) -> Result<tonic::Response<super::EnablePipelineResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Gofer/EnablePipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DisablePipeline prevents the pipeline from executing runs. Any trigger
        /// events that would normally cause the pipeline to be run are instead
        /// discarded.
        pub async fn disable_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::DisablePipelineRequest>,
        ) -> Result<tonic::Response<super::DisablePipelineResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Gofer/DisablePipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// CreatePipeline creates a new pipeline from the protobuf input. This is
        /// usually autogenerated from the command line tool.
        pub async fn create_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePipelineRequest>,
        ) -> Result<tonic::Response<super::CreatePipelineResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Gofer/CreatePipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpdatePipeline updates a pipeline from the protobuf input. This input is
        /// usually autogenerated from the command line tool.
        /// Updating a pipeline requires the pipeline to adhere
        /// to two constraints:
        ///    1) The pipeline must not have any current runs in progress.
        ///    2) The pipeline must be in a disabled state.
        pub async fn update_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePipelineRequest>,
        ) -> Result<tonic::Response<super::UpdatePipelineResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Gofer/UpdatePipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeletePipeline deletes a pipeline permenantly. It is not recoverable.
        pub async fn delete_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePipelineRequest>,
        ) -> Result<tonic::Response<super::DeletePipelineResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Gofer/DeletePipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod gofer_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with GoferServer.
    #[async_trait]
    pub trait Gofer: Send + Sync + 'static {
        /////////////// System RPCs //////////////
        async fn get_system_info(
            &self,
            request: tonic::Request<super::GetSystemInfoRequest>,
        ) -> Result<tonic::Response<super::GetSystemInfoResponse>, tonic::Status>;
        /// ListNamespaces returns all registered namespaces.
        async fn list_namespaces(
            &self,
            request: tonic::Request<super::ListNamespacesRequest>,
        ) -> Result<tonic::Response<super::ListNamespacesResponse>, tonic::Status>;
        /// CreateNamespace creates a new namespace that separates pipelines.
        async fn create_namespace(
            &self,
            request: tonic::Request<super::CreateNamespaceRequest>,
        ) -> Result<tonic::Response<super::CreateNamespaceResponse>, tonic::Status>;
        /// GetNamespace returns a single namespace by id.
        async fn get_namespace(
            &self,
            request: tonic::Request<super::GetNamespaceRequest>,
        ) -> Result<tonic::Response<super::GetNamespaceResponse>, tonic::Status>;
        /// UpdateNamespace updates the details of a particular namespace by id.
        async fn update_namespace(
            &self,
            request: tonic::Request<super::UpdateNamespaceRequest>,
        ) -> Result<tonic::Response<super::UpdateNamespaceResponse>, tonic::Status>;
        /// DeleteNamespace removes a namespace by id.
        async fn delete_namespace(
            &self,
            request: tonic::Request<super::DeleteNamespaceRequest>,
        ) -> Result<tonic::Response<super::DeleteNamespaceResponse>, tonic::Status>;
        /// GetPipeline returns a single pipeline by ID.
        async fn get_pipeline(
            &self,
            request: tonic::Request<super::GetPipelineRequest>,
        ) -> Result<tonic::Response<super::GetPipelineResponse>, tonic::Status>;
        /// ListPipelines returns all registered pipelines. Can control pagination by
        /// offset && limit request parameters.
        /// By default ListPipelines will return the first 100 pipelines ordered by
        /// creation.
        async fn list_pipelines(
            &self,
            request: tonic::Request<super::ListPipelinesRequest>,
        ) -> Result<tonic::Response<super::ListPipelinesResponse>, tonic::Status>;
        /// EnablePipeline allows a pipeline to execute runs by allowing it to receive
        /// trigger events. See DisablePipeline to prevent a pipeline from executing
        /// any more runs.
        async fn enable_pipeline(
            &self,
            request: tonic::Request<super::EnablePipelineRequest>,
        ) -> Result<tonic::Response<super::EnablePipelineResponse>, tonic::Status>;
        /// DisablePipeline prevents the pipeline from executing runs. Any trigger
        /// events that would normally cause the pipeline to be run are instead
        /// discarded.
        async fn disable_pipeline(
            &self,
            request: tonic::Request<super::DisablePipelineRequest>,
        ) -> Result<tonic::Response<super::DisablePipelineResponse>, tonic::Status>;
        /// CreatePipeline creates a new pipeline from the protobuf input. This is
        /// usually autogenerated from the command line tool.
        async fn create_pipeline(
            &self,
            request: tonic::Request<super::CreatePipelineRequest>,
        ) -> Result<tonic::Response<super::CreatePipelineResponse>, tonic::Status>;
        /// UpdatePipeline updates a pipeline from the protobuf input. This input is
        /// usually autogenerated from the command line tool.
        /// Updating a pipeline requires the pipeline to adhere
        /// to two constraints:
        ///    1) The pipeline must not have any current runs in progress.
        ///    2) The pipeline must be in a disabled state.
        async fn update_pipeline(
            &self,
            request: tonic::Request<super::UpdatePipelineRequest>,
        ) -> Result<tonic::Response<super::UpdatePipelineResponse>, tonic::Status>;
        /// DeletePipeline deletes a pipeline permenantly. It is not recoverable.
        async fn delete_pipeline(
            &self,
            request: tonic::Request<super::DeletePipelineRequest>,
        ) -> Result<tonic::Response<super::DeletePipelineResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct GoferServer<T: Gofer> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Gofer> GoferServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GoferServer<T>
    where
        T: Gofer,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/proto.Gofer/GetSystemInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetSystemInfoSvc<T: Gofer>(pub Arc<T>);
                    impl<
                        T: Gofer,
                    > tonic::server::UnaryService<super::GetSystemInfoRequest>
                    for GetSystemInfoSvc<T> {
                        type Response = super::GetSystemInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSystemInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_system_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSystemInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Gofer/ListNamespaces" => {
                    #[allow(non_camel_case_types)]
                    struct ListNamespacesSvc<T: Gofer>(pub Arc<T>);
                    impl<
                        T: Gofer,
                    > tonic::server::UnaryService<super::ListNamespacesRequest>
                    for ListNamespacesSvc<T> {
                        type Response = super::ListNamespacesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListNamespacesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_namespaces(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListNamespacesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Gofer/CreateNamespace" => {
                    #[allow(non_camel_case_types)]
                    struct CreateNamespaceSvc<T: Gofer>(pub Arc<T>);
                    impl<
                        T: Gofer,
                    > tonic::server::UnaryService<super::CreateNamespaceRequest>
                    for CreateNamespaceSvc<T> {
                        type Response = super::CreateNamespaceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateNamespaceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_namespace(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateNamespaceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Gofer/GetNamespace" => {
                    #[allow(non_camel_case_types)]
                    struct GetNamespaceSvc<T: Gofer>(pub Arc<T>);
                    impl<
                        T: Gofer,
                    > tonic::server::UnaryService<super::GetNamespaceRequest>
                    for GetNamespaceSvc<T> {
                        type Response = super::GetNamespaceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNamespaceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_namespace(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNamespaceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Gofer/UpdateNamespace" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateNamespaceSvc<T: Gofer>(pub Arc<T>);
                    impl<
                        T: Gofer,
                    > tonic::server::UnaryService<super::UpdateNamespaceRequest>
                    for UpdateNamespaceSvc<T> {
                        type Response = super::UpdateNamespaceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateNamespaceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_namespace(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateNamespaceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Gofer/DeleteNamespace" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteNamespaceSvc<T: Gofer>(pub Arc<T>);
                    impl<
                        T: Gofer,
                    > tonic::server::UnaryService<super::DeleteNamespaceRequest>
                    for DeleteNamespaceSvc<T> {
                        type Response = super::DeleteNamespaceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteNamespaceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_namespace(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteNamespaceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Gofer/GetPipeline" => {
                    #[allow(non_camel_case_types)]
                    struct GetPipelineSvc<T: Gofer>(pub Arc<T>);
                    impl<T: Gofer> tonic::server::UnaryService<super::GetPipelineRequest>
                    for GetPipelineSvc<T> {
                        type Response = super::GetPipelineResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPipelineRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_pipeline(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPipelineSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Gofer/ListPipelines" => {
                    #[allow(non_camel_case_types)]
                    struct ListPipelinesSvc<T: Gofer>(pub Arc<T>);
                    impl<
                        T: Gofer,
                    > tonic::server::UnaryService<super::ListPipelinesRequest>
                    for ListPipelinesSvc<T> {
                        type Response = super::ListPipelinesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPipelinesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_pipelines(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPipelinesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Gofer/EnablePipeline" => {
                    #[allow(non_camel_case_types)]
                    struct EnablePipelineSvc<T: Gofer>(pub Arc<T>);
                    impl<
                        T: Gofer,
                    > tonic::server::UnaryService<super::EnablePipelineRequest>
                    for EnablePipelineSvc<T> {
                        type Response = super::EnablePipelineResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EnablePipelineRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).enable_pipeline(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EnablePipelineSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Gofer/DisablePipeline" => {
                    #[allow(non_camel_case_types)]
                    struct DisablePipelineSvc<T: Gofer>(pub Arc<T>);
                    impl<
                        T: Gofer,
                    > tonic::server::UnaryService<super::DisablePipelineRequest>
                    for DisablePipelineSvc<T> {
                        type Response = super::DisablePipelineResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DisablePipelineRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).disable_pipeline(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DisablePipelineSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Gofer/CreatePipeline" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePipelineSvc<T: Gofer>(pub Arc<T>);
                    impl<
                        T: Gofer,
                    > tonic::server::UnaryService<super::CreatePipelineRequest>
                    for CreatePipelineSvc<T> {
                        type Response = super::CreatePipelineResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePipelineRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_pipeline(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreatePipelineSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Gofer/UpdatePipeline" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePipelineSvc<T: Gofer>(pub Arc<T>);
                    impl<
                        T: Gofer,
                    > tonic::server::UnaryService<super::UpdatePipelineRequest>
                    for UpdatePipelineSvc<T> {
                        type Response = super::UpdatePipelineResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdatePipelineRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_pipeline(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdatePipelineSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Gofer/DeletePipeline" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePipelineSvc<T: Gofer>(pub Arc<T>);
                    impl<
                        T: Gofer,
                    > tonic::server::UnaryService<super::DeletePipelineRequest>
                    for DeletePipelineSvc<T> {
                        type Response = super::DeletePipelineResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePipelineRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_pipeline(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeletePipelineSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Gofer> Clone for GoferServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Gofer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Gofer> tonic::transport::NamedService for GoferServer<T> {
        const NAME: &'static str = "proto.Gofer";
    }
}
