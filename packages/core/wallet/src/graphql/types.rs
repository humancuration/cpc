use async_graphql::*;
use chrono::{DateTime, Utc};
use crate::domain::primitives::Money;

#[derive(SimpleObject)]
pub struct TipNotification {
    pub transaction_id: ID,
    pub sender_id: ID,
    pub amount: MoneyGQL,
    pub timestamp: DateTime<Utc>,
    pub note: Option<String>,
}

#[derive(SimpleObject)]
pub struct MoneyGQL {
    pub amount: f64,
    pub currency: String,
}

impl From<Money> for MoneyGQL {
    fn from(money: Money) -> Self {
        Self {
            amount: money.amount.to_f64().unwrap_or(0.0),
            currency: money.currency.code().to_string(),
        }
    }
}