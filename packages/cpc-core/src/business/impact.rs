//! Refer to docs/architecture/impact-service.md#business-rules for current implementation

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use thiserror::Error;
use uuid::Uuid;

/// Represents an impact distribution category and its weight.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ImpactDistribution {
    /// The impact category (Community, Environment, Workers)
    pub category: String,
    /// The weight assigned to this category (0.0 to 1.0)
    pub weight: f64,
}

/// Errors that can occur during impact calculation
#[derive(Debug, Error)]
pub enum CalculationError {
    /// Insufficient data available to calculate impact distribution
    #[error("Insufficient data to calculate impact distribution")]
    InsufficientData,
    /// The calculated weights do not sum to exactly 1.0
    #[error("Invalid distribution: weights must sum to exactly 1.0")]
    InvalidDistribution,
    /// The specified user could not be found
    #[error("User data not found")]
    UserNotFound,
}

/// Business interface for calculating impact distributions
pub trait ImpactCalculator {
    /// Calculate the impact distribution for a given user
    async fn calculate(&self, user_id: &str) -> Result<Vec<ImpactDistribution>, CalculationError>;

    /// Validates that distribution weights sum to exactly 1.0 within floating-point tolerance
    fn validate_distribution(weights: &[ImpactDistribution]) -> Result<(), CalculationError> {
        let total: f64 = weights.iter().map(|w| w.weight).sum();
        // Using a small epsilon for floating-point comparison
        if (total - 1.0).abs() > 1e-10 {
            return Err(CalculationError::InvalidDistribution);
        }
        Ok(())
    }
}

/// Default implementation of ImpactCalculator using PostgreSQL
pub struct DefaultImpactCalculator {
    pool: PgPool,
}

impl DefaultImpactCalculator {
    /// Create a new calculator with database connection pool
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ImpactCalculator for DefaultImpactCalculator {
    async fn calculate(&self, user_id: &str) -> Result<Vec<ImpactDistribution>, CalculationError> {
        let user_uuid = Uuid::parse_str(user_id).map_err(|_| CalculationError::UserNotFound)?;

        let weights = sqlx::query_as!(
            ImpactDistribution,
            r#"
            SELECT 
                category AS "category: String",
                weight AS "weight: f64"
            FROM impact_weights
            WHERE user_id = $1
            "#,
            user_uuid
        )
        .fetch_all(&self.pool)
        .await;

        match weights {
            Ok(weights) => {
                if weights.is_empty() {
                    // Return default distribution when no user-specific weights exist
                    let default_weights = vec![
                        ImpactDistribution {
                            category: "Environment".to_string(),
                            weight: 0.33,
                        },
                        ImpactDistribution {
                            category: "Community".to_string(),
                            weight: 0.33,
                        },
                        ImpactDistribution {
                            category: "Workers".to_string(),
                            weight: 0.34,
                        }
                    ];
                    Self::validate_distribution(&default_weights)?;
                    Ok(default_weights)
                } else {
                    Self::validate_distribution(&weights)?;
                    Ok(weights)
                }
            }
            Err(sqlx::Error::RowNotFound) => {
                // Return default distribution for new users
                let default_weights = vec![
                    ImpactDistribution {
                        category: "Environment".to_string(),
                        weight: 0.33,
                    },
                    ImpactDistribution {
                        category: "Community".to_string(),
                        weight: 0.33,
                    },
                    ImpactDistribution {
                        category: "Workers".to_string(),
                        weight: 0.34,
                    }
                ];
                Self::validate_distribution(&default_weights)?;
                Ok(default_weights)
            }
            Err(_) => Err(CalculationError::InsufficientData),
        }
    }
}
