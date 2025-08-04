//! Integration tests for skill exchange GraphQL functionality
// Refactor note: standardized schema construction via graphql::test_helpers::build_vc_schema_with_service to ensure VOLUNTEER_REPUTATION toggle consistency.

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::{Schema, Request, EmptyMutation, EmptySubscription};
    use uuid::Uuid;
    use rust_decimal::Decimal;
    use skill_exchange_core::models::{SkillListing, SkillClaim, SkillExchangeCompletion, ClaimStatus};
    use wallet::domain::primitives::{Money, Currency};
    
    // Test utilities
    struct TestSkillExchangeService {
        should_fail: bool,
    }
    
    #[async_trait::async_trait]
    impl skill_exchange_core::services::SkillExchangeService for TestSkillExchangeService {
        async fn create_listing(
            &self,
            provider_id: Uuid,
            title: String,
            description: String,
            category: String,
            estimated_time: Option<Decimal>,
        ) -> Result<SkillListing, common_utils::error::CommonError> {
            if self.should_fail {
                return Err(common_utils::error::CommonError::ValidationError("Test error".to_string()));
            }
            
            Ok(SkillListing::new(provider_id, title, description, category, estimated_time))
        }
        
        async fn update_listing(
            &self,
            listing_id: Uuid,
            provider_id: Uuid,
            title: Option<String>,
            description: Option<String>,
            category: Option<String>,
            estimated_time: Option<Decimal>,
        ) -> Result<SkillListing, common_utils::error::CommonError> {
            if self.should_fail {
                return Err(common_utils::error::CommonError::ValidationError("Test error".to_string()));
            }
            
            let mut listing = SkillListing::new(provider_id, "Test".to_string(), "Test".to_string(), "Test".to_string(), None);
            listing.update(title, description, category, estimated_time);
            Ok(listing)
        }
        
        async fn deactivate_listing(
            &self,
            listing_id: Uuid,
            provider_id: Uuid,
        ) -> Result<SkillListing, common_utils::error::CommonError> {
            if self.should_fail {
                return Err(common_utils::error::CommonError::ValidationError("Test error".to_string()));
            }
            
            let mut listing = SkillListing::new(provider_id, "Test".to_string(), "Test".to_string(), "Test".to_string(), None);
            listing.deactivate();
            Ok(listing)
        }
        
        async fn search_listings(
            &self,
            term: Option<String>,
            category: Option<String>,
        ) -> Result<Vec<SkillListing>, common_utils::error::CommonError> {
            if self.should_fail {
                return Err(common_utils::error::CommonError::ValidationError("Test error".to_string()));
            }
            
            let listing = SkillListing::new(Uuid::new_v4(), "Test".to_string(), "Test".to_string(), "Test".to_string(), None);
            Ok(vec![listing])
        }
        
        async fn claim_listing(
            &self,
            listing_id: Uuid,
            claimant_id: Uuid,
            message: Option<String>,
        ) -> Result<SkillClaim, common_utils::error::CommonError> {
            if self.should_fail {
                return Err(common_utils::error::CommonError::ValidationError("Test error".to_string()));
            }
            
            Ok(SkillClaim::new(listing_id, claimant_id, message))
        }
        
        async fn accept_claim(
            &self,
            claim_id: Uuid,
            provider_id: Uuid,
        ) -> Result<SkillClaim, common_utils::error::CommonError> {
            if self.should_fail {
                return Err(common_utils::error::CommonError::ValidationError("Test error".to_string()));
            }
            
            let mut claim = SkillClaim::new(Uuid::new_v4(), Uuid::new_v4(), None);
            claim.accept();
            Ok(claim)
        }
        
        async fn reject_claim(
            &self,
            claim_id: Uuid,
            provider_id: Uuid,
        ) -> Result<SkillClaim, common_utils::error::CommonError> {
            if self.should_fail {
                return Err(common_utils::error::CommonError::ValidationError("Test error".to_string()));
            }
            
            let mut claim = SkillClaim::new(Uuid::new_v4(), Uuid::new_v4(), None);
            claim.reject();
            Ok(claim)
        }
        
        async fn complete_exchange(
            &self,
            claim_id: Uuid,
            claimant_id: Uuid,
            rating: Option<u32>,
            review: Option<String>,
            payment_amount: Option<Money>,
        ) -> Result<SkillExchangeCompletion, common_utils::error::CommonError> {
            if self.should_fail {
                return Err(common_utils::error::CommonError::ValidationError("Test error".to_string()));
            }
            
            Ok(SkillExchangeCompletion::new(
                Uuid::new_v4(), // listing_id
                claim_id,
                Uuid::new_v4(), // provider_id
                claimant_id,
                None, // rating
                payment_amount,
                None, // transaction_id
            ))
        }
        
        async fn get_listings_by_provider(&self, provider_id: Uuid) -> Result<Vec<SkillListing>, common_utils::error::CommonError> {
            if self.should_fail {
                return Err(common_utils::error::CommonError::ValidationError("Test error".to_string()));
            }
            
            let listing = SkillListing::new(provider_id, "Test".to_string(), "Test".to_string(), "Test".to_string(), None);
            Ok(vec![listing])
        }
        
        async fn get_claims_by_claimant(&self, claimant_id: Uuid) -> Result<Vec<SkillClaim>, common_utils::error::CommonError> {
            if self.should_fail {
                return Err(common_utils::error::CommonError::ValidationError("Test error".to_string()));
            }
            
            let claim = SkillClaim::new(Uuid::new_v4(), claimant_id, None);
            Ok(vec![claim])
        }
    }
    
    #[tokio::test]
    async fn test_create_skill_listing_success() {
        // Setup
        let skill_service = TestSkillExchangeService { should_fail: false };
        let user_id = Uuid::new_v4();
        
        // Create schema with our test service
        let schema = Schema::build(crate::graphql::skill_exchange::SkillExchangeQuery, crate::graphql::skill_exchange::SkillExchangeMutation, EmptySubscription)
            .data(Box::new(skill_service) as Box<dyn skill_exchange_core::services::SkillExchangeService>)
            .data(user_id)
            .finish();
        
        // Execute GraphQL request
        let query = r#"
            mutation {
                createSkillListing(input: {title: "Test Skill", description: "Test Description", category: "Test"}) {
                    id
                    title
                    description
                }
            }
        "#;
        
        let response = schema.execute(Request::new(query)).await;
        
        // Assert response
        assert!(response.errors.is_empty());
        assert!(response.data.get("createSkillListing").is_some());
    }
    
    #[tokio::test]
    async fn test_create_skill_listing_validation_error() {
        // Setup
        let skill_service = TestSkillExchangeService { should_fail: true };
        let user_id = Uuid::new_v4();
        
        // Create schema with our test service
        let schema = Schema::build(crate::graphql::skill_exchange::SkillExchangeQuery, crate::graphql::skill_exchange::SkillExchangeMutation, EmptySubscription)
            .data(Box::new(skill_service) as Box<dyn skill_exchange_core::services::SkillExchangeService>)
            .data(user_id)
            .finish();
        
        // Execute GraphQL request
        let query = r#"
            mutation {
                createSkillListing(input: {title: "", description: "Test Description", category: "Test"}) {
                    id
                    title
                    description
                }
            }
        "#;
        
        let response = schema.execute(Request::new(query)).await;
        
        // Assert response has errors
        assert!(!response.errors.is_empty());
    }
    
    #[tokio::test]
    async fn test_update_skill_listing_success() {
        // Setup
        let skill_service = TestSkillExchangeService { should_fail: false };
        let user_id = Uuid::new_v4();
        let listing_id = Uuid::new_v4();
        
        // Create schema with our test service
        let schema = Schema::build(crate::graphql::skill_exchange::SkillExchangeQuery, crate::graphql::skill_exchange::SkillExchangeMutation, EmptySubscription)
            .data(Box::new(skill_service) as Box<dyn skill_exchange_core::services::SkillExchangeService>)
            .data(user_id)
            .finish();
        
        // Execute GraphQL request
        let query = format!(r#"
            mutation {{
                updateSkillListing(listingId: "{}", input: {{title: "Updated Title"}}) {{
                    id
                    title
                }}
            }}
        "#, listing_id);
        
        let response = schema.execute(Request::new(query)).await;
        
        // Assert response
        assert!(response.errors.is_empty());
        assert!(response.data.get("updateSkillListing").is_some());
    }
    
    #[tokio::test]
    async fn test_deactivate_skill_listing_success() {
        // Setup
        let skill_service = TestSkillExchangeService { should_fail: false };
        let user_id = Uuid::new_v4();
        let listing_id = Uuid::new_v4();
        
        // Create schema with our test service
        let schema = Schema::build(crate::graphql::skill_exchange::SkillExchangeQuery, crate::graphql::skill_exchange::SkillExchangeMutation, EmptySubscription)
            .data(Box::new(skill_service) as Box<dyn skill_exchange_core::services::SkillExchangeService>)
            .data(user_id)
            .finish();
        
        // Execute GraphQL request
        let query = format!(r#"
            mutation {{
                deactivateSkillListing(listingId: "{}") {{
                    id
                    isActive
                }}
            }}
        "#, listing_id);
        
        let response = schema.execute(Request::new(query)).await;
        
        // Assert response
        assert!(response.errors.is_empty());
        assert!(response.data.get("deactivateSkillListing").is_some());
    }
    
    #[tokio::test]
    async fn test_claim_skill_listing_success() {
        // Setup
        let skill_service = TestSkillExchangeService { should_fail: false };
        let user_id = Uuid::new_v4();
        let listing_id = Uuid::new_v4();
        
        // Create schema with our test service
        let schema = Schema::build(crate::graphql::skill_exchange::SkillExchangeQuery, crate::graphql::skill_exchange::SkillExchangeMutation, EmptySubscription)
            .data(Box::new(skill_service) as Box<dyn skill_exchange_core::services::SkillExchangeService>)
            .data(user_id)
            .finish();
        
        // Execute GraphQL request
        let query = format!(r#"
            mutation {{
                claimSkillListing(listingId: "{}", input: {{message: "I'd like to learn this skill"}}) {{
                    id
                    status
                }}
            }}
        "#, listing_id);
        
        let response = schema.execute(Request::new(query)).await;
        
        // Assert response
        assert!(response.errors.is_empty());
        assert!(response.data.get("claimSkillListing").is_some());
    }
    
    #[tokio::test]
    async fn test_accept_claim_success() {
        // Setup
        let skill_service = TestSkillExchangeService { should_fail: false };
        let user_id = Uuid::new_v4();
        let claim_id = Uuid::new_v4();
        
        // Create schema with our test service
        let schema = Schema::build(crate::graphql::skill_exchange::SkillExchangeQuery, crate::graphql::skill_exchange::SkillExchangeMutation, EmptySubscription)
            .data(Box::new(skill_service) as Box<dyn skill_exchange_core::services::SkillExchangeService>)
            .data(user_id)
            .finish();
        
        // Execute GraphQL request
        let query = format!(r#"
            mutation {{
                acceptClaim(claimId: "{}") {{
                    id
                    status
                }}
            }}
        "#, claim_id);
        
        let response = schema.execute(Request::new(query)).await;
        
        // Assert response
        assert!(response.errors.is_empty());
        assert!(response.data.get("acceptClaim").is_some());
    }
    
    #[tokio::test]
    async fn test_reject_claim_success() {
        // Setup
        let skill_service = TestSkillExchangeService { should_fail: false };
        let user_id = Uuid::new_v4();
        let claim_id = Uuid::new_v4();
        
        // Create schema with our test service
        let schema = Schema::build(crate::graphql::skill_exchange::SkillExchangeQuery, crate::graphql::skill_exchange::SkillExchangeMutation, EmptySubscription)
            .data(Box::new(skill_service) as Box<dyn skill_exchange_core::services::SkillExchangeService>)
            .data(user_id)
            .finish();
        
        // Execute GraphQL request
        let query = format!(r#"
            mutation {{
                rejectClaim(claimId: "{}") {{
                    id
                    status
                }}
            }}
        "#, claim_id);
        
        let response = schema.execute(Request::new(query)).await;
        
        // Assert response
        assert!(response.errors.is_empty());
        assert!(response.data.get("rejectClaim").is_some());
    }
    
    #[tokio::test]
    async fn test_complete_skill_exchange_success() {
        // Setup
        let skill_service = TestSkillExchangeService { should_fail: false };
        let user_id = Uuid::new_v4();
        let claim_id = Uuid::new_v4();
        
        // Create schema with our test service
        let schema = Schema::build(crate::graphql::skill_exchange::SkillExchangeQuery, crate::graphql::skill_exchange::SkillExchangeMutation, EmptySubscription)
            .data(Box::new(skill_service) as Box<dyn skill_exchange_core::services::SkillExchangeService>)
            .data(user_id)
            .finish();
        
        // Execute GraphQL request
        let query = format!(r#"
            mutation {{
                completeSkillExchange(claimId: "{}", input: {{rating: 5, review: "Great exchange!"}}) {{
                    id
                    ratingValue
                }}
            }}
        "#, claim_id);
        
        let response = schema.execute(Request::new(query)).await;
        
        // Assert response
        assert!(response.errors.is_empty());
        assert!(response.data.get("completeSkillExchange").is_some());
    }
    
    #[tokio::test]
    async fn test_search_skill_listings_success() {
        // Setup
        let skill_service = TestSkillExchangeService { should_fail: false };
        let user_id = Uuid::new_v4();
        
        // Create schema with our test service
        let schema = Schema::build(crate::graphql::skill_exchange::SkillExchangeQuery, crate::graphql::skill_exchange::SkillExchangeMutation, EmptySubscription)
            .data(Box::new(skill_service) as Box<dyn skill_exchange_core::services::SkillExchangeService>)
            .data(user_id)
            .finish();
        
        // Execute GraphQL request
        let query = r#"
            query {
                searchSkillListings {
                    id
                    title
                    description
                }
            }
        "#;
        
        let response = schema.execute(Request::new(query)).await;
        
        // Assert response
        assert!(response.errors.is_empty());
        assert!(response.data.get("searchSkillListings").is_some());
    }
    
    #[tokio::test]
    async fn test_my_skill_listings_success() {
        // Setup
        let skill_service = TestSkillExchangeService { should_fail: false };
        let user_id = Uuid::new_v4();
        
        // Create schema with our test service
        let schema = Schema::build(crate::graphql::skill_exchange::SkillExchangeQuery, crate::graphql::skill_exchange::SkillExchangeMutation, EmptySubscription)
            .data(Box::new(skill_service) as Box<dyn skill_exchange_core::services::SkillExchangeService>)
            .data(user_id)
            .finish();
        
        // Execute GraphQL request
        let query = r#"
            query {
                mySkillListings {
                    id
                    title
                    description
                }
            }
        "#;
        
        let response = schema.execute(Request::new(query)).await;
        
        // Assert response
        assert!(response.errors.is_empty());
        assert!(response.data.get("mySkillListings").is_some());
    }
    
    #[tokio::test]
    async fn test_my_skill_claims_success() {
        // Setup
        let skill_service = TestSkillExchangeService { should_fail: false };
        let user_id = Uuid::new_v4();
        
        // Create schema with our test service
        let schema = Schema::build(crate::graphql::skill_exchange::SkillExchangeQuery, crate::graphql::skill_exchange::SkillExchangeMutation, EmptySubscription)
            .data(Box::new(skill_service) as Box<dyn skill_exchange_core::services::SkillExchangeService>)
            .data(user_id)
            .finish();
        
        // Execute GraphQL request
        let query = r#"
            query {
                mySkillClaims {
                    id
                    status
                }
            }
        "#;
        
        let response = schema.execute(Request::new(query)).await;
        
        // Assert response
        assert!(response.errors.is_empty());
        assert!(response.data.get("mySkillClaims").is_some());
    }
}