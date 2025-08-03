//! Alternative product recommendation engine
//! Basic matching logic

use crate::data_models::{Product, AlternativeSuggestion, SuggestionReason, ProductType};
use std::error::Error;
use std::fmt;

/// Custom error type for suggestions operations
#[derive(Debug)]
pub enum SuggestionsError {
    DataFetchError(String),
    MatchingError(String),
}

impl fmt::Display for SuggestionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SuggestionsError::DataFetchError(msg) => write!(f, "Data fetch error: {}", msg),
            SuggestionsError::MatchingError(msg) => write!(f, "Matching error: {}", msg),
        }
    }
}

impl Error for SuggestionsError {}

/// Suggestions service for recommending alternative products
pub struct SuggestionsService;

impl SuggestionsService {
    /// Create a new suggestions service
    pub fn new() -> Self {
        Self
    }

    /// Find alternative products based on a reference product
    pub async fn find_alternatives(&self, product: &Product) -> Result<Vec<AlternativeSuggestion>, SuggestionsError> {
        // For healthcare products, look for supplement alternatives
        if product.product_type == ProductType::Healthcare {
            self.find_supplement_alternatives(product).await
        } else {
            // Standard matching for other product types
            self.find_standard_alternatives(product).await
        }
    }
    
    /// Find standard alternatives for a product
    async fn find_standard_alternatives(&self, product: &Product) -> Result<Vec<AlternativeSuggestion>, SuggestionsError> {
        // Placeholder for alternative product matching
        // In a real implementation, this would:
        // 1. Query database for similar products
        // 2. Apply matching algorithms based on category, health score, ethical score, etc.
        // 3. Rank alternatives by various criteria
        // 4. Return ranked list of suggestions
        
        // For now, we'll return mock suggestions
        let suggestions = vec![
            AlternativeSuggestion {
                product_id: uuid::Uuid::new_v4(),
                reason: SuggestionReason::HealthierOption,
                health_improvement: 0.15,
                ethical_improvement: 0.05,
            },
            AlternativeSuggestion {
                product_id: uuid::Uuid::new_v4(),
                reason: SuggestionReason::MoreEthical,
                health_improvement: -0.05,
                ethical_improvement: 0.25,
            },
            AlternativeSuggestion {
                product_id: uuid::Uuid::new_v4(),
                reason: SuggestionReason::LocalProducer,
                health_improvement: 0.02,
                ethical_improvement: 0.15,
            },
        ];
        
        Ok(suggestions)
    }
    
    /// Find supplement alternatives for healthcare products
    async fn find_supplement_alternatives(&self, _product: &Product) -> Result<Vec<AlternativeSuggestion>, SuggestionsError> {
        // Placeholder for finding supplement alternatives to healthcare products
        // In a real implementation, this would:
        // 1. Query database for supplements with similar benefits
        // 2. Apply matching algorithms based on health conditions, ingredients, etc.
        // 3. Rank alternatives by various criteria
        // 4. Return ranked list of suggestions
        
        // For now, we'll return mock suggestions
        let suggestions = vec![
            AlternativeSuggestion {
                product_id: uuid::Uuid::new_v4(),
                reason: SuggestionReason::HealthierOption,
                health_improvement: 0.2,
                ethical_improvement: 0.1,
            },
            AlternativeSuggestion {
                product_id: uuid::Uuid::new_v4(),
                reason: SuggestionReason::MoreEthical,
                health_improvement: 0.1,
                ethical_improvement: 0.3,
            },
        ];
        
        Ok(suggestions)
    }

