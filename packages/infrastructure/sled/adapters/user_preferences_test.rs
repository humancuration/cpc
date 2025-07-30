//! Unit tests for the Sled user preferences adapter

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::user_preferences::SledUserPreferences;
    use packages::domains::finance::domain::primitives::Currency;
    use packages::domains::finance::application::user_preferences::UserPreferences;
    use tempfile::TempDir;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_set_and_get_currency() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let preferences = SledUserPreferences::new(&db);
        
        let user_id = Uuid::new_v4();
        
        // Set currency
        let result = preferences.set_preferred_currency(user_id, Currency::EUR).await;
        assert!(result.is_ok());
        
        // Get currency
        let currency = preferences.get_preferred_currency(user_id).await;
        assert!(currency.is_ok());
        assert_eq!(currency.unwrap(), Currency::EUR);
    }
    
    #[tokio::test]
    async fn test_get_nonexistent_currency() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let preferences = SledUserPreferences::new(&db);
        
        let user_id = Uuid::new_v4();
        
        // Try to get currency for user with no preference set
        let result = preferences.get_preferred_currency(user_id).await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_update_currency() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let preferences = SledUserPreferences::new(&db);
        
        let user_id = Uuid::new_v4();
        
        // Set initial currency
        let result = preferences.set_preferred_currency(user_id, Currency::USD).await;
        assert!(result.is_ok());
        
        // Update currency
        let result = preferences.set_preferred_currency(user_id, Currency::JPY).await;
        assert!(result.is_ok());
        
        // Get updated currency
        let currency = preferences.get_preferred_currency(user_id).await;
        assert!(currency.is_ok());
        assert_eq!(currency.unwrap(), Currency::JPY);
    }
    
    #[tokio::test]
    async fn test_multiple_users() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let preferences = SledUserPreferences::new(&db);
        
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();
        
        // Set different currencies for different users
        let result1 = preferences.set_preferred_currency(user1_id, Currency::EUR).await;
        assert!(result1.is_ok());
        
        let result2 = preferences.set_preferred_currency(user2_id, Currency::GBP).await;
        assert!(result2.is_ok());
        
        // Get currencies for each user
        let currency1 = preferences.get_preferred_currency(user1_id).await;
        assert!(currency1.is_ok());
        assert_eq!(currency1.unwrap(), Currency::EUR);
        
        let currency2 = preferences.get_preferred_currency(user2_id).await;
        assert!(currency2.is_ok());
        assert_eq!(currency2.unwrap(), Currency::GBP);
    }
    
    #[tokio::test]
    async fn test_conflict_resolution() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let preferences = SledUserPreferences::new(&db);
        
        let user_id = Uuid::new_v4();
        
        // Create two preferences with different timestamps
        let stored_old = StoredPreference {
            currency_code: "USD".to_string(),
            synced: false,
            timestamp: 1000,
        };
        
        let stored_new = StoredPreference {
            currency_code: "EUR".to_string(),
            synced: false,
            timestamp: 2000,
        };
        
        // The newer one should be chosen
        let resolved = preferences.resolve_conflict(&stored_old, &stored_new);
        assert_eq!(resolved.timestamp, 2000);
    }
}