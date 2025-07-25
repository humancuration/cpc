use cpc_core::expenses::service::ExpenseService;
use cpc_protos::expenses::expense_processing_server::{ExpenseProcessing, ExpenseProcessingServer};
use cpc_protos::expenses::{ReceiptJobRequest, ReceiptJobResult};
use std::pin::Pin;
use std::sync::Arc;
use tokio_stream::Stream;
use tonic::{Request, Response, Status, Streaming};

pub struct ExpenseProcessingService {
    expense_service: Arc<dyn ExpenseService>,
}

impl ExpenseProcessingService {
    pub fn new(expense_service: Arc<dyn ExpenseService>) -> Self {
        Self { expense_service }
    }
}

#[tonic::async_trait]
impl ExpenseProcessing for ExpenseProcessingService {
    type ProcessReceiptsStream = Pin<Box<dyn Stream<Item = Result<ReceiptJobResult, Status>> + Send>>;

    async fn process_receipts(
        &self,
        request: Request<Streaming<ReceiptJobRequest>>,
    ) -> Result<Response<Self::ProcessReceiptsStream>, Status> {
        // For now, we'll just log the request and return an empty stream.
        // In a real implementation, this would handle the stream of requests
        // from worker nodes, process them (e.g., using OCR), and send back results.
        println!("Received receipt processing request stream: {:?}", request);

        let output = async_stream::stream! {
            // empty stream
        };

        Ok(Response::new(Box::pin(output) as Self::ProcessReceiptsStream))
    }
}