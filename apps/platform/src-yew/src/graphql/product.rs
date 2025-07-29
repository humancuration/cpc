use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/queries/product_by_barcode.graphql",
    response_derives = "Debug, Clone"
)]
pub struct ProductByBarcode;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/queries/product_by_id.graphql",
    response_derives = "Debug, Clone"
)]
pub struct ProductById;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/queries/supply_chain_by_product.graphql",
    response_derives = "Debug, Clone"
)]
pub struct SupplyChainByProduct;