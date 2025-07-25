use yewdux::prelude::*;
use crate::graphql::product::ProductByBarcode;
use crate::services::api::ApiService;
use crate::types::product::{ProductResponse, BarcodeError, BarcodeErrorCode};

#[derive(Default, PartialEq, Clone)]
pub struct ProductState {
    pub current_product: Option<ProductResponse>,
    pub loading: bool,
    pub error: Option<BarcodeError>,
}

pub enum ProductAction {
    ScanBarcode(String),
    ScanSuccess(ProductResponse),
    ScanError(BarcodeError),
    Reset,
}

impl Reducer<ProductState> for ProductAction {
    fn apply(self, state: ProductState) -> ProductState {
        match self {
            ProductAction::ScanBarcode(_) => ProductState {
                loading: true,
                ..state
            },
            ProductAction::ScanSuccess(product) => ProductState {
                current_product: Some(product),
                loading: false,
                error: None,
            },
            ProductAction::ScanError(error) => ProductState {
                error: Some(error),
                loading: false,
                current_product: None,
            },
            ProductAction::Reset => ProductState::default(),
        }
    }
}

pub fn scan_barcode(barcode: String, dispatch: Dispatch<ProductStore>) {
    dispatch.apply(ProductAction::ScanBarcode(barcode.clone()));
    
    wasm_bindgen_futures::spawn_local(async move {
        let response = ApiService::query::<ProductByBarcode>(product_by_barcode::Variables {
            barcode,
        })
        .await;
        
        match response {
            Ok(data) => dispatch.apply(ProductAction::ScanSuccess(data.product_by_barcode)),
            Err(e) => dispatch.apply(ProductAction::ScanError(BarcodeError {
                code: BarcodeErrorCode::NetworkError,
                message: e.to_string(),
            })),
        }
    });
}

pub type ProductStore = Store<ProductState, ProductAction>;