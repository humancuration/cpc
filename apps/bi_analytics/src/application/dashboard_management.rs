//! Dashboard management service for the BI & Analytics module

use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;
use crate::domain::{
    dashboard::{Dashboard, DashboardReport, GridPosition},
    report::Report,
};
use thiserror::Error;

/// Error types for dashboard management operations
#[derive(Error, Debug)]
pub enum DashboardManagementError {
    #[error("Dashboard error: {0}")]
    DashboardError(String),
    
    #[error("Report error: {0}")]
    ReportError(String),
    
    #[error("Layout error: {0}")]
    LayoutError(String),
}

/// Dashboard management service
pub struct DashboardManagementService<R: DashboardRepository, Rep: ReportRepository> {
    dashboard_repository: R,
    report_repository: Rep,
}

impl<R: DashboardRepository, Rep: ReportRepository> DashboardManagementService<R, Rep> {
    /// Create a new dashboard management service
    pub fn new(dashboard_repository: R, report_repository: Rep) -> Self {
        Self {
            dashboard_repository,
            report_repository,
        }
    }
    
    /// Create a new dashboard
    pub async fn create_dashboard(
        &self,
        name: String,
        owner_id: Uuid,
        description: Option<String>,
        layout: HashMap<String, serde_json::Value>,
    ) -> Result<Dashboard, DashboardManagementError> {
        let dashboard = Dashboard::new(name, owner_id, description, layout)
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))?;
        
        self.dashboard_repository.save_dashboard(&dashboard)
            .await
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))?;
        
        Ok(dashboard)
    }
    
    /// Get dashboards by owner
    pub async fn get_dashboards_by_owner(
        &self,
        owner_id: Uuid,
    ) -> Result<Vec<Dashboard>, DashboardManagementError> {
        self.dashboard_repository.get_dashboards_by_owner(owner_id)
            .await
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))
    }
    
    /// Get a dashboard by ID
    pub async fn get_dashboard(
        &self,
        dashboard_id: Uuid,
    ) -> Result<Dashboard, DashboardManagementError> {
        self.dashboard_repository.get_dashboard(dashboard_id)
            .await
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))
    }
    
    /// Update dashboard information
    pub async fn update_dashboard(
        &self,
        dashboard_id: Uuid,
        name: Option<String>,
        description: Option<String>,
        layout: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<Dashboard, DashboardManagementError> {
        let mut dashboard = self.dashboard_repository.get_dashboard(dashboard_id)
            .await
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))?;
        
        dashboard.update_info(name, description, layout)
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))?;
        
        self.dashboard_repository.save_dashboard(&dashboard)
            .await
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))?;
        
        Ok(dashboard)
    }
    
    /// Add a report to a dashboard
    pub async fn add_report_to_dashboard(
        &self,
        dashboard_id: Uuid,
        report_id: Uuid,
        position: GridPosition,
    ) -> Result<DashboardReport, DashboardManagementError> {
        // Verify the dashboard exists
        let _dashboard = self.dashboard_repository.get_dashboard(dashboard_id)
            .await
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))?;
        
        // Verify the report exists
        let _report = self.report_repository.get_report(report_id)
            .await
            .map_err(|e| DashboardManagementError::ReportError(e.to_string()))?;
        
        // Create dashboard report association
        let dashboard_report = DashboardReport {
            id: Uuid::new_v4(),
            dashboard_id,
            report_id,
            position,
        };
        
        self.dashboard_repository.save_dashboard_report(&dashboard_report)
            .await
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))?;
        
        Ok(dashboard_report)
    }
    
    /// Get reports for a dashboard
    pub async fn get_dashboard_reports(
        &self,
        dashboard_id: Uuid,
    ) -> Result<Vec<(DashboardReport, Report)>, DashboardManagementError> {
        // Get dashboard report associations
        let dashboard_reports = self.dashboard_repository.get_dashboard_reports(dashboard_id)
            .await
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))?;
        
        // Get the actual reports
        let mut reports_with_positions = Vec::new();
        
        for dashboard_report in dashboard_reports {
            let report = self.report_repository.get_report(dashboard_report.report_id)
                .await
                .map_err(|e| DashboardManagementError::ReportError(e.to_string()))?;
            
            reports_with_positions.push((dashboard_report, report));
        }
        
        Ok(reports_with_positions)
    }
    
    /// Remove a report from a dashboard
    pub async fn remove_report_from_dashboard(
        &self,
        dashboard_report_id: Uuid,
    ) -> Result<(), DashboardManagementError> {
        self.dashboard_repository.delete_dashboard_report(dashboard_report_id)
            .await
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))
    }
    
    /// Update the position of a report on a dashboard
    pub async fn update_report_position(
        &self,
        dashboard_report_id: Uuid,
        position: GridPosition,
    ) -> Result<DashboardReport, DashboardManagementError> {
        let mut dashboard_report = self.dashboard_repository.get_dashboard_report(dashboard_report_id)
            .await
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))?;
        
        dashboard_report.position = position;
        
        self.dashboard_repository.save_dashboard_report(&dashboard_report)
            .await
            .map_err(|e| DashboardManagementError::DashboardError(e.to_string()))?;
        
        Ok(dashboard_report)
    }
}

