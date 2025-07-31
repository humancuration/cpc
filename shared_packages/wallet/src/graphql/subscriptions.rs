//! GraphQL subscriptions for wallet features
//! 
//! This module provides real-time notifications for wallet events,
//! including tip notifications.

use async_graphql::{Subscription, Context, ID};
use async_stream::stream;
use futures_util::stream::Stream;
use uuid::Uuid;
use crate::{
    application::WalletService,
    domain::wallet::TipSentEvent,
};
use super::types::TipNotification;

/// Root subscription object for wallet features
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    /// Subscribe to tip received events
    /// 
    /// This subscription provides real-time notifications when a user receives a tip.
    /// 
    /// # Arguments
    /// * `ctx` - GraphQL context containing the wallet service
    /// * `recipient_id` - ID of the user receiving tips
    /// 
    /// # Returns
    /// Stream of TipNotification events
    /// 
    /// # Sequence Diagram
    /// Refer to docs/wallet/MONETIZATION.md for the GraphQL subscription flow.
    async fn tip_received(
        &self,
        ctx: &Context<'_>,
        recipient_id: ID,
    ) -> impl Stream<Item = TipNotification> {
        // Get the wallet service from the context
        let wallet_service = ctx.data::<Box<dyn WalletService>>()
            .expect("WalletService not found in context");
        
        // Subscribe to tip events
        let mut receiver = wallet_service.subscribe_tip_events();
        
        // Convert the recipient_id to Uuid for comparison
        let recipient_uuid = Uuid::parse_str(recipient_id.as_str()).unwrap_or_default();
        
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