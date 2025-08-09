// Privacy-related constants and utilities
pub const DATA_PROCESSING_CONSENT: &str = "data_processing";
pub const MARKETING_CONSENT: &str = "marketing";
pub const ANALYTICS_CONSENT: &str = "analytics";
pub const THIRD_PARTY_SHARING_CONSENT: &str = "third_party_sharing";

pub fn is_required_consent(consent_type: &str) -> bool {
    matches!(consent_type, DATA_PROCESSING_CONSENT)
}

pub fn get_consent_description(consent_type: &str) -> &'static str {
    match consent_type {
        DATA_PROCESSING_CONSENT => "Allow processing of personal data for core functionality",
        MARKETING_CONSENT => "Allow use of data for marketing communications",
        ANALYTICS_CONSENT => "Allow collection of usage analytics",
        THIRD_PARTY_SHARING_CONSENT => "Allow sharing data with trusted third parties",
        _ => "Unknown consent type",
    }
}
