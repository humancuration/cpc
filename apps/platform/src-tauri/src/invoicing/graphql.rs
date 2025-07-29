use graphql_client::{GraphQLQuery, Response};
use crate::api::client::ApiClient;
use anyhow::{Result, anyhow};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/invoicing/graphql/schema.graphql",
    query_path = "src/invoicing/graphql/queries/get_invoice_dashboard_data.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone"
)]
pub struct GetInvoiceDashboardData;

pub async fn fetch_invoice_dashboard_data(
    organization_id: String,
) -> Result<get_invoice_dashboard_data::ResponseData> {
    let client = ApiClient::new()?;
    let variables = get_invoice_dashboard_data::Variables { organization_id };
    let response = client.post::<GetInvoiceDashboardData>(variables).await?;

    if let Some(data) = response.data {
        Ok(data)
    } else {
        Err(anyhow!("No data returned from query"))
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/invoicing/graphql/schema.graphql",
    query_path = "src/invoicing/graphql/mutations/generate_invoice_pdf.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone"
)]
pub struct GenerateInvoicePdf;

pub async fn generate_invoice_pdf_mutation(invoice_id: String) -> Result<String> {
    let client = ApiClient::new()?;
    let variables = generate_invoice_pdf::Variables {
        invoice_id: invoice_id.into(),
    };
    let response: Response<generate_invoice_pdf::ResponseData> = client.post::<GenerateInvoicePdf>(variables).await?;

    if let Some(data) = response.data {
         Ok(data.generate_invoice_pdf)
    } else {
        Err(anyhow!("Mutation failed to return data"))
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/invoicing/graphql/schema.graphql",
    query_path = "src/invoicing/graphql/queries/get_invoice_details.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone"
)]
pub struct GetInvoiceDetails;

pub async fn fetch_invoice_details(
    invoice_id: String,
) -> Result<get_invoice_details::ResponseData> {
    let client = ApiClient::new()?;
    let variables = get_invoice_details::Variables { invoice_id };
    let response = client.post::<GetInvoiceDetails>(variables).await?;

    if let Some(data) = response.data {
        Ok(data)
    } else {
        Err(anyhow!("No data returned from query"))
    }
}
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/invoicing/graphql/schema.graphql",
    query_path = "src/invoicing/graphql/mutations/create_invoice.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone"
)]
pub struct CreateInvoice;

pub async fn create_invoice_mutation(
    input: create_invoice::CreateInvoiceInput,
) -> Result<create_invoice::ResponseData> {
    let client = ApiClient::new()?;
    let variables = create_invoice::Variables { input };
    let response = client.post::<CreateInvoice>(variables).await?;

    if let Some(data) = response.data {
        Ok(data)
    } else {
        Err(anyhow!("Mutation failed to return data"))
    }
}