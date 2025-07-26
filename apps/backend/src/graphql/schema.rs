use async_graphql::*;
use crate::graphql::asset_browser::{AssetBrowserQuery, AssetBrowserMutation, AssetBrowserSubscription};
use crate::graphql::impact::{ImpactQuery, ImpactMutation, ImpactSubscription};
use crate::graphql::product::{ProductQueryRoot, ProductMutationRoot, ProductSubscriptionRoot};
use crate::graphql::supply_chain::{SupplyChainQueryRoot, SupplyChainMutationRoot, SupplyChainSubscriptionRoot};
use crate::graphql::financial_forecasting::{FinancialForecastingQueryRoot, FinancialForecastingMutationRoot, FinancialForecastingSubscriptionRoot};
use crate::graphql::expenses::{ExpensesQueryRoot, ExpensesMutationRoot, ExpensesSubscriptionRoot};
use crate::graphql::community::{CommunityQuery, CommunityMutation, CommunitySubscription};
use crate::invoicing::graphql::{InvoicingQuery, InvoicingMutation, InvoicingSubscription};
use crate::graphql::project::{ProjectQuery, ProjectMutation};
use crate::graphql::feature_flags::FeatureFlagsQuery;
use crate::graphql::finance::{FinanceQuery, FinanceMutation, FinanceSubscription};
use crate::config::UiThresholds;

// New GraphQL modules for android-rust-migration
use crate::graphql::user_management::{UserQuery, UserMutation, UserSubscription};
use crate::graphql::social_interactions::{SocialQuery, SocialMutation, SocialSubscription};
use crate::graphql::forum_system::{ForumQuery, ForumMutation, ForumSubscription};
use crate::graphql::governance_system::{GovernanceQuery, GovernanceMutation, GovernanceSubscription};

#[derive(MergedObject, Default)]
pub struct RootQuery(
    AssetBrowserQuery,
    ImpactQuery,
    ProductQueryRoot,
    SupplyChainQueryRoot,
    FinancialForecastingQueryRoot,
    ExpensesQueryRoot,
    CommunityQuery,
    InvoicingQuery,
    ProjectQuery,
    FeatureFlagsQuery, // Add feature flags query
    FinanceQuery,
    // New queries for android-rust-migration
    UserQuery,
    SocialQuery,
    ForumQuery,
    GovernanceQuery
);

#[derive(MergedObject, Default)]
pub struct RootMutation(
    AssetBrowserMutation,
    ImpactMutation,
    ProductMutationRoot,
    SupplyChainMutationRoot,
    FinancialForecastingMutationRoot,
    ExpensesMutationRoot,
    CommunityMutation,
    InvoicingMutation,
    ProjectMutation,
    FinanceMutation,
    // New mutations for android-rust-migration
    UserMutation,
    SocialMutation,
    ForumMutation,
    GovernanceMutation
);

#[derive(MergedSubscription, Default)]
pub struct RootSubscription(
    AssetBrowserSubscription,
    ImpactSubscription,
    ProductSubscriptionRoot,
    SupplyChainSubscriptionRoot,
    FinancialForecastingSubscriptionRoot,
    ExpensesSubscriptionRoot,
    CommunitySubscription,
    InvoicingSubscription,
    FinanceSubscription,
    // New subscriptions for android-rust-migration
    UserSubscription,
    SocialSubscription,
    ForumSubscription,
    GovernanceSubscription
);

pub struct GraphQLContext {
    pub ui_thresholds: UiThresholds,
}

impl From<&crate::config::Config> for GraphQLContext {
    fn from(config: &crate::config::Config) -> Self {
        Self {
            ui_thresholds: config.ui_thresholds.clone(),
        }
    }
}

pub type Schema = async_graphql::Schema<
    RootQuery,
    RootMutation,
    RootSubscription,
    async_graphql::EmptySubscription,
    GraphQLContext
>;