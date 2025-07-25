use async_graphql::*;
use crate::graphql::asset_browser::{AssetBrowserQuery, AssetBrowserMutation, AssetBrowserSubscription};
use crate::graphql::impact::{ImpactQuery, ImpactMutation, ImpactSubscription};
use crate::graphql::product::{ProductQueryRoot, ProductMutationRoot, ProductSubscriptionRoot};
use crate::graphql::supply_chain::{SupplyChainQueryRoot, SupplyChainMutationRoot, SupplyChainSubscriptionRoot};
use crate::graphql::financial_forecasting::{FinancialForecastingQueryRoot, FinancialForecastingMutationRoot, FinancialForecastingSubscriptionRoot};

#[derive(MergedObject, Default)]
pub struct RootQuery(AssetBrowserQuery, ImpactQuery, ProductQueryRoot, SupplyChainQueryRoot, FinancialForecastingQueryRoot);

#[derive(MergedObject, Default)]
pub struct RootMutation(AssetBrowserMutation, ImpactMutation, ProductMutationRoot, SupplyChainMutationRoot, FinancialForecastingMutationRoot);

#[derive(MergedSubscription, Default)]
pub struct RootSubscription(AssetBrowserSubscription, ImpactSubscription, ProductSubscriptionRoot, SupplyChainSubscriptionRoot, FinancialForecastingSubscriptionRoot);

pub type Schema = async_graphql::Schema<RootQuery, RootMutation, RootSubscription>;