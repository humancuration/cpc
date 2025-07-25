use async_graphql::*;
use crate::graphql::asset_browser::{AssetBrowserQuery, AssetBrowserMutation, AssetBrowserSubscription};
use crate::graphql::impact::{ImpactQuery, ImpactMutation, ImpactSubscription};
use crate::graphql::product::{ProductQueryRoot, ProductMutationRoot, ProductSubscriptionRoot};
use crate::graphql::supply_chain::{SupplyChainQueryRoot, SupplyChainMutationRoot, SupplyChainSubscriptionRoot};
use crate::graphql::financial_forecasting::{FinancialForecastingQueryRoot, FinancialForecastingMutationRoot, FinancialForecastingSubscriptionRoot};
use crate::graphql::expenses::{ExpensesQueryRoot, ExpensesMutationRoot, ExpensesSubscriptionRoot};
use crate::graphql::community::{CommunityQuery, CommunityMutation, CommunitySubscription};

#[derive(MergedObject, Default)]
pub struct RootQuery(
    AssetBrowserQuery,
    ImpactQuery,
    ProductQueryRoot,
    SupplyChainQueryRoot,
    FinancialForecastingQueryRoot,
    ExpensesQueryRoot,
    CommunityQuery
);

#[derive(MergedObject, Default)]
pub struct RootMutation(
    AssetBrowserMutation,
    ImpactMutation,
    ProductMutationRoot,
    SupplyChainMutationRoot,
    FinancialForecastingMutationRoot,
    ExpensesMutationRoot,
    CommunityMutation
);

#[derive(MergedSubscription, Default)]
pub struct RootSubscription(
    AssetBrowserSubscription,
    ImpactSubscription,
    ProductSubscriptionRoot,
    SupplyChainSubscriptionRoot,
    FinancialForecastingSubscriptionRoot,
    ExpensesSubscriptionRoot,
    CommunitySubscription
);

pub type Schema = async_graphql::Schema<RootQuery, RootMutation, RootSubscription>;