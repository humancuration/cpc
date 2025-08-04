//! GraphQL implementations for the Messenger application

use async_graphql::{async_trait, Context, Extension, ExtensionFactory, Object, Result, SimpleObject, InputObject, Subscription};
use async_stream::stream;
use futures_util::stream::Stream;
use std::sync::Arc;
use uuid::Uuid;
use crate::models::{Reaction, Message};
use crate::services::MessageService;
use crate::auth::AuthService;

/// GraphQL representation of a Reaction
#[derive(SimpleObject)]
pub struct ReactionObject {
    id: Uuid,
    message_id: Uuid,
    user_id: Uuid,
    reaction_type: String,
    created_at: String, // ISO 8601 format
}

impl From<Reaction> for ReactionObject {
    fn from(reaction: Reaction) -> Self {
        Self {
            id: reaction.id,
            message_id: reaction.message_id,
            user_id: reaction.user_id,
            reaction_type: reaction.reaction_type,
            created_at: reaction.created_at.to_rfc3339(),
        }
    }
}

/// GraphQL representation of a Message
#[derive(SimpleObject)]
pub struct MessageObject {
    id: Uuid,
    conversation_id: Uuid,
    sender_id: Uuid,
    content: String,
    sent_at: String,
    updated_at: Option<String>,
}

impl From<Message> for MessageObject {
    fn from(message: Message) -> Self {
        Self {
            id: message.id,
            conversation_id: message.conversation_id,
            sender_id: message.sender_id,
            content: match message.content {
                crate::models::MessageContent::Text(text) => text,
                crate::models::MessageContent::Media(_) => "[Media]".to_string(),
                crate::models::MessageContent::System(_) => "[System]".to_string(),
            },
            sent_at: message.sent_at.to_rfc3339(),
            updated_at: message.updated_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

/// Input for updating a message
#[derive(InputObject)]
pub struct UpdateMessageInput {
    pub message_id: Uuid,
    pub content: String,
}
/// Input for adding a reaction
#[derive(InputObject)]
pub struct AddReactionInput {
    pub message_id: Uuid,
    pub reaction_type: String,
}

/// Input for removing a reaction
#[derive(InputObject)]
pub struct RemoveReactionInput {
    pub message_id: Uuid,
    pub reaction_type: String,
}

/// GraphQL mutations for messenger features
pub struct Mutation;

#[Object]
impl Mutation {
    async fn update_message(&self, ctx: &Context<'_>, input: UpdateMessageInput) -> Result<MessageObject> {
        // In a real implementation, we would:
        // 1. Get the user ID from the context
        // 2. Validate ownership of the message
        // 3. Update message content
        // 4. Broadcast update event
        
        // Get the message service from context
        let message_service = ctx.data::<std::sync::Arc<dyn crate::services::MessageService>>()?;
        
        // Get user ID from context
        let user_id = ctx.data::<Uuid>()?;
        
        // Update the message
        let updated_message = message_service
            .update_message(
                input.message_id,
                user_id,
                crate::models::MessageContent::Text(input.content)
            )
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to update message: {}", e)))?;
        
        // Convert to GraphQL object
        Ok(MessageObject::from(updated_message))
    async fn delete_message(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        // In a real implementation, we would:
        // 1. Get the user ID from the context
        // 2. Validate permissions (ownership or admin)
        // 3. Soft delete with tombstone
        // 4. Broadcast deletion event
        
        // Get the message service from context
        let message_service = ctx.data::<std::sync::Arc<dyn crate::services::MessageService>>()?;
        
        // Get user ID from context
        let user_id = ctx.data::<Uuid>()?;
        
        // Delete the message
        match message_service.delete_message(id, user_id).await {
            Ok(()) => Ok(true),
            Err(e) => Err(async_graphql::Error::new(format!("Failed to delete message: {}", e))),
        }
    }
        }
    }
    
    async fn add_reaction(&self, ctx: &Context<'_>, input: AddReactionInput) -> Result<ReactionObject> {
        // Get the reaction service from context
        let reaction_service = ctx.data::<std::sync::Arc<dyn crate::services::ReactionService>>()?;
        
        // Get user ID from context
        let user_id = ctx.data::<Uuid>()?;
        
        // Add the reaction
        let reaction = reaction_service
            .add_reaction(input.message_id, user_id, input.reaction_type)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to add reaction: {}", e)))?;
        
        // Convert to GraphQL object
        Ok(ReactionObject::from(reaction))
    async fn remove_reaction(&self, ctx: &Context<'_>, input: RemoveReactionInput) -> Result<bool> {
        // Get the reaction service from context
        let reaction_service = ctx.data::<std::sync::Arc<dyn crate::services::ReactionService>>()?;
        
        // Get user ID from context
        let user_id = ctx.data::<Uuid>()?;
        
        // Remove the reaction
        match reaction_service
            .remove_reaction(input.message_id, user_id, input.reaction_type)
            .await
        {
            Ok(()) => Ok(true),
            Err(e) => Err(async_graphql::Error::new(format!("Failed to remove reaction: {}", e))),
        }
    }
        }
    }
}

/// Authentication middleware for GraphQL
pub struct AuthMiddleware;

impl ExtensionFactory for AuthMiddleware {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(AuthExtension)
    }
}

struct AuthExtension;

#[async_trait]
impl Extension for AuthExtension {
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: async_graphql::Request,
        next: async_graphql::extensions::NextPrepareRequest<'_>,
    ) -> async_graphql::ServerResult<async_graphql::Request> {
        // Extract token from headers (simplified implementation)
        // In a real implementation, you would extract the Authorization header
        // and validate the token using the AuthService
        
        // For now, we'll just pass through the request
        next.run(ctx, request).await
    }
}

/// GraphQL subscriptions for real-time events
pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn reaction_events(
        &self,
        ctx: &Context<'_>,
        message_id: Uuid,
    ) -> impl Stream<Item = ReactionObject> {
        // In a real implementation, we would:
        // 1. Get event stream from WebSocket broker
        // 2. Filter events for the specified message_id
        
        // Placeholder implementation
        stream! {
            // This is just a placeholder - in reality, this would connect to a real event stream
            loop {
                // Simulate waiting for events
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                // This would never actually yield in a real implementation without events
                // yield ReactionObject { /* ... */ };
            }
        }
    }
}