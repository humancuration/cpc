use tonic::{Request, Response, Status};
use pdf_rs::writer::{PdfWriter, Page, PageSize, Point};
use pdf_rs::writer::font::Font;
use pdf_rs::writer::rgb::RGB;
use std::io::Cursor;
use cpc_protos::metrics::*;
use crate::models::AggregatedMetrics;
use tracing::info;

#[derive(Debug, Default)]
pub struct MetricsService {}

#[tonic::path = "cpc.metrics.MetricsService"]
impl MetricsService {
    pub fn new() -> Self {
        MetricsService {}
    }
}

use cpc_protos::metrics::{TestResult, metrics_service_server};
use google_protobuf::Empty;

#[tonic::async_trait]
impl metrics_service_server::MetricsService for MetricsService {
    async fn log_test_result(
        &self,
        request: Request<TestResult>,
    ) -> Result<Response<Empty>, Status> {
        let test_result = request.into_inner();
        info!(
            "Test result: {} - {} - {}",
            test_result.test_name,
            if test_result.passed { "PASSED" } else { "FAILED" },
            test_result.message
        );
        Ok(Response::new(Empty {}))
    }

    async fn get_aggregated_metrics(
        &self,
        request: Request<MetricsRequest>,
    ) -> Result<Response<AggregatedMetrics>, Status> {
        let req = request.into_inner();
        info!("Received metrics request for time range: {:?}, roles: {:?}",
              req.time_range, req.member_roles);

        // TODO: Implement actual metrics aggregation
        let metrics = AggregatedMetrics {
            total_members: 150,
            active_members: 120,
            total_products: 85,
            total_sales: 125000.0,
            total_profit: 25000.0,
            total_carbon_saved: 4500.5,
            avg_profit_per_member: 208.33,
            member_engagement: 0.85,
        };

        Ok(Response::new(metrics))
    }

