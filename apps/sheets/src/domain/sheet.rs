use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use super::{Cell, CellAddress, Formula, ChartSpec};

/// Permission level for sheet access
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionLevel {
    Owner,
    Editor,
    Viewer,
    None,
}

/// Sheet entity representing a collaborative spreadsheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    pub id: Uuid,
    pub name: String,
    pub owner: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64,
    #[serde(skip)]
    pub cells: HashMap<CellAddress, Cell>,
    #[serde(skip)]
    pub formulas: HashMap<CellAddress, Formula>,
    pub charts: Vec<ChartSpec>,
    pub permissions: HashMap<Uuid, PermissionLevel>,
}

impl Sheet {
    pub fn new(name: String, owner: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            owner,
            created_at: now,
            updated_at: now,
            version: 1,
            cells: HashMap::new(),
            formulas: HashMap::new(),
            charts: Vec::new(),
            permissions: {
                let mut perms = HashMap::new();
                perms.insert(owner, PermissionLevel::Owner);
                perms
            },
        }
    }
    
    pub fn update_cell(&mut self, address: CellAddress, cell: Cell) {
        self.cells.insert(address, cell);
        self.updated_at = Utc::now();
        self.version += 1;
    }
    
    pub fn get_cell(&self, address: &CellAddress) -> Option<&Cell> {
        self.cells.get(address)
    }
    
    pub fn update_formula(&mut self, address: CellAddress, formula: Formula) {
        self.formulas.insert(address, formula);
        self.updated_at = Utc::now();
        self.version += 1;
    }
    
    pub fn get_formula(&self, address: &CellAddress) -> Option<&Formula> {
        self.formulas.get(address)
    }
    
    pub fn add_chart(&mut self, chart: ChartSpec) {
        self.charts.push(chart);
        self.updated_at = Utc::now();
    }
    
    pub fn remove_chart(&mut self, chart_id: &Uuid) -> bool {
        let initial_len = self.charts.len();
        self.charts.retain(|c| &c.id != chart_id);
        let removed = self.charts.len() < initial_len;
        if removed {
            self.updated_at = Utc::now();
        }
        removed
    }
    
    pub fn set_permission(&mut self, user_id: Uuid, level: PermissionLevel) {
        self.permissions.insert(user_id, level);
        self.updated_at = Utc::now();
    }
    
    pub fn get_permission(&self, user_id: &Uuid) -> Option<&PermissionLevel> {
        self.permissions.get(user_id)
    }
    
    pub fn can_edit(&self, user_id: &Uuid) -> bool {
        match self.permissions.get(user_id) {
            Some(PermissionLevel::Owner) | Some(PermissionLevel::Editor) => true,
            _ => false,
        }
    }
    
    pub fn can_view(&self, user_id: &Uuid) -> bool {
        match self.permissions.get(user_id) {
            Some(PermissionLevel::Owner) | Some(PermissionLevel::Editor) | Some(PermissionLevel::Viewer) => true,
            _ => false,
        }
    }
}