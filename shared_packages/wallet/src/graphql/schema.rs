use async_graphql::{Schema, EmptyMutation};
use super::{types::*, subscriptions::SubscriptionRoot};

pub type WalletSchema = Schema<(), EmptyMutation, SubscriptionRoot>;

pub fn create_schema() -> WalletSchema {
    Schema::build((), EmptyMutation, SubscriptionRoot).finish()
}