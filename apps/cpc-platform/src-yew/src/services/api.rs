use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use crate::types::error::ApiError;
use crate::types::product::ProductResponse;
use crate::graphql::product::ProductByBarcode;
use tracing::info;

pub struct ApiService;

impl ApiService {
    pub async fn query<Q: GraphQLQuery>(
        variables: Q::Variables,
    ) -> Result<Q::ResponseData, ApiError> {
        let request_body = Q::build_query(variables);
        let client = Client::new();
        let res = client
            .post("http://localhost:8080/graphql")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        let response_body: Response<Q::ResponseData> = res
            .json()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))?;
            
        if let Some(errors) = response_body.errors {
            return Err(ApiError::GraphQLError(
                errors.into_iter().map(|e| e.message).collect()
            ));
        }
        
        response_body.data
            .ok_or_else(|| ApiError::EmptyResponse)
    }
}

impl ApiService {
    pub async fn fetch_product_by_barcode(barcode: &str) -> Result<ProductResponse, ApiError> {
        let variables = product_by_barcode::Variables {
            barcode: barcode.to_string(),
        };
        let response = Self::query::<ProductByBarcode>(variables).await?;
        Ok(response.product_by_barcode)
    }
}