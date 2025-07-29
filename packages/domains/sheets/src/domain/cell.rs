use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Cell value types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CellValue {
    Empty,
    Text(String),
    Number(f64),
    Boolean(bool),
    Error(String),
    DateTime(DateTime<Utc>),
}

/// Cell address in a spreadsheet (row, column)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CellAddress {
    pub row: u32,
    pub column: u32,
}

impl CellAddress {
    pub fn new(row: u32, column: u32) -> Self {
        Self { row, column }
    }
}

/// Cell style information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CellStyle {
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub font_size: Option<u32>,
    pub font_weight: Option<String>,
    pub text_align: Option<String>,
}

impl Default for CellStyle {
    fn default() -> Self {
        Self {
            background_color: None,
            text_color: None,
            font_size: None,
            font_weight: None,
            text_align: None,
        }
    }
}

/// Cell entity representing a single cell in a spreadsheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub address: CellAddress,
    pub value: CellValue,
    pub formatted_value: String,
    pub style: CellStyle,
    pub dependencies: Vec<CellAddress>, // For formula recalculation
}

impl Cell {
    pub fn new(address: CellAddress, value: CellValue) -> Self {
        let formatted_value = match &value {
            CellValue::Empty => "".to_string(),
            CellValue::Text(s) => s.clone(),
            CellValue::Number(n) => format!("{:.2}", n),
            CellValue::Boolean(b) => b.to_string(),
            CellValue::Error(e) => format!("#{}", e),
            CellValue::DateTime(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        
        Self {
            address,
            value,
            formatted_value,
            style: CellStyle::default(),
            dependencies: Vec::new(),
        }
    }
    
    pub fn update_value(&mut self, value: CellValue) {
        self.value = value;
        self.formatted_value = match &self.value {
            CellValue::Empty => "".to_string(),
            CellValue::Text(s) => s.clone(),
            CellValue::Number(n) => format!("{:.2}", n),
            CellValue::Boolean(b) => b.to_string(),
            CellValue::Error(e) => format!("#{}", e),
            CellValue::DateTime(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    }
    
    pub fn set_style(&mut self, style: CellStyle) {
        self.style = style;
    }
    
    pub fn add_dependency(&mut self, dependency: CellAddress) {
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }
}

/// CRDT-based cell for collaborative editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellCrdt {
    pub address: CellAddress,
    pub values: HashMap<Clock, CellValue>,
    pub metadata: HashMap<Clock, CellMetadata>,
}

/// Logical clock for CRDT operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Clock {
    pub peer_id: Uuid,
    pub timestamp: u64,
    pub counter: u32,
}

/// Metadata for CRDT operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellMetadata {
    pub timestamp: DateTime<Utc>,
    pub user_id: Uuid,
}

impl CellCrdt {
    pub fn new(address: CellAddress) -> Self {
        Self {
            address,
            values: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn merge(&mut self, other: CellCrdt) {
        // CRDT merge implementation
        for (clock, value) in other.values {
            if !self.values.contains_key(&clock) || self.metadata[&clock].timestamp < other.metadata[&clock].timestamp {
                self.values.insert(clock, value);
                self.metadata.insert(clock, other.metadata[&clock].clone());
            }
        }
    }
    
    pub fn get_current_value(&self) -> Option<&CellValue> {
        // Return most recent value based on logical clock
        self.values.values().next()
    }
    
    pub fn update_value(&mut self, clock: Clock, value: CellValue, metadata: CellMetadata) {
        self.values.insert(clock, value);
        self.metadata.insert(clock, metadata);
    }
}