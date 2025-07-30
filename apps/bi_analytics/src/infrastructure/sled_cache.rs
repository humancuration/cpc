//! Sled cache implementation for the BI & Analytics module

use sled::Db;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::{
    domain::{
        dataset::{Dataset, DataPoint},
        report::Report,
        dashboard::Dashboard,
    },
    application::{
        data_ingestion::DataRepository,
        report_generation::{ReportRepository, DataRepository as ReportDataRepository},
        dashboard_management::DashboardRepository,
    },
};
use tracing::{info, debug, warn};

/// Sled cache implementation for BI & Analytics data
pub struct SledCache {
    db: Db,
}

impl SledCache {
    /// Create a new Sled cache
    pub fn new(db: Db) -> Self {
        Self { db }
    }
    
    /// Get a tree from the database
    fn get_tree(&self, name: &str) -> sled::Tree {
        self.db.open_tree(name).expect("Failed to open tree")
    }
}

/// Cached dataset model
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedDataset {
    id: Uuid,
    data: Vec<u8>, // Serialized Dataset
    timestamp: chrono::DateTime<chrono::Utc>,
}

/// Cached data points model
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedDataPoints {
    dataset_id: Uuid,
    data: Vec<u8>, // Serialized Vec<DataPoint>
    timestamp: chrono::DateTime<chrono::Utc>,
}

/// Cached report model
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedReport {
    id: Uuid,
    data: Vec<u8>, // Serialized Report
    timestamp: chrono::DateTime<chrono::Utc>,
}

