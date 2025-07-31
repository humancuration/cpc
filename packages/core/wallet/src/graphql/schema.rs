use async_graphql::{Schema, Subscription, ID, Context};
use async_stream::stream;
use futures_util::stream::Stream;
use tokio_stream::StreamExt;
use crate::{application::wallet_service::WalletService, domain::wallet::TipSentEvent};
use super::types::TipNotification;

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn tip_received(
        &self,
        ctx: &Context<'_>,
        recipient_id: ID,
    ) -> impl Stream<Item = TipNotification> {
        // Get the wallet service from the context
        let wallet_service = ctx.data::<WalletService>().expect("WalletService not found in context");
        
        // Subscribe to tip events
        let mut receiver = wallet_service.subscribe_tip_events();
        
        // Convert the recipient_id to Uuid for comparison
        let recipient_uuid = uuid::Uuid::parse_str(recipient_id.as_str()).unwrap_or_default();
        
        // Create a stream that filters events for this recipient
        stream! {
            while let Ok(event) = receiver.recv().await {
                if event.recipient_id == recipient_uuid {
                    yield TipNotification {
                        transaction_id: ID::from(event.transaction_id.to_string()),
                        sender_id: ID::from(event.sender_id.to_string()),
                        amount: event.amount.into(),
                        timestamp: event.timestamp,
                        note: event.note,
                    };
                }
            }
        }
    }
}

pub type WalletSchema = Schema<(), (), SubscriptionRoot>;

pub fn create_schema() -> WalletSchema {
    Schema::build((), (), SubscriptionRoot).finish()
}