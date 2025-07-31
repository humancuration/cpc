//! Integration tests for the GraphQL API

use super::{
    create_schema,
    SocialIntegrationSchema,
    types::{MoneyInput, Currency},
};
// use crate::{
//     application::tip_service::TipService, // DEPRECATED - moved to wallet package
//     infrastructure::repositories::{PostgresTipTransactionRepository, TipTransactionRepository}, // DEPRECATED - moved to wallet package
// };
use async_graphql::{http::GraphiQLSource, Schema, EmptySubscription};
use std::sync::Arc;
use uuid::Uuid;

// Helper function to create a test schema
fn create_test_schema() -> SocialIntegrationSchema {
    // In a real test, we would use mock services
    // For now, we'll create a schema with dummy services
    // NOTE: The tip functionality has been moved to the wallet package
    // let tip_service = Arc::new(TipService::new(
    //     Box::new(MockWalletService {}),
    //     Box::new(MockTipTransactionRepository {}),
    // ));
    
    // create_schema(tip_service)
    create_schema() // Create schema without tip service
}

// Mock wallet service for testing
struct MockWalletService;

#[async_trait::async_trait]
impl cpc_wallet::application::wallet_service::WalletService for MockWalletService {
    async fn transfer_dabloons(
        &self,
        _from_user_id: Uuid,
        _to_user_id: Uuid,
        _amount: cpc_wallet::domain::primitives::Money,
        _description: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn get_balance(
        &self,
        _user_id: Uuid,
    ) -> Result<cpc_wallet::domain::primitives::Money, Box<dyn std::error::Error + Send + Sync>> {
        Ok(cpc_wallet::domain::primitives::Money {
            amount: rust_decimal::Decimal::from(100),
            currency: cpc_wallet::domain::primitives::Currency::DAB,
        })
    }
// Mock tip transaction repository for testing
// struct MockTipTransactionRepository;
//
// #[async_trait::async_trait]
// impl TipTransactionRepository for MockTipTransactionRepository {
//     async fn record_transaction(
//         &self,
//         _sender_id: Uuid,
//         _recipient_id: Uuid,
//         _amount: cpc_wallet::domain::primitives::Money,
//         _transaction_type: String,
//         _description: String,
//     ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         Ok(())
//     }
//
//     async fn get_transactions_for_user(
//         &self,
//         _user_id: Uuid,
//         _limit: i64,
//         _offset: i64,
//     ) -> Result<Vec<crate::domain::tip_transaction::TipTransaction>, Box<dyn std::error::Error + Send + Sync>> {
//         Ok(vec![])
//     }
// }
    }
}

// #[tokio::test]
// async fn test_send_tip_mutation() {
//     let schema = create_test_schema();
//
//     let recipient_id = Uuid::new_v4();
//     let query = format!(
//         r#"
//         mutation {{
//             sendTip(
//                 recipientId: "{}",
//                 amount: {{
//                     amount: 10.0,
//                     currency: DAB
//                 }},
//                 note: "Thanks for your help!"
//             ) {{
//                 id
//                 senderId
//                 recipientId
//                 amount {{
//                     amount
//                     currency
//                 }}
//                 note
//                 createdAt
//             }}
//         }}
//         "#,
//         recipient_id
//     );
//
//     // In a real test, we would set up the context with an authenticated user
//     // For now, we'll just execute the query and check for errors
//     let response = schema.execute(query).await;
//     // We expect this to fail because we haven't set up the context with an authenticated user
//     assert!(!response.errors.is_empty());
// }

// #[tokio::test]
// async fn test_tip_transactions_by_user_query() {
//     let schema = create_test_schema();
//
//     let user_id = Uuid::new_v4();
//     let query = format!(
//         r#"
//         query {{
//             tipTransactionsByUser(
//                 userId: "{}",
//                 first: 10
//             ) {{
//                 edges {{
//                     node {{
//                         id
//                         senderId
//                         recipientId
//                         amount {{
//                             amount
//                             currency
//                         }}
//                         note
//                         createdAt
//                     }}
//                     cursor
//                 }}
//                 pageInfo {{
//                     hasNextPage
//                     endCursor
//                 }}
//             }}
//         }}
//         "#,
//         user_id
//     );
//
//     // In a real test, we would set up the context with an authenticated user
//     // For now, we'll just execute the query and check for errors
//     let response = schema.execute(query).await;
//     // We expect this to fail because we haven't set up the context with an authenticated user
//     assert!(!response.errors.is_empty());
// }

// #[tokio::test]
// async fn test_invalid_tip_amount() {
//     let schema = create_test_schema();
//
//     let recipient_id = Uuid::new_v4();
//     let query = format!(
//         r#"
//         mutation {{
//             sendTip(
//                 recipientId: "{}",
//                 amount: {{
//                     amount: -5.0,
//                     currency: DAB
//                 }},
//                 note: "Invalid tip"
//             ) {{
//                 id
//             }}
//         }}
//         "#,
//         recipient_id
//     );
//
//     let response = schema.execute(query).await;
//     // We expect this to fail due to invalid amount
//     assert!(!response.errors.is_empty());
// }

// #[tokio::test]
// async fn test_note_length_validation() {
//     let schema = create_test_schema();
//
//     let recipient_id = Uuid::new_v4();
//     let long_note = "a".repeat(201); // Exceeds 200 character limit
//     let query = format!(
//         r#"
//         mutation {{
//             sendTip(
//                 recipientId: "{}",
//                 amount: {{
//                     amount: 10.0,
//                     currency: DAB
//                 }},
//                 note: "{}"
//             ) {{
//                 id
//             }}
//         }}
//         "#,
//         recipient_id,
//         long_note
//     );
//
//     let response = schema.execute(query).await;
//     // We expect this to fail due to note being too long
//     assert!(!response.errors.is_empty());
// }