/// Cached dashboard model
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedDashboard {
    id: Uuid,
    data: Vec<u8>, // Serialized Dashboard
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl SledCache {
    /// Serialize and cache a dataset
    fn cache_dataset(&self, dataset: &Dataset) -> Result<(), Box<dyn std::error::Error>> {
        let tree = self.get_tree("datasets");
        let serialized = bincode::serialize(dataset)?;
        let cached = CachedDataset {
            id: dataset.id,
            data: serialized,
            timestamp: chrono::Utc::now(),
        };
        let cached_bytes = bincode::serialize(&cached)?;
        tree.insert(dataset.id.to_string().as_bytes(), cached_bytes)?;
        Ok(())
    }
    
    /// Get a cached dataset
    fn get_cached_dataset(&self, id: Uuid) -> Result<Option<Dataset>, Box<dyn std::error::Error>> {
        let tree = self.get_tree("datasets");
        if let Some(bytes) = tree.get(id.to_string().as_bytes())? {
            let cached: CachedDataset = bincode::deserialize(&bytes)?;
            let dataset: Dataset = bincode::deserialize(&cached.data)?;
            Ok(Some(dataset))
        } else {
            Ok(None)
        }
    }
    
    /// Serialize and cache data points
    fn cache_data_points(&self, dataset_id: Uuid, data_points: &[DataPoint]) -> Result<(), Box<dyn std::error::Error>> {
        let tree = self.get_tree("data_points");
        let serialized = bincode::serialize(data_points)?;
        let cached = CachedDataPoints {
            dataset_id,
            data: serialized,
            timestamp: chrono::Utc::now(),
        };
        let cached_bytes = bincode::serialize(&cached)?;
        tree.insert(dataset_id.to_string().as_bytes(), cached_bytes)?;
        Ok(())
    }
    
    /// Get cached data points
    fn get_cached_data_points(&self, dataset_id: Uuid) -> Result<Option<Vec<DataPoint>>, Box<dyn std::error::Error>> {
        let tree = self.get_tree("data_points");
        if let Some(bytes) = tree.get(dataset_id.to_string().as_bytes())? {
            let cached: CachedDataPoints = bincode::deserialize(&bytes)?;
            let data_points: Vec<DataPoint> = bincode::deserialize(&cached.data)?;
            Ok(Some(data_points))
        } else {
            Ok(None)
        }
    }
    
    /// Serialize and cache a report
    fn cache_report(&self, report: &Report) -> Result<(), Box<dyn std::error::Error>> {
        let tree = self.get_tree("reports");
        let serialized = bincode::serialize(report)?;
        let cached = CachedReport {
            id: report.id,
            data: serialized,
            timestamp: chrono::Utc::now(),
        };
        let cached_bytes = bincode::serialize(&cached)?;
        tree.insert(report.id.to_string().as_bytes(), cached_bytes)?;
        Ok(())
    }
    
    /// Get a cached report
    fn get_cached_report(&self, id: Uuid) -> Result<Option<Report>, Box<dyn std::error::Error>> {
        let tree = self.get_tree("reports");
        if let Some(bytes) = tree.get(id.to_string().as_bytes())? {
            let cached: CachedReport = bincode::deserialize(&bytes)?;
            let report: Report = bincode::deserialize(&cached.data)?;
            Ok(Some(report))
        } else {
            Ok(None)
        }
    }
    
    /// Serialize and cache a dashboard
    fn cache_dashboard(&self, dashboard: &Dashboard) -> Result<(), Box<dyn std::error::Error>> {
        let tree = self.get_tree("dashboards");
        let serialized = bincode::serialize(dashboard)?;
        let cached = CachedDashboard {
            id: dashboard.id,
            data: serialized,
            timestamp: chrono::Utc::now(),
        };
        let cached_bytes = bincode::serialize(&cached)?;
        tree.insert(dashboard.id.to_string().as_bytes(), cached_bytes)?;
        Ok(())
    }
    
    /// Get a cached dashboard
    fn get_cached_dashboard(&self, id: Uuid) -> Result<Option<Dashboard>, Box<dyn std::error::Error>> {
        let tree = self.get_tree("dashboards");
        if let Some(bytes) = tree.get(id.to_string().as_bytes())? {
            let cached: CachedDashboard = bincode::deserialize(&bytes)?;
            let dashboard: Dashboard = bincode::deserialize(&cached.data)?;
            Ok(Some(dashboard))
        } else {
            Ok(None)
        }
    }
    
    /// Clear expired cache entries
    pub fn clear_expired_cache(&self, max_age_seconds: i64) -> Result<(), Box<dyn std::error::Error>> {
        let now = chrono::Utc::now();
        let cutoff = now - chrono::Duration::seconds(max_age_seconds);
        
        // Clear expired datasets
        let dataset_tree = self.get_tree("datasets");
        for result in dataset_tree.iter() {
            let (key, value) = result?;
            let cached: CachedDataset = bincode::deserialize(&value)?;
            if cached.timestamp < cutoff {
                dataset_tree.remove(key)?;
            }
        }
        
        // Clear expired data points
        let data_points_tree = self.get_tree("data_points");
        for result in data_points_tree.iter() {
            let (key, value) = result?;
            let cached: CachedDataPoints = bincode::deserialize(&value)?;
            if cached.timestamp < cutoff {
                data_points_tree.remove(key)?;
            }
        }
        
        // Clear expired reports
        let reports_tree = self.get_tree("reports");
        for result in reports_tree.iter() {
            let (key, value) = result?;
            let cached: CachedReport = bincode::deserialize(&value)?;
            if cached.timestamp < cutoff {
                reports_tree.remove(key)?;
            }
        }
        
        // Clear expired dashboards
        let dashboards_tree = self.get_tree("dashboards");
        for result in dashboards_tree.iter() {
            let (key, value) = result?;
            let cached: CachedDashboard = bincode::deserialize(&value)?;
            if cached.timestamp < cutoff {
                dashboards_tree.remove(key)?;
            }
        }
        
        Ok(())
    }
}

impl DataRepository for SledCache {
    async fn save_dataset(&self, dataset: &Dataset) -> Result<(), super::super::application::data_ingestion::DataIngestionError> {
        info!("Caching dataset: {}", dataset.id);
        self.cache_dataset(dataset)
            .map_err(|e| super::super::application::data_ingestion::DataIngestionError::StorageError(e.to_string()))
    }
    
    async fn get_dataset(&self, id: Uuid) -> Result<Dataset, super::super::application::data_ingestion::DataIngestionError> {
        debug!("Getting cached dataset: {}", id);
        match self.get_cached_dataset(id)
            .map_err(|e| super::super::application::data_ingestion::DataIngestionError::StorageError(e.to_string()))?
        {
            Some(dataset) => {
                debug!("Found cached dataset: {}", id);
                Ok(dataset)
            }
            None => {
                warn!("Dataset not found in cache: {}", id);
                Err(super::super::application::data_ingestion::DataIngestionError::DatasetError("Dataset not found in cache".to_string()))
            }
        }
    }
    
    async fn store_data_points(&self, dataset_id: Uuid, data_points: Vec<DataPoint>) -> Result<(), super::super::application::data_ingestion::DataIngestionError> {
        info!("Caching {} data points for dataset: {}", data_points.len(), dataset_id);
        self.cache_data_points(dataset_id, &data_points)
            .map_err(|e| super::super::application::data_ingestion::DataIngestionError::StorageError(e.to_string()))
    }
    
    async fn get_data_points(&self, dataset_id: Uuid) -> Result<Vec<DataPoint>, super::super::application::data_ingestion::DataIngestionError> {
        debug!("Getting cached data points for dataset: {}", dataset_id);
        match self.get_cached_data_points(dataset_id)
            .map_err(|e| super::super::application::data_ingestion::DataIngestionError::StorageError(e.to_string()))?
        {
            Some(data_points) => {
                debug!("Found {} cached data points for dataset: {}", data_points.len(), dataset_id);
                Ok(data_points)
            }
            None => {
                warn!("Data points not found in cache for dataset: {}", dataset_id);
                Err(super::super::application::data_ingestion::DataIngestionError::DatasetError("Data points not found in cache".to_string()))
            }
        }
    }
}

impl ReportRepository for SledCache {
    async fn save_report(&self, report: &Report) -> Result<(), super::super::application::report_generation::ReportGenerationError> {
        info!("Caching report: {}", report.id);
        self.cache_report(report)
            .map_err(|e| super::super::application::report_generation::ReportGenerationError::ReportError(e.to_string()))
    }
    
    async fn get_report(&self, id: Uuid) -> Result<Report, super::super::application::report_generation::ReportGenerationError> {
        debug!("Getting cached report: {}", id);
        match self.get_cached_report(id)
            .map_err(|e| super::super::application::report_generation::ReportGenerationError::ReportError(e.to_string()))?
        {
            Some(report) => {
                debug!("Found cached report: {}", id);
                Ok(report)
            }
            None => {
                warn!("Report not found in cache: {}", id);
                Err(super::super::application::report_generation::ReportGenerationError::ReportError("Report not found in cache".to_string()))
            }
        }
    }
    
    async fn get_reports_by_owner(&self, _owner_id: Uuid) -> Result<Vec<Report>, super::super::application::report_generation::ReportGenerationError> {
        // Sled cache doesn't efficiently support querying by owner
        // This would typically be handled by the primary database
        warn!("get_reports_by_owner not implemented for Sled cache");
        Err(super::super::application::report_generation::ReportGenerationError::ReportError("Not implemented in cache".to_string()))
    }
}

impl ReportDataRepository for SledCache {
    async fn get_dataset(&self, id: Uuid) -> Result<crate::domain::dataset::Dataset, super::super::application::report_generation::ReportGenerationError> {
        // Delegate to DataRepository implementation
        let data_repo = self as &dyn DataRepository;
        data_repo.get_dataset(id).await
            .map_err(|e| super::super::application::report_generation::ReportGenerationError::DatasetError(e.to_string()))
    }
    
    async fn get_data_points(&self, dataset_id: Uuid) -> Result<Vec<crate::domain::dataset::DataPoint>, super::super::application::report_generation::ReportGenerationError> {
        // Delegate to DataRepository implementation
        let data_repo = self as &dyn DataRepository;
        data_repo.get_data_points(dataset_id).await
            .map_err(|e| super::super::application::report_generation::ReportGenerationError::DataQueryError(e.to_string()))
    }
}

impl DashboardRepository for SledCache {
    async fn save_dashboard(&self, dashboard: &Dashboard) -> Result<(), super::super::application::dashboard_management::DashboardManagementError> {
        info!("Caching dashboard: {}", dashboard.id);
        self.cache_dashboard(dashboard)
            .map_err(|e| super::super::application::dashboard_management::DashboardManagementError::DashboardError(e.to_string()))
    }
    
    async fn get_dashboard(&self, id: Uuid) -> Result<Dashboard, super::super::application::dashboard_management::DashboardManagementError> {
        debug!("Getting cached dashboard: {}", id);
        match self.get_cached_dashboard(id)
            .map_err(|e| super::super::application::dashboard_management::DashboardManagementError::DashboardError(e.to_string()))?
        {
            Some(dashboard) => {
                debug!("Found cached dashboard: {}", id);
                Ok(dashboard)
            }
            None => {
                warn!("Dashboard not found in cache: {}", id);
                Err(super::super::application::dashboard_management::DashboardManagementError::DashboardError("Dashboard not found in cache".to_string()))
            }
        }
    }
    
    async fn get_dashboards_by_owner(&self, _owner_id: Uuid) -> Result<Vec<Dashboard>, super::super::application::dashboard_management::DashboardManagementError> {
        // Sled cache doesn't efficiently support querying by owner
        // This would typically be handled by the primary database
        warn!("get_dashboards_by_owner not implemented for Sled cache");
        Err(super::super::application::dashboard_management::DashboardManagementError::DashboardError("Not implemented in cache".to_string()))
    }
    
    async fn save_dashboard_report(&self, _dashboard_report: &crate::domain::dashboard::DashboardReport) -> Result<(), super::super::application::dashboard_management::DashboardManagementError> {
        // Dashboard reports are typically not cached separately
        warn!("save_dashboard_report not implemented for Sled cache");
        Err(super::super::application::dashboard_management::DashboardManagementError::DashboardError("Not implemented in cache".to_string()))
    }
    
    async fn get_dashboard_report(&self, _id: Uuid) -> Result<crate::domain::dashboard::DashboardReport, super::super::application::dashboard_management::DashboardManagementError> {
        // Dashboard reports are typically not cached separately
        warn!("get_dashboard_report not implemented for Sled cache");
        Err(super::super::application::dashboard_management::DashboardManagementError::DashboardError("Not implemented in cache".to_string()))
    }
    
    async fn get_dashboard_reports(&self, _dashboard_id: Uuid) -> Result<Vec<crate::domain::dashboard::DashboardReport>, super::super::application::dashboard_management::DashboardManagementError> {
        // Dashboard reports are typically not cached separately
        warn!("get_dashboard_reports not implemented for Sled cache");
        Err(super::super::application::dashboard_management::DashboardManagementError::DashboardError("Not implemented in cache".to_string()))
    }
    
    async fn delete_dashboard_report(&self, _id: Uuid) -> Result<(), super::super::application::dashboard_management::DashboardManagementError> {
        // Dashboard reports are typically not cached separately
        warn!("delete_dashboard_report not implemented for Sled cache");
        Err(super::super::application::dashboard_management::DashboardManagementError::DashboardError("Not implemented in cache".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        dataset::{Dataset, DataSource, FieldDefinition, DataType},
        report::{Report, VisualizationType},
        dashboard::Dashboard,
    };
    use std::collections::HashMap;
    use tempdir::TempDir;
    
    fn create_test_db() -> Db {
        let temp_dir = TempDir::new("sled_test").unwrap();
        sled::open(temp_dir.path()).unwrap()
    }
    
    #[tokio::test]
    async fn test_cache_dataset() {
        let db = create_test_db();
        let cache = SledCache::new(db);
        
        let fields = vec![
            FieldDefinition {
                name: "revenue".to_string(),
                data_type: DataType::Float,
                is_nullable: false,
                description: Some("Monthly revenue".to_string()),
            }
        ];
        
        let dataset = Dataset::new(
            "Test Dataset".to_string(),
            DataSource::Finance,
            fields,
            Uuid::new_v4(),
            Some("Test dataset".to_string()),
        ).unwrap();
        
        // Save to cache
        cache.save_dataset(&dataset).await.unwrap();
        
        // Retrieve from cache
        let retrieved = cache.get_dataset(dataset.id).await.unwrap();
        assert_eq!(retrieved.id, dataset.id);
        assert_eq!(retrieved.name, dataset.name);
    }
    
    #[tokio::test]
    async fn test_cache_report() {
        let db = create_test_db();
        let cache = SledCache::new(db);
        
        let report = Report::new(
            Uuid::new_v4(),
            "Test Report".to_string(),
            "SELECT *".to_string(),
            VisualizationType::BarChart,
            Uuid::new_v4(),
            Some("Test report".to_string()),
        ).unwrap();
        
        // Save to cache
        cache.save_report(&report).await.unwrap();
        
        // Retrieve from cache
        let retrieved = cache.get_report(report.id).await.unwrap();
        assert_eq!(retrieved.id, report.id);
        assert_eq!(retrieved.name, report.name);
    }
    
    #[tokio::test]
    async fn test_cache_dashboard() {
        let db = create_test_db();
        let cache = SledCache::new(db);
        
        let layout = HashMap::new();
        let dashboard = Dashboard::new(
            "Test Dashboard".to_string(),
            Uuid::new_v4(),
            Some("Test dashboard".to_string()),
            layout,
        ).unwrap();
        
        // Save to cache
        cache.save_dashboard(&dashboard).await.unwrap();
        
        // Retrieve from cache
        let retrieved = cache.get_dashboard(dashboard.id).await.unwrap();
        assert_eq!(retrieved.id, dashboard.id);
        assert_eq!(retrieved.name, dashboard.name);
    }
    
    #[test]
    fn test_clear_expired_cache() {
        let db = create_test_db();
        let cache = SledCache::new(db);
        
        // This test would require mocking time or using a more complex setup
        // For now, we'll just ensure it doesn't panic
        cache.clear_expired_cache(3600).unwrap();
    }
}