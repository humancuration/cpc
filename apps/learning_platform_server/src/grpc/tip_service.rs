use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::Utc;
use crate::database::repository::DatabaseRepository;
use crate::database::models::Tip as DatabaseTip;

// Import the generated protobuf types
tonic::include_proto!("cpc.learning_platform");

pub struct TipService {
    repository: DatabaseRepository,
}

impl TipService {
    pub fn new(repository: DatabaseRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl tip_service_server::TipService for TipService {
    async fn send_tip(
        &self,
        request: Request<SendTipRequest>,
    ) -> Result<Response<SendTipResponse>, Status> {
        let req = request.into_inner();
        
        let sender_id = Uuid::parse_str(&req.sender_id)
            .map_err(|_| Status::invalid_argument("Invalid sender ID"))?;
            
        let recipient_id = Uuid::parse_str(&req.recipient_id)
            .map_err(|_| Status::invalid_argument("Invalid recipient ID"))?;
        
        // Validate amount
        if req.amount <= 0.0 {
            return Err(Status::invalid_argument("Tip amount must be positive"));
        }
        
        // Validate course ID if provided
        let course_id = if let Some(ref course_id_str) = req.course_id {
            Some(Uuid::parse_str(course_id_str)
                .map_err(|_| Status::invalid_argument("Invalid course ID"))?)
        } else {
            None
        };
        
        // Check if course exists if course_id is provided
        if let Some(course_id) = course_id {
            if self.repository.get_course_by_id(course_id).await
                .map_err(|e| Status::internal(format!("Failed to check course: {}", e)))?
                .is_none() {
                return Err(Status::not_found("Course not found"));
            }
        }
        
        // Create tip
        let tip_id = Uuid::new_v4();
        let db_tip = DatabaseTip {
            id: tip_id,
            from_user_id: sender_id,
            to_user_id: recipient_id,
            course_id,
            amount: req.amount,
            currency: req.currency.clone(),
            created_at: Utc::now(),
        };
        
        // Save to database
        let saved_tip = self.repository.create_tip(&db_tip).await
            .map_err(|e| Status::internal(format!("Failed to create tip: {}", e)))?;
        
        // Convert to protobuf tip
        let proto_tip = Tip {
            id: saved_tip.id.to_string(),
            sender_id: saved_tip.from_user_id.to_string(),
            recipient_id: saved_tip.to_user_id.to_string(),
            amount: saved_tip.amount,
            currency: saved_tip.currency,
            course_id: saved_tip.course_id.map(|id| id.to_string()),
        };
        
        let response = SendTipResponse {
            tip: Some(proto_tip),
        };
        
        Ok(Response::new(response))
    }
}