/// Repository trait for dashboard storage
#[async_trait]
pub trait DashboardRepository: Send + Sync {
    /// Save a dashboard
    async fn save_dashboard(&self, dashboard: &Dashboard) -> Result<(), DashboardManagementError>;
    
    /// Get a dashboard by ID
    async fn get_dashboard(&self, id: Uuid) -> Result<Dashboard, DashboardManagementError>;
    
    /// Get dashboards by owner
    async fn get_dashboards_by_owner(&self, owner_id: Uuid) -> Result<Vec<Dashboard>, DashboardManagementError>;
    
    /// Save a dashboard report association
    async fn save_dashboard_report(&self, dashboard_report: &DashboardReport) -> Result<(), DashboardManagementError>;
    
    /// Get a dashboard report association by ID
    async fn get_dashboard_report(&self, id: Uuid) -> Result<DashboardReport, DashboardManagementError>;
    
    /// Get dashboard report associations by dashboard ID
    async fn get_dashboard_reports(&self, dashboard_id: Uuid) -> Result<Vec<DashboardReport>, DashboardManagementError>;
    
    /// Delete a dashboard report association
    async fn delete_dashboard_report(&self, id: Uuid) -> Result<(), DashboardManagementError>;
}

/// Repository trait for report access
#[async_trait]
pub trait ReportRepository: Send + Sync {
    /// Get a report by ID
    async fn get_report(&self, id: Uuid) -> Result<Report, DashboardManagementError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::dashboard::GridPosition;
    use std::collections::HashMap;
    
    // Mock dashboard repository for testing
    struct MockDashboardRepository;
    
    #[async_trait]
    impl DashboardRepository for MockDashboardRepository {
        async fn save_dashboard(&self, _dashboard: &Dashboard) -> Result<(), DashboardManagementError> {
            Ok(())
        }
        
        async fn get_dashboard(&self, _id: Uuid) -> Result<Dashboard, DashboardManagementError> {
            let layout = HashMap::new();
            
            Ok(Dashboard::new(
                "Test Dashboard".to_string(),
                Uuid::new_v4(),
                Some("Test dashboard".to_string()),
                layout,
            ).unwrap())
        }
        
        async fn get_dashboards_by_owner(&self, _owner_id: Uuid) -> Result<Vec<Dashboard>, DashboardManagementError> {
            let layout = HashMap::new();
            
            let dashboard = Dashboard::new(
                "Test Dashboard".to_string(),
                Uuid::new_v4(),
                Some("Test dashboard".to_string()),
                layout,
            ).unwrap();
            
            Ok(vec![dashboard])
        }
        
