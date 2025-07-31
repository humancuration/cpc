//! Basic usage example for the social integration crate

use social_integration::domain::post::{UnifiedPost, AppSource, PostMetadata, EngagementMetrics, PrivacySettings};
// use social_integration::domain::tip_transaction::TipTransaction; // DEPRECATED - moved to wallet package
// use social_integration::application::tip_service::TipService; // DEPRECATED - moved to wallet package
use social_integration::infrastructure::repositories::InMemoryUnifiedPostRepository;
use cpc_wallet::domain::primitives::{Money, Currency};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Social Integration Crate - Basic Usage Example");
    
    // Create a unified post
    let post_id = Uuid::new_v4();
    let author_id = Uuid::new_v4();
    let content = "Hello, CPC social network!".to_string();
    
    let metadata = PostMetadata {
        created_at: Utc::now(),
        updated_at: Utc::now(),
        engagement: EngagementMetrics::new(),
        media_attachments: Vec::new(),
        hashtags: vec!["hello".to_string(), "cpc".to_string()],
        privacy: PrivacySettings {
            is_public: true,
            allowed_viewers: Vec::new(),
            shareable: true,
        },
    };
    
    let post = UnifiedPost::new(
        AppSource::Yapper,
        post_id,
        author_id,
        content,
        metadata,
    );
    
    println!("Created unified post: {}", post.id);
    println!("Post content: {}", post.content);
    println!("Post source: {:?}", post.source);
    
    // Create a repository and save the post
    let repo = InMemoryUnifiedPostRepository::new();
    repo.save(&post).await?;
    
    // Find the post by ID
    let found_post = repo.find_by_id(post.id).await?;
    if let Some(found) = found_post {
        println!("Found post by ID: {}", found.id);
    }
    
    // Create a tip service (with a mock wallet service)
    // NOTE: The tip functionality has been moved to the wallet package
    // let tip_service = TipService::new(
    //     Box::new(MockWalletService::new()),
    //     Box::new(MockTipTransactionRepository::new()),
    // );
    
    // Send a tip from one user to another
    // let sender_id = Uuid::new_v4();
    // let recipient_id = author_id; // Tip the post author
    // let amount = Money::new(dec!(5), Currency::Dabloons);
    // let note = Some("Great post!".to_string());
    
    // tip_service.send_tip(sender_id, recipient_id, amount, note).await?;
    // println!("Sent tip from user {} to user {}", sender_id, recipient_id);
    
    Ok(())
}
// #[async_trait::async_trait]
// impl social_integration::infrastructure::repositories::TipTransactionRepository for MockTipTransactionRepository {
//     async fn record_transaction(
//         &self,
//         sender_id: uuid::Uuid,
//         recipient_id: uuid::Uuid,
//         amount: cpc_wallet::domain::primitives::Money,
//         transaction_type: String,
//         description: String
//     ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         println!("Recording tip transaction: {} dabloons from user {} to user {} (type: {}, description: {})",
//                  amount.amount, sender_id, recipient_id, transaction_type, description);
//         Ok(())
//     }
// }
//
// /// Mock wallet service for demonstration
// struct MockWalletService;
//
// /// Mock tip transaction repository for demonstration
// struct MockTipTransactionRepository;
//
// impl MockWalletService {
//     fn new() -> Self {
//         Self
//     }
// }
//
// impl MockTipTransactionRepository {
//     fn new() -> Self {
//         Self
//     }
// }
}

#[async_trait::async_trait]
impl cpc_wallet::application::wallet_service::WalletService for MockWalletService {
    async fn get_or_create_wallet(&self, user_id: uuid::Uuid) -> Result<cpc_wallet::domain::wallet::Wallet, cpc_wallet::domain::primitives::FinancialError> {
        Ok(cpc_wallet::domain::wallet::Wallet::new(user_id))
    }
    
