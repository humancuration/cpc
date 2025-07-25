use graphql_client::GraphQLQuery;

type ObjectID = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/queries/get_expenses.graphql",
    response_derives = "Debug, Clone, PartialEq"
)]
pub struct GetExpenses;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/queries/get_expense.graphql",
    response_derives = "Debug, Clone, PartialEq"
)]
pub struct GetExpense;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/mutations/create_expense.graphql",
    response_derives = "Debug, Clone, PartialEq"
)]
pub struct CreateExpense;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/mutations/update_expense.graphql",
    response_derives = "Debug"
)]
pub struct UpdateExpenseMutation;