        async fn save_dashboard_report(&self, _dashboard_report: &DashboardReport) -> Result<(), DashboardManagementError> {
            Ok(())
        }
        
        async fn get_dashboard_report(&self, _id: Uuid) -> Result<DashboardReport, DashboardManagementError> {
            Ok(DashboardReport {
                id: Uuid::new_v4(),
                dashboard_id: Uuid::new_v4(),
                report_id: Uuid::new_v4(),
                position: GridPosition {
                    x: 0,
                    y: 0,
                    width: 6,
                    height: 4,
                },
            })
        }
        
        async fn get_dashboard_reports(&self, _dashboard_id: Uuid) -> Result<Vec<DashboardReport>, DashboardManagementError> {
            Ok(vec![DashboardReport {
                id: Uuid::new_v4(),
                dashboard_id: Uuid::new_v4(),
                report_id: Uuid::new_v4(),
                position: GridPosition {
                    x: 0,
                    y: 0,
                    width: 6,
                    height: 4,
                },
            }])
        }
        
        async fn delete_dashboard_report(&self, _id: Uuid) -> Result<(), DashboardManagementError> {
            Ok(())
        }
    }
    
    // Mock report repository for testing
    struct MockReportRepository;
    
    #[async_trait]
    impl ReportRepository for MockReportRepository {
        async fn get_report(&self, _id: Uuid) -> Result<Report, DashboardManagementError> {
            Ok(Report::new(
                Uuid::new_v4(),
                "Test Report".to_string(),
                "SELECT *".to_string(),
                crate::domain::report::VisualizationType::BarChart,
                Uuid::new_v4(),
                Some("Test report".to_string()),
            ).unwrap())
        }
    }
    
    #[tokio::test]
    async fn test_create_dashboard() {
        let dashboard_repository = MockDashboardRepository;
        let report_repository = MockReportRepository;
        let service = DashboardManagementService::new(dashboard_repository, report_repository);
        
        let layout = HashMap::new();
        let dashboard = service.create_dashboard(
            "Test Dashboard".to_string(),
            Uuid::new_v4(),
            Some("Test dashboard".to_string()),
            layout,
        ).await.unwrap();
        
        assert_eq!(dashboard.name, "Test Dashboard");
    }
    
    #[tokio::test]
    async fn test_get_dashboards_by_owner() {
        let dashboard_repository = MockDashboardRepository;
        let report_repository = MockReportRepository;
        let service = DashboardManagementService::new(dashboard_repository, report_repository);
        
        let dashboards = service.get_dashboards_by_owner(Uuid::new_v4()).await.unwrap();
        
        assert_eq!(dashboards.len(), 1);
        assert_eq!(dashboards[0].name, "Test Dashboard");
    }
    
    #[tokio::test]
    async fn test_add_report_to_dashboard() {
        let dashboard_repository = MockDashboardRepository;
        let report_repository = MockReportRepository;
        let service = DashboardManagementService::new(dashboard_repository, report_repository);
        
        let position = GridPosition {
            x: 0,
            y: 0,
            width: 6,
            height: 4,
        };
        
        let dashboard_report = service.add_report_to_dashboard(
            Uuid::new_v4(),
            Uuid::new_v4(),
            position,
        ).await.unwrap();
        
        assert_eq!(dashboard_report.position.x, 0);
        assert_eq!(dashboard_report.position.y, 0);
        assert_eq!(dashboard_report.position.width, 6);
        assert_eq!(dashboard_report.position.height, 4);
    }
    
    #[tokio::test]
    async fn test_get_dashboard_reports() {
        let dashboard_repository = MockDashboardRepository;
        let report_repository = MockReportRepository;
        let service = DashboardManagementService::new(dashboard_repository, report_repository);
        
        let reports = service.get_dashboard_reports(Uuid::new_v4()).await.unwrap();
        
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].1.name, "Test Report");
    }
}