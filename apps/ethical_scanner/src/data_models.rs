use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductType {
    Food,
    Cosmetic,
    Supplement,
    Healthcare,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub barcode: String,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub product_type: ProductType,
    pub ingredients: Vec<Ingredient>,
    pub nutritional_info: NutritionalFacts,
    pub cosmetic_ingredients: Vec<CosmeticIngredient>,
    pub supplement_facts: Option<SupplementFacts>,
    pub ethical_score: f32,
    pub supply_chain: Vec<SupplyChainNode>,
}

impl Product {
    /// Validate the product data
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.barcode.is_empty() {
            return Err(ValidationError::InvalidBarcode("Barcode cannot be empty".to_string()));
        }
        
        if self.name.is_empty() {
            return Err(ValidationError::InvalidName("Name cannot be empty".to_string()));
        }
        
        if self.ethical_score < 0.0 || self.ethical_score > 1.0 {
            return Err(ValidationError::InvalidScore("Ethical score must be between 0.0 and 1.0".to_string()));
        }
        
        // Validate nutritional info
        self.nutritional_info.validate()?;
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub origin: String,
    pub is_allergen: bool,
    pub health_impact: HealthImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmeticIngredient {
    pub name: String,
    pub ewg_score: f32, // 0-10 hazard score
    pub is_allergen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthImpact {
    Positive,
    Neutral,
    Negative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionalFacts {
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fats: f32,
    pub sugars: f32,
    pub fiber: f32,
    pub sodium: f32,
}

impl NutritionalFacts {
    /// Validate the nutritional facts data
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.calories < 0.0 {
            return Err(ValidationError::InvalidNutrition("Calories cannot be negative".to_string()));
        }
        
        if self.protein < 0.0 {
            return Err(ValidationError::InvalidNutrition("Protein cannot be negative".to_string()));
        }
        
        if self.carbs < 0.0 {
            return Err(ValidationError::InvalidNutrition("Carbs cannot be negative".to_string()));
        }
        
        if self.fats < 0.0 {
            return Err(ValidationError::InvalidNutrition("Fats cannot be negative".to_string()));
        }
        
        if self.sugars < 0.0 {
            return Err(ValidationError::InvalidNutrition("Sugars cannot be negative".to_string()));
        }
        
        if self.fiber < 0.0 {
            return Err(ValidationError::InvalidNutrition("Fiber cannot be negative".to_string()));
        }
        
        if self.sodium < 0.0 {
            return Err(ValidationError::InvalidNutrition("Sodium cannot be negative".to_string()));
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainNode {
    pub step: String,
    pub location: String,
    pub company: String,
    pub ethical_rating: f32,
    pub environmental_impact: f32,
}

impl SupplyChainNode {
    /// Validate the supply chain node data
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.step.is_empty() {
            return Err(ValidationError::InvalidSupplyChain("Step cannot be empty".to_string()));
        }
        
        if self.ethical_rating < 0.0 || self.ethical_rating > 1.0 {
            return Err(ValidationError::InvalidScore("Ethical rating must be between 0.0 and 1.0".to_string()));
        }
        
        if self.environmental_impact < 0.0 || self.environmental_impact > 1.0 {
            return Err(ValidationError::InvalidScore("Environmental impact must be between 0.0 and 1.0".to_string()));
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeSuggestion {
    pub product_id: Uuid,
    pub reason: SuggestionReason,
    pub health_improvement: f32,
    pub ethical_improvement: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplementFacts {
    pub daily_value_percentage: f32,
    pub is_verified: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionReason {
    HealthierOption,
    MoreEthical,
    LocalProducer,
    SustainablePackaging,
}

/// Custom error type for validation errors
#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidBarcode(String),
    InvalidName(String),
    InvalidScore(String),
    InvalidNutrition(String),
    InvalidSupplyChain(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::InvalidBarcode(msg) => write!(f, "Invalid barcode: {}", msg),
            ValidationError::InvalidName(msg) => write!(f, "Invalid name: {}", msg),
            ValidationError::InvalidScore(msg) => write!(f, "Invalid score: {}", msg),
            ValidationError::InvalidNutrition(msg) => write!(f, "Invalid nutrition: {}", msg),
            ValidationError::InvalidSupplyChain(msg) => write!(f, "Invalid supply chain: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}