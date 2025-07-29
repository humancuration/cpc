use crate::domain::{Sheet, Cell, CellAddress, Formula, ChartSpec};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

/// Repository trait for sheet persistence
pub trait SheetRepository {
    fn save(&self, sheet: &Sheet) -> Result<(), Box<dyn std::error::Error>>;
    fn find_by_id(&self, id: &Uuid) -> Result<Option<Sheet>, Box<dyn std::error::Error>>;
    fn find_by_owner(&self, owner_id: &Uuid) -> Result<Vec<Sheet>, Box<dyn std::error::Error>>;
    fn delete(&self, id: &Uuid) -> Result<(), Box<dyn std::error::Error>>;
}

/// Service for managing sheets
pub struct SheetService {
    repository: Arc<dyn SheetRepository>,
}

impl SheetService {
    pub fn new(repository: Arc<dyn SheetRepository>) -> Self {
        Self { repository }
    }
    
    /// Create a new sheet
    pub fn create_sheet(&self, name: String, owner: Uuid) -> Result<Sheet, Box<dyn std::error::Error>> {
        let sheet = Sheet::new(name, owner);
        self.repository.save(&sheet)?;
        Ok(sheet)
    }
    
    /// Get a sheet by ID
    pub fn get_sheet(&self, id: &Uuid) -> Result<Option<Sheet>, Box<dyn std::error::Error>> {
        self.repository.find_by_id(id)
    }
    
    /// Get all sheets owned by a user
    pub fn get_sheets_by_owner(&self, owner_id: &Uuid) -> Result<Vec<Sheet>, Box<dyn std::error::Error>> {
        self.repository.find_by_owner(owner_id)
    }
    
    /// Update a cell in a sheet
    pub fn update_cell(&self, sheet_id: &Uuid, address: CellAddress, cell: Cell) -> Result<(), Box<dyn std::error::Error>> {
        let mut sheet = self.repository.find_by_id(sheet_id)?
            .ok_or("Sheet not found")?;
        sheet.update_cell(address, cell);
        self.repository.save(&sheet)?;
        Ok(())
    }
    
    /// Update a formula in a sheet
    pub fn update_formula(&self, sheet_id: &Uuid, address: CellAddress, formula: Formula) -> Result<(), Box<dyn std::error::Error>> {
        let mut sheet = self.repository.find_by_id(sheet_id)?
            .ok_or("Sheet not found")?;
        sheet.update_formula(address, formula);
        self.repository.save(&sheet)?;
        Ok(())
    }
    
    /// Add a chart to a sheet
    pub fn add_chart(&self, sheet_id: &Uuid, chart: ChartSpec) -> Result<(), Box<dyn std::error::Error>> {
        let mut sheet = self.repository.find_by_id(sheet_id)?
            .ok_or("Sheet not found")?;
        sheet.add_chart(chart);
        self.repository.save(&sheet)?;
        Ok(())
    }
    
    /// Remove a chart from a sheet
    pub fn remove_chart(&self, sheet_id: &Uuid, chart_id: &Uuid) -> Result<bool, Box<dyn std::error::Error>> {
        let mut sheet = self.repository.find_by_id(sheet_id)?
            .ok_or("Sheet not found")?;
        let removed = sheet.remove_chart(chart_id);
        if removed {
            self.repository.save(&sheet)?;
        }
        Ok(removed)
    }
    
    /// Set permission for a user on a sheet
    pub fn set_permission(&self, sheet_id: &Uuid, user_id: Uuid, level: crate::domain::PermissionLevel) -> Result<(), Box<dyn std::error::Error>> {
        let mut sheet = self.repository.find_by_id(sheet_id)?
            .ok_or("Sheet not found")?;
        sheet.set_permission(user_id, level);
        self.repository.save(&sheet)?;
        Ok(())
    }
    
    /// Delete a sheet
    pub fn delete_sheet(&self, id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.delete(id)
    }
}