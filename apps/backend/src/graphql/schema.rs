use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

#[derive(MergedObject, Default)]
pub struct Query(/* Add query modules */);

#[derive(MergedObject, Default)]
pub struct Mutation(/* Add mutation modules */);

pub fn build_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}