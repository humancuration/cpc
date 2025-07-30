//! Database repository for currency preferences

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    domain::currency::{Currency, CurrencyCode},
    application::currency::user_prefs::{UserCurrencyPreferences, UserCurrencyPreferencesRepository, CurrencyPreferencesError},
};

/// Database repository for user currency preferences
pub struct DatabaseCurrencyPreferencesRepository {
    pool: PgPool,
}

impl DatabaseCurrencyPreferencesRepository {
    /// Create a new database currency preferences repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserCurrencyPreferencesRepository for DatabaseCurrencyPreferencesRepository {
    async fn get_preferences(&self, user_id: Uuid) -> Result<UserCurrencyPreferences, CurrencyPreferencesError> {
        let row = sqlx::query!(
            r#"
            SELECT 
                ucp.default_currency,
                ucp.preferred_locale,
                ucp.show_currency_symbols,
                c.name as currency_name,
                c.symbol as currency_symbol,
                c.decimal_places as currency_decimal_places,
                c.is_dabloon as currency_is_dabloon
            FROM user_currency_preferences ucp
            JOIN currencies c ON ucp.default_currency = c.code
            WHERE ucp.user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CurrencyPreferencesError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                let currency = Currency::new(
                    CurrencyCode::new(row.default_currency),
                    row.currency_name,
                    row.currency_symbol,
                    row.currency_decimal_places as u32,
                    row.currency_is_dabloon,
                );
                
                Ok(UserCurrencyPreferences::new(
                    user_id,
                    currency,
                    row.preferred_locale,
                ))
            }
            None => {
                // Return default preferences if none exist
                let currency: Currency = CurrencyCode::new("USD").into();
                Ok(UserCurrencyPreferences::new(
                    user_id,
                    currency,
                    "en-US".to_string(),
                ))
            }
        }
    }

    async fn save_preferences(&self, preferences: &UserCurrencyPreferences) -> Result<(), CurrencyPreferencesError> {
        sqlx::query!(
            r#"
            INSERT INTO user_currency_preferences (
                user_id,
                default_currency,
                preferred_locale,
                show_currency_symbols
            ) VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id) DO UPDATE SET
                default_currency = EXCLUDED.default_currency,
                preferred_locale = EXCLUDED.preferred_locale,
                show_currency_symbols = EXCLUDED.show_currency_symbols,
                updated_at = NOW()
            "#,
            preferences.user_id,
            preferences.default_currency.code(),
            preferences.preferred_locale,
            preferences.show_currency_symbols
        )
        .execute(&self.pool)
        .await
        .map_err(|e| CurrencyPreferencesError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn set_default_currency(&self, user_id: Uuid, currency_code: CurrencyCode) -> Result<(), CurrencyPreferencesError> {
        // First check if the currency exists
        let currency_exists = sqlx::query!(
            "SELECT 1 FROM currencies WHERE code = $1",
            currency_code.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CurrencyPreferencesError::DatabaseError(e.to_string()))?
        .is_some();

        if !currency_exists {
            return Err(CurrencyPreferencesError::InvalidCurrencyCode(currency_code.as_str().to_string()));
        }

        sqlx::query!(
            r#"
            INSERT INTO user_currency_preferences (user_id, default_currency)
            VALUES ($1, $2)
            ON CONFLICT (user_id) DO UPDATE SET
                default_currency = EXCLUDED.default_currency,
                updated_at = NOW()
            "#,
            user_id,
            currency_code.as_str()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| CurrencyPreferencesError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_save_and_get_preferences(pool: PgPool) -> sqlx::Result<()> {
        let repo = DatabaseCurrencyPreferencesRepository::new(pool);
        let user_id = Uuid::new_v4();
        let currency: Currency = CurrencyCode::new("USD").into();
        let preferences = UserCurrencyPreferences::new(user_id, currency, "en-US".to_string());

        // Save preferences
        repo.save_preferences(&preferences).await.unwrap();

        // Get preferences
        let retrieved = repo.get_preferences(user_id).await.unwrap();
        assert_eq!(retrieved.user_id, user_id);
        assert_eq!(retrieved.default_currency_code(), "USD");
        assert_eq!(retrieved.preferred_locale, "en-US");

        Ok(())
    }

    #[sqlx::test]
    async fn test_set_default_currency(pool: PgPool) -> sqlx::Result<()> {
        let repo = DatabaseCurrencyPreferencesRepository::new(pool);
        let user_id = Uuid::new_v4();

        // Set default currency
        repo.set_default_currency(user_id, CurrencyCode::new("EUR")).await.unwrap();

        // Get preferences and check currency
        let retrieved = repo.get_preferences(user_id).await.unwrap();
        assert_eq!(retrieved.default_currency_code(), "EUR");

        Ok(())
    }
}