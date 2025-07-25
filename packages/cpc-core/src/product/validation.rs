use crate::errors::ModelError;
use crate::models::product::Product;
use validator::Validate;

pub trait ProductValidate {
    fn full_validate(&self) -> Result<(), ModelError>;
}

impl ProductValidate for Product {
    fn full_validate(&self) -> Result<(), ModelError> {
        // First validate basic fields using validator crate
        self.validate()
            .map_err(|e| ModelError::ValidationFailed(e.to_string()))?;

        // Then validate business rules
        if self.cost.is_none() && self.material_cost.is_none() && self.labor_cost.is_none() {
            return Err(ModelError::ValidationFailed(
                "Product must have at least one cost field set".to_string(),
            ));
        }

        Ok(())
    }
}