    /// Get detailed information for alternative products
    pub async fn get_alternative_details(&self, suggestions: &[AlternativeSuggestion]) -> Result<Vec<Product>, SuggestionsError> {
        // Placeholder for fetching detailed product information
        // In a real implementation, this would query the database for each suggested product
        
        let mut products = Vec::new();
        
        for suggestion in suggestions {
            // Mock product data
            let product = Product {
                id: suggestion.product_id,
                barcode: format!("ALT{}", suggestion.product_id.to_string()[..8].to_string()),
                name: format!("Alternative Product {}", &suggestion.product_id.to_string()[..4]),
                brand: "Alternative Brand".to_string(),
                category: "Food".to_string(),
                product_type: ProductType::Food,
                ingredients: vec![],
                nutritional_info: crate::data_models::NutritionalFacts {
                    calories: 80.0,
                    protein: 8.0,
                    carbs: 15.0,
                    fats: 1.5,
                    sugars: 5.0,
                    fiber: 5.0,
                    sodium: 0.05,
                },
                cosmetic_ingredients: vec![],
                supplement_facts: None,
                ethical_score: 0.85,
                supply_chain: vec![],
            };
            
            products.push(product);
        }
        
        Ok(products)
    }

    /// Rank alternatives based on user preferences
    pub fn rank_alternatives(
        &self,
        suggestions: Vec<AlternativeSuggestion>,
        preference: &UserPreference,
    ) -> Vec<AlternativeSuggestion> {
        let mut ranked = suggestions;
        
        ranked.sort_by(|a, b| {
            let score_a = self.calculate_suggestion_score(a, preference);
            let score_b = self.calculate_suggestion_score(b, preference);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        ranked
    }

    /// Calculate suggestion score based on user preferences
    fn calculate_suggestion_score(&self, suggestion: &AlternativeSuggestion, preference: &UserPreference) -> f32 {
        let health_weight = preference.health_weight;
        let ethical_weight = preference.ethical_weight;
        let local_weight = preference.local_weight;
        
        let health_component = suggestion.health_improvement * health_weight;
        let ethical_component = suggestion.ethical_improvement * ethical_weight;
        
        let local_bonus = match suggestion.reason {
            SuggestionReason::LocalProducer => local_weight,
            _ => 0.0,
        };
        
        health_component + ethical_component + local_bonus
    }
}

/// User preferences for alternative product recommendations
#[derive(Debug)]
pub struct UserPreference {
    pub health_weight: f32,
    pub ethical_weight: f32,
    pub local_weight: f32,
}

impl Default for UserPreference {
    fn default() -> Self {
        Self {
            health_weight: 0.5,
            ethical_weight: 0.3,
            local_weight: 0.2,
        }
    }
}

impl Default for SuggestionsService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_models::{Product, AlternativeSuggestion, SuggestionReason};

    #[tokio::test]
    async fn test_suggestions_service_creation() {
        let service = SuggestionsService::new();
        assert!(true); // Simple test to ensure creation works
    }

    #[tokio::test]
    async fn test_find_alternatives() {
        let service = SuggestionsService::new();
        let product = Product {
            id: uuid::Uuid::new_v4(),
            barcode: "123456789012".to_string(),
            name: "Test Product".to_string(),
            brand: "Test Brand".to_string(),
            category: "Food".to_string(),
            ingredients: vec![],
            nutritional_info: crate::data_models::NutritionalFacts {
                calories: 100.0,
                protein: 5.0,
                carbs: 20.0,
                fats: 2.0,
                sugars: 10.0,
                fiber: 3.0,
                sodium: 0.1,
            },
            ethical_score: 0.75,
            supply_chain: vec![],
        };
        
        let suggestions = service.find_alternatives(&product).await;
        assert!(suggestions.is_ok());
        assert!(!suggestions.unwrap().is_empty());
    }

    #[test]
    fn test_suggestion_ranking() {
        let service = SuggestionsService::new();
        let suggestions = vec![
            AlternativeSuggestion {
                product_id: uuid::Uuid::new_v4(),
                reason: SuggestionReason::HealthierOption,
                health_improvement: 0.2,
                ethical_improvement: 0.1,
            },
            AlternativeSuggestion {
                product_id: uuid::Uuid::new_v4(),
                reason: SuggestionReason::MoreEthical,
                health_improvement: 0.05,
                ethical_improvement: 0.3,
            }
        ];
        
        let preference = UserPreference {
            health_weight: 0.6,
            ethical_weight: 0.4,
            local_weight: 0.0,
        };
        
        let ranked = service.rank_alternatives(suggestions, &preference);
        assert_eq!(ranked.len(), 2);
    }
}