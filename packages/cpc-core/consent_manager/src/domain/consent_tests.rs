//! Tests for consent domain logic.

#[cfg(test)]
mod tests {
    use super::super::consent::*;

    #[test]
    fn test_data_sharing_level_priority() {
        assert_eq!(DataSharingLevel::None.priority(), 0);
        assert_eq!(DataSharingLevel::Minimal.priority(), 1);
        assert_eq!(DataSharingLevel::Standard.priority(), 2);
        assert_eq!(DataSharingLevel::Full.priority(), 3);
    }

    #[test]
    fn test_consent_profile_creation() {
        let profile = ConsentProfile::new(
            "user123".to_string(),
            Domain::FinancialData,
            DataSharingLevel::Standard,
        );
        
        assert_eq!(profile.user_id, "user123");
        assert_eq!(profile.domain, Domain::FinancialData);
        assert_eq!(profile.level, DataSharingLevel::Standard);
        assert_eq!(profile.created_at, profile.updated_at);
    }

    #[test]
    fn test_consent_profile_get_level() {
        let profile = ConsentProfile::new(
            "user123".to_string(),
            Domain::FinancialData,
            DataSharingLevel::Standard,
        );
        
        assert_eq!(profile.get_level(), &DataSharingLevel::Standard);
    }

    #[test]
    fn test_consent_profile_set_level() {
        let mut profile = ConsentProfile::new(
            "user123".to_string(),
            Domain::FinancialData,
            DataSharingLevel::Standard,
        );
        
        let old_updated_at = profile.updated_at;
        
        // Setting the same level should return an error
        assert_eq!(
            profile.set_level(DataSharingLevel::Standard),
            Err(crate::domain::errors::ConsentError::NoChange)
        );
        
        // Setting a different level should succeed
        assert!(profile.set_level(DataSharingLevel::Full).is_ok());
        assert_eq!(profile.level, DataSharingLevel::Full);
        assert!(profile.updated_at > old_updated_at);
    }

    #[test]
    fn test_consent_profile_allows() {
        let profile = ConsentProfile::new(
            "user123".to_string(),
            Domain::FinancialData,
            DataSharingLevel::Standard,
        );
        
        // Profile with Standard level should allow Minimal and Standard, but not Full
        assert!(profile.allows(DataSharingLevel::Minimal));
        assert!(profile.allows(DataSharingLevel::Standard));
        assert!(!profile.allows(DataSharingLevel::Full));
        assert!(!profile.allows(DataSharingLevel::None)); // None is always allowed
        
        // Profile with None level should only allow None
        let profile_none = ConsentProfile::new(
            "user123".to_string(),
            Domain::FinancialData,
            DataSharingLevel::None,
        );
        
        assert!(!profile_none.allows(DataSharingLevel::Minimal));
        assert!(!profile_none.allows(DataSharingLevel::Standard));
        assert!(!profile_none.allows(DataSharingLevel::Full));
    }
}