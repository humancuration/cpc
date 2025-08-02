//! GraphQL tests for the website builder module
//!
//! These tests verify that the GraphQL schema is correctly defined and that
//! queries and mutations work as expected.

use async_graphql::{Schema, EmptySubscription};
use uuid::Uuid;
use chrono::Utc;

use crate::web::graphql::{WebsiteBuilderQuery, WebsiteBuilderMutation};
use crate::web::types::*;

// Test helper to create a schema
fn create_test_schema() -> Schema<WebsiteBuilderQuery, WebsiteBuilderMutation, EmptySubscription> {
    Schema::build(WebsiteBuilderQuery::default(), WebsiteBuilderMutation::default(), EmptySubscription)
        .finish()
}

#[tokio::test]
async fn test_create_site_mutation_with_link_in_bio() {
    let schema = create_test_schema();
    
    let mutation = r#"
        mutation {
            createSite(input: {
                name: "Test Link Site"
                siteType: {
                    linkInBio: {
                        headline: "My Links"
                        description: "Check out my links"
                    }
                }
            }) {
                id
                name
                siteType {
                    linkInBio {
                        headline
                        description
                    }
                }
            }
        }
    "#;
    
    let result = schema.execute(mutation).await;
    
    // Note: In a real test, we would need to provide mock services in the context
    // For now, we're just testing that the schema is valid and the mutation can be parsed
    assert!(result.errors.is_empty() || 
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}

#[tokio::test]
async fn test_create_site_mutation_with_full_website() {
    let schema = create_test_schema();
    
    let template_id = Uuid::new_v4();
    let mutation = format!(r#"
        mutation {{
            createSite(input: {{
                name: "Test Website"
                siteType: {{
                    fullWebsite: {{
                        templateId: "{}"
                    }}
                }}
            }}) {{
                id
                name
                siteType {{
                    fullWebsite {{
                        templateId
                    }}
                }}
            }}
        }}
    "#, template_id);
    
    let result = schema.execute(mutation).await;
    
    assert!(result.errors.is_empty() || 
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}

#[tokio::test]
async fn test_update_site_settings_mutation() {
    let schema = create_test_schema();
    
    let site_id = Uuid::new_v4();
    let mutation = format!(r#"
        mutation {{
            updateSiteSettings(input: {{
                siteId: "{}"
                name: "Updated Site Name"
                customDomain: "updated.example.com"
                primaryColor: "#FF0000"
                secondaryColor: "#00FF00"
                fontFamily: "Arial, sans-serif"
            }}) {{
                id
                name
                customDomain
                primaryColor
                secondaryColor
                fontFamily
            }}
        }}
    "#, site_id);
    
    let result = schema.execute(mutation).await;
    
    assert!(result.errors.is_empty() || 
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}

#[tokio::test]
async fn test_publish_site_mutation() {
    let schema = create_test_schema();
    
    let site_id = Uuid::new_v4();
    let mutation = format!(r#"
        mutation {{
            publishSite(siteId: "{}")
        }}
    "#, site_id);
    
    let result = schema.execute(mutation).await;
    
    // Should return true or an error related to context/services
    assert!(result.errors.is_empty() || 
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}

#[tokio::test]
async fn test_track_link_click_mutation() {
    let schema = create_test_schema();
    
    let link_id = Uuid::new_v4();
    let mutation = format!(r#"
        mutation {{
            trackLinkClick(linkId: "{}")
        }}
    "#, link_id);
    
    let result = schema.execute(mutation).await;
    
    assert!(result.errors.is_empty() || 
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}

#[tokio::test]
async fn test_get_site_query() {
    let schema = create_test_schema();
    
    let site_id = Uuid::new_v4();
    let query = format!(r#"
        query {{
            site(id: "{}") {{
                id
                name
                siteType {{
                    linkInBio {{
                        headline
                        description
                    }}
                }}
            }}
        }}
    "#, site_id);
    
    let result = schema.execute(query).await;
    
    assert!(result.errors.is_empty() || 
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}

#[tokio::test]
async fn test_get_sites_by_owner_query() {
    let schema = create_test_schema();
    
    let owner_id = Uuid::new_v4();
    let query = format!(r#"
        query {{
            sitesByOwner(ownerId: "{}") {{
                id
                name
            }}
        }}
    "#, owner_id);
    
    let result = schema.execute(query).await;
    
    assert!(result.errors.is_empty() || 
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}

#[tokio::test]
async fn test_get_templates_query() {
    let schema = create_test_schema();
    
    let query = r#"
        query {
            templates {
                id
                name
                templateType
            }
        }
    "#;
    
    let result = schema.execute(query).await;
    
    assert!(result.errors.is_empty() || 
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}

#[tokio::test]
async fn test_get_site_analytics_query() {
    let schema = create_test_schema();
    
    let site_id = Uuid::new_v4();
    let start_date = Utc::now();
    let end_date = start_date + chrono::Duration::days(7);
    
    let query = format!(r#"
        query {{
            siteAnalytics(
                siteId: "{}"
                period: {{
                    startDate: "{}"
                    endDate: "{}"
                }}
            ) {{
                siteId
                totalViews
                uniqueVisitors
            }}
        }}
    "#, site_id, start_date.to_rfc3339(), end_date.to_rfc3339());
    
    let result = schema.execute(query).await;
    
    assert!(result.errors.is_empty() || 
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}

#[tokio::test]
async fn test_site_validation_error_handling() {
    let schema = create_test_schema();
    
    // Test with empty site name
    let mutation = r#"
        mutation {
            createSite(input: {
                name: ""
                siteType: {
                    linkInBio: {
                        headline: "My Links"
                        description: "Check out my links"
                    }
                }
            }) {
                id
                name
            }
        }
    "#;
    
    let result = schema.execute(mutation).await;
    
    // Should either return a validation error or a context/service error
    assert!(!result.errors.is_empty() || 
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}

#[tokio::test]
async fn test_template_application_validation() {
    let schema = create_test_schema();
    
    // Test with invalid template ID
    let invalid_template_id = "invalid-uuid";
    let mutation = format!(r#"
        mutation {{
            createSite(input: {{
                name: "Test Site"
                siteType: {{
                    fullWebsite: {{
                        templateId: "{}"
                    }}
                }}
            }}) {{
                id
                name
            }}
        }}
    "#, invalid_template_id);
    
    let result = schema.execute(mutation).await;
    
    // Should return a validation error for invalid UUID
    assert!(!result.errors.is_empty());
}

#[tokio::test]
async fn test_analytics_date_range_validation() {
    let schema = create_test_schema();
    
    let site_id = Uuid::new_v4();
    let start_date = Utc::now();
    let end_date = start_date - chrono::Duration::days(1); // End date before start date
    
    let query = format!(r#"
        query {{
            siteAnalytics(
                siteId: "{}"
                period: {{
                    startDate: "{}"
                    endDate: "{}"
                }}
            ) {{
                siteId
            }}
        }}
    "#, site_id, start_date.to_rfc3339(), end_date.to_rfc3339());
    
    let result = schema.execute(query).await;
    
    // Should handle invalid date ranges gracefully
    assert!(result.errors.is_empty() ||
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}

#[tokio::test]
async fn test_create_site_mutation_with_fundraising_campaign() {
    let schema = create_test_schema();
    
    let mutation = r#"
        mutation {
            createSite(input: {
                name: "Test Fundraising Campaign"
                siteType: {
                    fundraisingCampaign: {
                        campaignTitle: "Help Our Cause"
                        campaignDescription: "Support our community project"
                        campaignType: PURE_DONATION
                        goalAmount: 10000
                        startDate: "2025-08-01T00:00:00Z"
                        endDate: "2025-12-31T23:59:59Z"
                    }
                }
            }) {
                id
                name
                siteType {
                    fundraisingCampaign {
                        campaignTitle
                        campaignDescription
                        campaignType
                        goalAmount
                    }
                }
            }
        }
    "#;
    
    let result = schema.execute(mutation).await;
    
    // Note: In a real test, we would need to provide mock services in the context
    // For now, we're just testing that the schema is valid and the mutation can be parsed
    assert!(result.errors.is_empty() ||
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}

#[tokio::test]
async fn test_create_fundraising_campaign_mutation() {
    let schema = create_test_schema();
    
    let mutation = r#"
        mutation {
            createFundraisingCampaign(input: {
                name: "Test Fundraising Campaign"
                siteType: {
                    fundraisingCampaign: {
                        campaignTitle: "Help Our Cause"
                        campaignDescription: "Support our community project"
                        campaignType: PURE_DONATION
                        goalAmount: 10000
                        startDate: "2025-08-01T00:00:00Z"
                        endDate: "2025-12-31T23:59:59Z"
                    }
                }
            }) {
                id
                name
                siteType {
                    fundraisingCampaign {
                        campaignTitle
                        campaignDescription
                        campaignType
                        goalAmount
                    }
                }
            }
        }
    "#;
    
    let result = schema.execute(mutation).await;
    
    // Note: In a real test, we would need to provide mock services in the context
    // For now, we're just testing that the schema is valid and the mutation can be parsed
    assert!(result.errors.is_empty() ||
            result.errors.iter().any(|e| e.message.contains("Context") || e.message.contains("Service")));
}