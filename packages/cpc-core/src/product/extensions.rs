use crate::models::Product;
use crate::types::SupplyChain;
use crate::errors::ModelError;
use validator::Validate;

pub trait ProductExt {
    fn calculate_tax(&self) -> f64;
    fn validate_supply_chain(&self) -> Result<(), ModelError>;
    fn total_cost(&self) -> Option<f64>;
}

impl ProductExt for Product {
    /// Calculate tax for this product (10% of cost)
    fn calculate_tax(&self) -> f64 {
        self.cost.as_ref().map(|c| c.amount * 0.1).unwrap_or(0.0)
    }

    /// Validate that the product has a valid supply chain
    fn validate_supply_chain(&self) -> Result<(), ModelError> {
        if let Some(sc) = &self.supply_chain {
            if sc.chain.is_empty() {
                return Err(ModelError::InvalidData("Supply chain must not be empty".to_string()));
            }
            Ok(())
        } else {
            Err(ModelError::MissingField("supply_chain".to_string()))
        }
    }

    /// Calculate total cost including material and labor
    fn total_cost(&self) -> Option<f64> {
        let material = self.material_cost.unwrap_or(0.0) as f64;
        let labor = self.labor_cost.unwrap_or(0.0) as f64;
        Some(material + labor)
    }
}