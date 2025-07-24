use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::types::invoice::{Contact, InvoiceTemplate};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/invoicing.graphql",
    response_derives = "Debug, Clone, Serialize, Deserialize"
)]
pub struct GetInvoices;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/invoicing.graphql",
    response_derives = "Debug, Clone, Serialize, Deserialize"
)]
pub struct CreateInvoice;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/invoicing.graphql",
    response_derives = "Debug, Clone, Serialize, Deserialize"
)]
pub struct UpdateInvoice;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/invoicing.graphql",
    response_derives = "Debug, Clone, Serialize, Deserialize"
)]
pub struct DeleteInvoice;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/invoicing.graphql",
    response_derives = "Debug, Clone, Serialize, Deserialize"
)]
pub struct GetInvoiceTemplates;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/invoicing.graphql",
    response_derives = "Debug, Clone, Serialize, Deserialize"
)]
pub struct CreateInvoiceTemplate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/invoicing.graphql",
    response_derives = "Debug, Clone, Serialize, Deserialize"
)]
pub struct GetAgingReport;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/invoicing.graphql",
    response_derives = "Debug, Clone, Serialize, Deserialize"
)]
pub struct GetSupplierPerformance;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceFilters {
    pub status: Option<String>,
    pub client_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceSummary {
    pub total_invoices: i64,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub pending_amount: f64,
    pub overdue_amount: f64,
}

pub struct InvoicingService {
    client: reqwest::Client,
    endpoint: String,
}

impl InvoicingService {
    pub fn new(endpoint: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint,
        }
    }

    pub async fn get_invoices(&self, filters: InvoiceFilters) -> Result<Vec<GetInvoices::GetInvoicesInvoices>, Box<dyn std::error::Error>> {
        let variables = get_invoices::Variables {
            status: filters.status,
            client_id: filters.client_id,
            date_from: filters.date_from,
            date_to: filters.date_to,
            min_amount: filters.min_amount,
            max_amount: filters.max_amount,
        };

        let response = self.client
            .post(&self.endpoint)
            .json(&GetInvoices::build_query(variables))
            .send()
            .await?
            .json::<graphql_client::Response<get_invoices::ResponseData>>()
            .await?;

        response.data
            .map(|d| d.invoices)
            .unwrap_or_default()
            .ok_or_else(|| "No invoices found".into())
    }

    pub async fn create_invoice(&self, input: CreateInvoice::CreateInvoiceInput) -> Result<CreateInvoice::CreateInvoiceCreateInvoice, Box<dyn std::error::Error>> {
        let variables = create_invoice::Variables { input };
        
        let response = self.client
            .post(&self.endpoint)
            .json(&CreateInvoice::build_query(variables))
            .send()
            .await?
            .json::<graphql_client::Response<create_invoice::ResponseData>>()
            .await?;

        response.data
            .and_then(|d| d.create_invoice)
            .ok_or_else(|| "Failed to create invoice".into())
    }

    pub async fn update_invoice(&self, id: String, input: UpdateInvoice::UpdateInvoiceInput) -> Result<UpdateInvoice::UpdateInvoiceUpdateInvoice, Box<dyn std::error::Error>> {
        let variables = update_invoice::Variables { id, input };
        
        let response = self.client
            .post(&self.endpoint)
            .json(&UpdateInvoice::build_query(variables))
            .send()
            .await?
            .json::<graphql_client::Response<update_invoice::ResponseData>>()
            .await?;

        response.data
            .and_then(|d| d.update_invoice)
            .ok_or_else(|| "Failed to update invoice".into())
    }

    pub async fn delete_invoice(&self, id: String) -> Result<bool, Box<dyn std::error::Error>> {
        let variables = delete_invoice::Variables { id };
        
        let response = self.client
            .post(&self.endpoint)
            .json(&DeleteInvoice::build_query(variables))
            .send()
            .await?
            .json::<graphql_client::Response<delete_invoice::ResponseData>>()
            .await?;

        Ok(response.data
            .and_then(|d| d.delete_invoice)
            .unwrap_or(false))
    }

