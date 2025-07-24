use async_graphql::*;
use crate::graphql::media_mutations::MediaMutations;
use crate::graphql::media_subscriptions::MediaSubscriptions;
use crate::bi::graphql::{BIQuery, BIMutation, BISubscription};
use crate::graphql::financial_forecasting::{FinancialForecastingMutation, FinancialForecastingQuery};
use crate::graphql::calendar::{CalendarMutation, CalendarQuery, CalendarSubscription}; // Added CalendarSubscription
use crate::graphql::user_testing::Mutation as UserTestingMutation;

#[derive(MergedObject, Default)]
pub struct RootQuery(BIQuery, FinancialForecastingQuery, CalendarQuery);

#[derive(MergedObject, Default)]
pub struct RootMutation(
    MediaMutations,
    BIMutation,
    FinancialForecastingMutation,
    CalendarMutation,
    UserTestingMutation
);

#[derive(MergedSubscription, Default)]
pub struct RootSubscription(MediaSubscriptions, BISubscription, CalendarSubscription); // Added CalendarSubscription

pub type Schema = async_graphql::Schema<RootQuery, RootMutation, RootSubscription>;