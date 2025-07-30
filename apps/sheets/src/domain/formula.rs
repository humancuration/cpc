use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::{CellAddress, CellValue};

/// Formula function types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FormulaFunction {
    Sum,
    Average,
    Count,
    Min,
    Max,
    If,
    VLookup,
    HLookup,
    Concatenate,
    Upper,
    Lower,
    Len,
    Left,
    Right,
    Mid,
    // Additional functions...
}

/// Formula entity representing a cell formula
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Formula {
    pub expression: String,
    pub dependencies: Vec<CellAddress>,
    pub last_evaluated: DateTime<Utc>,
    pub cache: Option<CellValue>,
}

impl Formula {
    pub fn new(expression: String, dependencies: Vec<CellAddress>) -> Self {
        Self {
            expression,
            dependencies,
            last_evaluated: Utc::now(),
            cache: None,
        }
    }
    
    pub fn update_dependencies(&mut self, dependencies: Vec<CellAddress>) {
        self.dependencies = dependencies;
    }
    
    pub fn update_cache(&mut self, value: CellValue) {
        self.cache = Some(value);
        self.last_evaluated = Utc::now();
    }
    
    pub fn clear_cache(&mut self) {
        self.cache = None;
    }
    
    pub fn needs_evaluation(&self) -> bool {
        self.cache.is_none()
    }
}

/// Cell range in a spreadsheet
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CellRange {
    pub start: CellAddress,
    pub end: CellAddress,
}

impl CellRange {
    pub fn new(start: CellAddress, end: CellAddress) -> Self {
        Self { start, end }
    }
    
    pub fn contains(&self, address: &CellAddress) -> bool {
        address.row >= self.start.row && address.row <= self.end.row &&
        address.column >= self.start.column && address.column <= self.end.column
    }
    
    pub fn iter(&self) -> CellRangeIterator {
        CellRangeIterator {
            range: self.clone(),
            current_row: self.start.row,
            current_col: self.start.column,
        }
    }
}

/// Iterator for cell ranges
pub struct CellRangeIterator {
    range: CellRange,
    current_row: u32,
    current_col: u32,
}

impl Iterator for CellRangeIterator {
    type Item = CellAddress;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row > self.range.end.row {
            return None;
        }
        
        let address = CellAddress::new(self.current_row, self.current_col);
        
        // Move to next cell
        self.current_col += 1;
        if self.current_col > self.range.end.column {
            self.current_col = self.range.start.column;
            self.current_row += 1;
        }
        
        Some(address)
    }
}