    pub async fn get_templates(&self) -> Result<Vec<GetInvoiceTemplates::GetInvoiceTemplatesTemplates>, Box<dyn std::error::Error>> {
        let response = self.client
            .post(&self.endpoint)
            .json(&GetInvoiceTemplates::build_query(()))
            .send()
            .await?
            .json::<graphql_client::Response<get_invoice_templates::ResponseData>>()
            .await?;

        response.data
            .map(|d| d.templates)
            .unwrap_or_default()
            .ok_or_else(|| "No templates found".into())
    }

    pub async fn get_aging_report(&self, days_past_due: i32) -> Result<Vec<GetAgingReport::GetAgingReportAgingReport>, Box<dyn std::error::Error>> {
        let variables = get_aging_report::Variables { days_past_due };
        
        let response = self.client
            .post(&self.endpoint)
            .json(&GetAgingReport::build_query(variables))
            .send()
            .await?
            .json::<graphql_client::Response<get_aging_report::ResponseData>>()
            .await?;

        response.data
            .map(|d| d.aging_report)
            .unwrap_or_default()
            .ok_or_else(|| "No aging report found".into())
    }

    pub async fn get_supplier_performance(&self, supplier_id: Option<String>) -> Result<Vec<GetSupplierPerformance::GetSupplierPerformanceSupplierPerformance>, Box<dyn std::error::Error>> {
        let variables = get_supplier_performance::Variables { supplier_id };
        
        let response = self.client
            .post(&self.endpoint)
            .json(&GetSupplierPerformance::build_query(variables))
            .send()
            .await?
            .json::<graphql_client::Response<get_supplier_performance::ResponseData>>()
            .await?;

        response.data
            .map(|d| d.supplier_performance)
            .unwrap_or_default()
            .ok_or_else(|| "No supplier performance data found".into())
    }

    pub async fn get_templates(&self) -> Result<Vec<InvoiceTemplate>, Box<dyn std::error::Error>> {
        let response = self.client
            .post(&self.endpoint)
            .json(&GetInvoiceTemplates::build_query(()))
            .send()
            .await?
            .json::<graphql_client::Response<get_invoice_templates::ResponseData>>()
            .await?;

        let templates = response.data
            .map(|d| d.templates)
            .unwrap_or_default()
            .unwrap_or_default();

        Ok(templates.into_iter().map(|t| InvoiceTemplate {
            id: t.id,
            name: t.name,
            description: t.description,
            default_layout: t.default_layout,
            custom_fields: t.custom_fields,
        }).collect())
    }

    pub async fn get_contacts(&self) -> Result<Vec<Contact>, Box<dyn std::error::Error>> {
        let response = self.client
            .get(format!("{}/api/contacts", self.endpoint))
            .send()
            .await?
            .json::<Vec<Contact>>()
            .await?;

        Ok(response)
    }

    pub async fn create_invoice_mutation(&self, invoice: crate::types::invoice::Invoice) -> Result<CreateInvoice::CreateInvoiceCreateInvoice, Box<dyn std::error::Error>> {
        let items: Vec<CreateInvoice::CreateInvoiceInputItems> = invoice.items
            .iter()
            .map(|item| CreateInvoice::CreateInvoiceInputItems {
                description: item.description.clone(),
                quantity: item.quantity,
                unit_price: item.unit_price,
            })
            .collect();

        let input = CreateInvoice::CreateInvoiceInput {
            recipient_id: invoice.recipient_id,
            items,
            due_date: invoice.due_date.to_rfc3339(),
            template_id: invoice.template_id,
            notes: invoice.notes,
            tax_rate: invoice.tax_rate,
            discount: invoice.discount,
        };

        self.create_invoice(input).await
    }
}