use async_graphql::*;
use crate::graphql::media_mutations::MediaMutations;
use crate::graphql::media_subscriptions::MediaSubscriptions;
use crate::bi::graphql::{BIQuery, BIMutation, BISubscription};
use crate::graphql::financial_forecasting::{FinancialForecastingMutation, FinancialForecastingQuery};
use crate::graphql::calendar::{CalendarMutation, CalendarQuery, CalendarSubscription};
use crate::graphql::user_testing::Mutation as UserTestingMutation;
use crate::graphql::asset_browser::{AssetBrowserQuery, AssetBrowserMutation, AssetBrowserSubscription};

#[derive(MergedObject, Default)]
pub struct RootQuery(BIQuery, FinancialForecastingQuery, CalendarQuery, AssetBrowserQuery);

#[derive(MergedObject, Default)]
pub struct RootMutation(
    MediaMutations,
    BIMutation,
    FinancialForecastingMutation,
    CalendarMutation,
    UserTestingMutation,
    AssetBrowserMutation,
);

#[derive(MergedSubscription, Default)]
pub struct RootSubscription(MediaSubscriptions, BISubscription, CalendarSubscription, AssetBrowserSubscription);

pub type Schema = async_graphql::Schema<RootQuery, RootMutation, RootSubscription>;