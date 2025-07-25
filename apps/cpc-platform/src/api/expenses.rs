use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/expenses.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize, Deserialize"
)]
pub struct GetExpenses;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/expenses.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize, Deserialize"
)]
pub struct CreateExpense;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/expenses.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize, Deserialize"
)]
pub struct GetExpense;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/expenses.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize, Deserialize"
)]
pub struct DeleteExpense;