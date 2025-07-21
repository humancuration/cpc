use tonic::transport::{Channel, Endpoint};
use crate::grpc::internal::{
    node_orchestration_client::NodeOrchestrationClient, 
    NodeRegistrationRequest,
    HealthCheckRequest,
    MetricsUpdate,
    ReplicationRequest,
    ReplicationStatus,
    MetricsAck,
    ReplicationAck,
    HealthCheckResponse,
    NodeRegistrationResponse
};

pub struct OrchestratorClient {
    client: NodeOrchestrationClient<Channel>,
}

impl OrchestratorClient {
    pub async fn connect(addr: String) -> Result<Self, tonic::transport::Error> {
        let channel = Endpoint::from_shared(addr)?
            .connect()
            .await?;
        let client = NodeOrchestrationClient::new(channel);
        Ok(Self { client })
    }
    
    pub async fn register_node(
        &mut self, 
        request: NodeRegistrationRequest
    ) -> Result<NodeRegistrationResponse, tonic::Status> {
        let response = self.client.register_node(request).await?;
        Ok(response.into_inner())
    }
    
    pub async fn health_check(
        &mut self, 
        request: HealthCheckRequest
    ) -> Result<HealthCheckResponse, tonic::Status> {
        let response = self.client.health_check(request).await?;
        Ok(response.into_inner())
    }
    
    pub async fn stream_metrics(
        &mut self, 
        request: tonic::Streaming<MetricsUpdate>
    ) -> Result<MetricsAck, tonic::Status> {
        let response = self.client.stream_metrics(request).await?;
        Ok(response.into_inner())
    }
    
    pub async fn replicate_content(
        &mut self, 
        request: ReplicationRequest
    ) -> Result<ReplicationStatus, tonic::Status> {
        let response = self.client.replicate_content(request).await?;
        Ok(response.into_inner())
    }
    
    pub async fn report_replication(
        &mut self, 
        request: ReplicationStatus
    ) -> Result<ReplicationAck, tonic::Status> {
        let response = self.client.report_replication(request).await?;
        Ok(response.into_inner())
    }
}