    async fn add_dabloons(&self, user_id: uuid::Uuid, amount: cpc_wallet::domain::primitives::Money, description: Option<String>) -> Result<cpc_wallet::domain::wallet::Wallet, cpc_wallet::domain::primitives::FinancialError> {
        println!("Adding {} dabloons to user {} (description: {:?})", amount.amount, user_id, description);
        Ok(cpc_wallet::domain::wallet::Wallet::new(user_id))
    }
    
    async fn subtract_dabloons(&self, user_id: uuid::Uuid, amount: cpc_wallet::domain::primitives::Money, description: Option<String>) -> Result<cpc_wallet::domain::wallet::Wallet, cpc_wallet::domain::primitives::FinancialError> {
        println!("Subtracting {} dabloons from user {} (description: {:?})", amount.amount, user_id, description);
        Ok(cpc_wallet::domain::wallet::Wallet::new(user_id))
    }
    
    async fn transfer_dabloons(&self, from_user_id: uuid::Uuid, to_user_id: uuid::Uuid, amount: cpc_wallet::domain::primitives::Money, description: Option<String>) -> Result<(cpc_wallet::domain::wallet::Wallet, cpc_wallet::domain::wallet::Wallet), cpc_wallet::domain::primitives::FinancialError> {
        println!("Transferring {} dabloons from user {} to user {} (description: {:?})", amount.amount, from_user_id, to_user_id, description);
        Ok((cpc_wallet::domain::wallet::Wallet::new(from_user_id), cpc_wallet::domain::wallet::Wallet::new(to_user_id)))
    }
    
    async fn get_transaction_history(&self, user_id: uuid::Uuid) -> Result<Vec<cpc_wallet::domain::wallet::WalletTransaction>, cpc_wallet::domain::primitives::FinancialError> {
        Ok(vec![])
    }
    
    async fn distribute_universal_income(&self, user_id: uuid::Uuid, amount: cpc_wallet::domain::primitives::Money, distribution_date: chrono::NaiveDate) -> Result<cpc_wallet::domain::wallet::Wallet, cpc_wallet::domain::primitives::FinancialError> {
        println!("Distributing {} dabloons as Universal Income to user {} for date {}", amount.amount, user_id, distribution_date);
        Ok(cpc_wallet::domain::wallet::Wallet::new(user_id))
    }
}
    async fn add_dabloons(&self, user_id: uuid::Uuid, amount: cpc_wallet::domain::primitives::Money, description: Option<String>) -> Result<cpc_wallet::domain::wallet::Wallet, Box<dyn std::error::Error + Send + Sync>> {
        println!("Adding {} dabloons to user {} (description: {:?})", amount.amount, user_id, description);
        // In a real implementation, this would interact with a wallet repository
        Ok(cpc_wallet::domain::wallet::Wallet::new(user_id))
    }
    
    async fn subtract_dabloons(&self, user_id: uuid::Uuid, amount: cpc_wallet::domain::primitives::Money, description: Option<String>) -> Result<cpc_wallet::domain::wallet::Wallet, Box<dyn std::error::Error + Send + Sync>> {
        println!("Subtracting {} dabloons from user {} (description: {:?})", amount.amount, user_id, description);
        // In a real implementation, this would interact with a wallet repository
        Ok(cpc_wallet::domain::wallet::Wallet::new(user_id))
    }
    
    async fn transfer_dabloons(&self, from_user_id: uuid::Uuid, to_user_id: uuid::Uuid, amount: cpc_wallet::domain::primitives::Money, description: Option<String>) -> Result<(cpc_wallet::domain::wallet::Wallet, cpc_wallet::domain::wallet::Wallet), Box<dyn std::error::Error + Send + Sync>> {
        println!("Transferring {} dabloons from user {} to user {} (description: {:?})", amount.amount, from_user_id, to_user_id, description);
        // In a real implementation, this would interact with a wallet repository
        Ok((cpc_wallet::domain::wallet::Wallet::new(from_user_id), cpc_wallet::domain::wallet::Wallet::new(to_user_id)))
    }
}