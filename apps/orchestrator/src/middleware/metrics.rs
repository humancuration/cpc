use tonic::{Request, Response, Status};
use crate::cpc_orchestrator::NodeOrchestration;
use crate::metrics::MetricsService;
use std::sync::Arc;

pub struct MetricsMiddleware<T> {
    inner: T,
    metrics: Arc<MetricsService>,
}

impl<T> MetricsMiddleware<T> {
    pub fn new(inner: T, metrics: Arc<MetricsService>) -> Self {
        Self { inner, metrics }
    }
}

#[tonic::async_trait]
impl<T: NodeOrchestration + Send + Sync + 'static> NodeOrchestration for MetricsMiddleware<T> {
    async fn register_node(
        &self,
        request: Request<NodeInfo>,
    ) -> Result<Response<RegistrationResponse>, Status> {
        self.metrics.record_pre_handle("register_node");
        let result = self.inner.register_node(request).await;
        self.metrics.record_post_handle("register_node", &result);
        result
    }

    async fn heartbeat(
        &self,
        request: Request<NodePing>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        self.metrics.record_pre_handle("heartbeat");
        let result = self.inner.heartbeat(request).await;
        self.metrics.record_post_handle("heartbeat", &result);
        result
    }

    async fn request_resources(
        &self,
        request: Request<ResourceRequest>,
    ) -> Result<Response<ResourceAllocation>, Status> {
        self.metrics.record_pre_handle("request_resources");
        let result = self.inner.request_resources(request).await;
        self.metrics.record_post_handle("request_resources", &result);
        result
    }
}