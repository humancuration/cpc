//! Role-based permissions for CPC features
use strum_macros::{Display, EnumIter};

/// Permission enum defining access levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, Display)]
pub enum Permission {
    /// Access to accounting features
    ManageAccounting,
    /// Access to product management
    ManageProducts,
    /// Access to service management
    ManageServices,
    /// Access to inventory management
    ManageInventory,
    /// Access to financial forecasting features
    ManageFinancialForecasting,
}

impl Permission {
    /// Get all permissions as a vector
    pub fn all() -> Vec<Self> {
        use strum::IntoEnumIterator;
        Self::iter().collect()
    }
}