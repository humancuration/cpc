use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};
use crate::api::client::perform_graphql_query;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/invoicing/graphql/queries/get_invoice_dashboard_data.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone, PartialEq"
)]
pub struct GetInvoiceDashboardData;

pub type InvoiceNode = get_invoice_dashboard_data::GetInvoiceDashboardDataInvoices;

pub async fn fetch_invoice_dashboard_data(
    organization_id: String,
) -> Result<get_invoice_dashboard_data::ResponseData, String> {
    let variables = get_invoice_dashboard_data::Variables { organization_id };
    let response =
        perform_graphql_query::<GetInvoiceDashboardData>(variables)
            .await?;
    Ok(response)
}