    async fn export_metrics_to_pdf(
        &self,
        request: Request<MetricsRequest>,
    ) -> Result<Response<PdfResponse>, Status> {
        let metrics = self.get_aggregated_metrics(request).await?.into_inner();
        let mut writer = PdfWriter::new();
        let mut doc = writer.new_doc(PageSize::A4);

        // Create title
        let title = "Cooperative Performance Report";
        let title_font = Font::new(&doc, "Helvetica").unwrap();
        doc.page(|page| {
            page.set_font(&title_font, 24.0);
            page.set_fill_color(RGB::new(0, 0, 0));
            page.text(Point::new(50.0, 700.0), title);
            Ok(())
        })?;

        // Create metrics table
        let mut y_pos = 650.0;
        let row_height = 20.0;
        let metrics_data = [
            ("Total Members", metrics.total_members.to_string()),
            ("Active Members", metrics.active_members.to_string()),
            ("Total Products", metrics.total_products.to_string()),
            ("Total Sales", format!("${:.2}", metrics.total_sales)),
            ("Total Profit", format!("${:.2}", metrics.total_profit)),
            ("Carbon Saved", format!("{:.1} kg", metrics.total_carbon_saved)),
            ("Avg Profit/Member", format!("${:.2}", metrics.avg_profit_per_member)),
            ("Member Engagement", format!("{:.1}%", metrics.member_engagement * 100.0)),
        ];

        for (label, value) in metrics_data.iter() {
            doc.page(|page| {
                page.set_font(&title_font, 12.0);
                page.text(Point::new(50.0, y_pos), label);
                page.text(Point::new(200.0, y_pos), value);
                y_pos -= row_height;
                Ok(())
            })?;
        }

        // Finalize PDF
        let mut buf = Cursor::new(Vec::new());
        writer.write(&mut buf).map_err(|e| Status::internal(e.to_string()))?;
        let pdf_data = buf.into_inner();

        Ok(Response::new(PdfResponse {
            pdf_data,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use cpc_protos::metrics::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_get_aggregated_metrics_day_range() {
        let service = MetricsService::new();
        let request = Request::new(MetricsRequest {
            time_range: Some("day".to_string()),
            member_roles: vec![],
        });
        
        let response = service.get_aggregated_metrics(request).await.unwrap();
        let metrics = response.into_inner();
        
        assert_eq!(metrics.total_members, 150);
        assert_eq!(metrics.active_members, 120);
    }

    #[tokio::test]
    async fn test_get_aggregated_metrics_week_range() {
        let service = MetricsService::new();
        let request = Request::new(MetricsRequest {
            time_range: Some("week".to_string()),
            member_roles: vec![],
        });
        
        let response = service.get_aggregated_metrics(request).await.unwrap();
        let metrics = response.into_inner();
        
        assert_eq!(metrics.total_products, 85);
        assert_eq!(metrics.total_sales, 125000.0);
    }

    #[tokio::test]
    async fn test_get_aggregated_metrics_month_range() {
        let service = MetricsService::new();
        let request = Request::new(MetricsRequest {
            time_range: Some("month".to_string()),
            member_roles: vec![],
        });
        
        let response = service.get_aggregated_metrics(request).await.unwrap();
        let metrics = response.into_inner();
        
        assert_eq!(metrics.total_profit, 25000.0);
        assert_eq!(metrics.total_carbon_saved, 4500.5);
    }

    #[tokio::test]
    async fn test_get_aggregated_metrics_single_role() {
        let service = MetricsService::new();
        let request = Request::new(MetricsRequest {
            time_range: Some("day".to_string()),
            member_roles: vec!["admin".to_string()],
        });
        
        let response = service.get_aggregated_metrics(request).await.unwrap();
        let metrics = response.into_inner();
        
        assert_eq!(metrics.avg_profit_per_member, 208.33);
        assert_eq!(metrics.member_engagement, 0.85);
    }

    #[tokio::test]
    async fn test_get_aggregated_metrics_multiple_roles() {
        let service = MetricsService::new();
        let request = Request::new(MetricsRequest {
            time_range: Some("day".to_string()),
            member_roles: vec!["admin".to_string(), "member".to_string()],
        });
        
        let response = service.get_aggregated_metrics(request).await.unwrap();
        let metrics = response.into_inner();
        
        // Verify metrics are consistent regardless of role combination
        assert_eq!(metrics.total_members, 150);
    }

    #[tokio::test]
    async fn test_get_aggregated_metrics_no_roles() {
        let service = MetricsService::new();
        let request = Request::new(MetricsRequest {
            time_range: Some("day".to_string()),
            member_roles: vec![],
        });
        
        let response = service.get_aggregated_metrics(request).await.unwrap();
        let metrics = response.into_inner();
        
        // Should return metrics for all roles
        assert_eq!(metrics.active_members, 120);
    }

    #[tokio::test]
    async fn test_export_metrics_to_pdf() {
        let service = MetricsService::new();
        let request = Request::new(MetricsRequest {
            time_range: Some("day".to_string()),
            member_roles: vec![],
        });
        
        let response = service.export_metrics_to_pdf(request).await.unwrap();
        let pdf_response = response.into_inner();
        
        // Verify PDF is generated and has reasonable size
        assert!(!pdf_response.pdf_data.is_empty());
        assert!(pdf_response.pdf_data.len() > 500);
        
        // Verify PDF header
        let header = String::from_utf8_lossy(&pdf_response.pdf_data[0..8]);
        assert_eq!(header, "%PDF-1.");
    }

    #[tokio::test]
    async fn test_export_metrics_to_pdf_large_data() {
        let service = MetricsService::new();
        let request = Request::new(MetricsRequest {
            time_range: Some("month".to_string()),
            member_roles: vec!["admin".to_string(), "member".to_string()],
        });
        
        let response = service.export_metrics_to_pdf(request).await.unwrap();
        let pdf_response = response.into_inner();
        
        // Verify PDF can handle different data volumes
        assert!(!pdf_response.pdf_data.is_empty());
    }

    #[tokio::test]
    async fn test_invalid_date_format() {
        let service = MetricsService::new();
        let request = Request::new(MetricsRequest {
            time_range: Some("invalid-date".to_string()),
            member_roles: vec![],
        });
        
        let response = service.get_aggregated_metrics(request).await;
        assert!(response.is_ok(), "Should handle invalid date formats gracefully");
    }

    #[tokio::test]
    async fn test_nonexistent_role() {
        let service = MetricsService::new();
        let request = Request::new(MetricsRequest {
            time_range: Some("day".to_string()),
            member_roles: vec!["nonexistent-role".to_string()],
        });
        
        let response = service.get_aggregated_metrics(request).await;
        assert!(response.is_ok(), "Should handle nonexistent roles gracefully");
    }
}