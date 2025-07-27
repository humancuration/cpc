//! GraphQL module for the Personal Finance API
//!
//! Provides GraphQL schema, queries, mutations, and subscriptions
//! for managing personal finances through a unified API interface.

pub mod schema;

pub use schema::{
    QueryRoot, MutationRoot, SubscriptionRoot,
    Expense, Budget, SavingsGoal, Receipt, ReceiptItem,
    MonthlyTrend,
};