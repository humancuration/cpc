use tonic::{Request, Response, Status, Streaming};
use cpc_core::services::impact::ImpactCalculator;
use cpc_protos::impact::v1::impact_service_server::ImpactService;
use cpc_protos::impact::v1::{
    ComputeImpactReportRequest, ComputeImpactReportResponse, ProgressUpdate, ImpactReport
};
use tonic::codegen::tokio_stream::{self, StreamExt};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;

use ed25519_dalek::{PublicKey, Verifier};
use base64;

pub struct ImpactHandler {
    impact_calculator: Arc<ImpactCalculator>,
    verification_key: PublicKey,
}

impl ImpactHandler {
    pub fn new(impact_calculator: Arc<ImpactCalculator>, verification_key: PublicKey) -> Self {
        Self { impact_calculator, verification_key }
    }
}

#[tonic::async_trait]
impl ImpactService for ImpactHandler {
    type ComputeImpactReportStream = tokio_stream::Once<Result<ComputeImpactReportResponse, Status>>;

    async fn compute_impact_report(
        &self,
        request: Request<ComputeImpactReportRequest>
    ) -> Result<Response<Self::ComputeImpactReportStream>, Status> {
        let req = request.into_inner();
        
        // Parse user ID
        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|_| Status::invalid_argument("Invalid user ID format"))?;
            
        // Convert protobuf timestamps to DateTime<Utc>
        let start_date = req.start_date.ok_or(Status::invalid_argument("Missing start_date"))?
            .try_into()
            .map_err(|_| Status::invalid_argument("Invalid start_date"))?;
            
        let end_date = req.end_date.ok_or(Status::invalid_argument("Missing end_date"))?
            .try_into()
            .map_err(|_| Status::invalid_argument("Invalid end_date"))?;

        // Calculate impact report
        let report = self.impact_calculator.calculate_impact(user_id, start_date, end_date)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Verify the signature
        if !self.verify_report_signature(&report) {
            return Err(Status::invalid_argument("Invalid signature"));
            fn verify_report_signature(&self, report: &cpc_core::models::impact::ImpactReport) -> bool {
                #[derive(serde::Serialize)]
                struct CanonicalReport {
                    id: Uuid,
                    user_id: Uuid,
                    period_start: DateTime<Utc>,
                    period_end: DateTime<Utc>,
                    overall_score: f64,
                    category_distribution: HashMap<ImpactCategory, f64>,
                }
                
                let canonical = CanonicalReport {
                    id: report.id,
                    user_id: report.user_id,
                    period_start: report.period_start,
                    period_end: report.period_end,
                    overall_score: report.overall_score,
                    category_distribution: report.category_distribution.clone(),
                };
                
                if let Ok(serialized) = serde_json::to_vec(&canonical) {
                    if let Ok(signature_bytes) = base64::decode(&report.signature) {
                        if let Ok(signature) = ed25519_dalek::Signature::from_bytes(&signature_bytes) {
                            return self.verification_key.verify(&serialized, &signature).is_ok();
                        }
                    }
                }
                false
            }
        }

        // Convert to protobuf response
        let pb_report = ImpactReport {
            id: report.id.to_string(),
            user_id: report.user_id.to_string(),
            period_start: Some(report.period_start.into()),
            period_end: Some(report.period_end.into()),
            overall_score: report.overall_score,
            category_distribution: report.category_distribution.iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
            signature: report.signature,
        };

        let response = ComputeImpactReportResponse {
            result: Some(cpc_protos::impact::v1::compute_impact_report_response::Result::Report(pb_report)),
        };

        // Create response stream (simple one-item stream for now)
        let stream = tokio_stream::once(Ok(response));
        Ok(Response::new(Box::pin(stream) as Self::ComputeImpactReportStream))
    }
}