use anyhow::{Context, Result};
use tonic::{Request, Response, Status};
use tracing::{info, warn};
use cpc_core::models::product::Product;
use cpc_core::repositories::product_repository::ProductRepository;
use cpc_protos::barcode::v1::{
    barcode_service_server::BarcodeService,
    BarcodeRequest, ProductResponse, BarcodeError, BarcodeErrorCode
};

pub struct BarcodeServiceImpl<R: ProductRepository> {
    product_repository: R,
}

impl<R: ProductRepository> BarcodeServiceImpl<R> {
    pub fn new(product_repository: R) -> Self {
        Self { product_repository }
    }

    fn validate_barcode(&self, barcode: &str) -> Result<(), BarcodeError> {
        if barcode.is_empty() {
            return Err(BarcodeError {
                code: BarcodeErrorCode::InvalidFormat as i32,
                message: "Barcode cannot be empty".into(),
            });
        }

        // Check barcode type and validate accordingly
        if barcode.len() == 13 && barcode.chars().all(|c| c.is_ascii_digit()) {
            // EAN-13 validation with checksum
            self.validate_ean13(barcode)
        } else if barcode.len() == 12 && barcode.chars().all(|c| c.is_ascii_digit()) {
            // UPC-A validation with checksum
            self.validate_upc_a(barcode)
        } else if barcode.len() <= 100 {
            // QR code validation (alphanumeric)
            Ok(())
        } else {
            Err(BarcodeError {
                code: BarcodeErrorCode::InvalidFormat as i32,
                message: "Invalid barcode format".into(),
            })
        }
    }

    fn validate_ean13(&self, barcode: &str) -> Result<(), BarcodeError> {
        let digits: Vec<u32> = barcode.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        if digits.len() != 13 {
            return Err(BarcodeError {
                code: BarcodeErrorCode::InvalidFormat as i32,
                message: "EAN-13 must have exactly 13 digits".into(),
            });
        }

        let checksum = digits[12];
        let calculated_checksum = Self::calculate_ean_checksum(&digits[0..12]);

        if checksum != calculated_checksum {
            warn!("Invalid EAN-13 checksum for barcode: {}", barcode);
            return Err(BarcodeError {
                code: BarcodeErrorCode::InvalidFormat as i32,
                message: "Invalid EAN-13 checksum".into(),
            });
        }

        Ok(())
    }

    fn validate_upc_a(&self, barcode: &str) -> Result<(), BarcodeError> {
        let digits: Vec<u32> = barcode.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        if digits.len() != 12 {
            return Err(BarcodeError {
                code: BarcodeErrorCode::InvalidFormat as i32,
                message: "UPC-A must have exactly 12 digits".into(),
            });
        }

        let checksum = digits[11];
        let calculated_checksum = Self::calculate_upc_checksum(&digits[0..11]);

        if checksum != calculated_checksum {
            warn!("Invalid UPC-A checksum for barcode: {}", barcode);
            return Err(BarcodeError {
                code: BarcodeErrorCode::InvalidFormat as i32,
                message: "Invalid UPC-A checksum".into(),
            });
        }

        Ok(())
    }

    fn calculate_ean_checksum(digits: &[u32]) -> u32 {
        let sum: u32 = digits.iter().enumerate()
            .map(|(i, &d)| if i % 2 == 0 { d } else { d * 3 })
            .sum();
        
        (10 - (sum % 10)) % 10
    }

    fn calculate_upc_checksum(digits: &[u32]) -> u32 {
        let sum: u32 = digits.iter().enumerate()
            .map(|(i, &d)| if i % 2 == 0 { d * 3 } else { d })
            .sum();
        
        (10 - (sum % 10)) % 10
    }
}

#[tonic::async_trait]
impl<R: ProductRepository + 'static> BarcodeService for BarcodeServiceImpl<R> {
    async fn process_scan(
        &self,
        request: Request<BarcodeRequest>,
    ) -> Result<Response<ProductResponse>, Status> {
        let barcode_data = request.into_inner().barcode;
        info!("Processing barcode scan: {}", barcode_data);

        // Validate barcode format
        self.validate_barcode(&barcode_data)
            .map_err(|e| Status::invalid_argument(e.message))?;

        // Lookup product
        let product = self.product_repository
            .get_by_barcode(&barcode_data)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| {
                Status::not_found(format!("Product not found for barcode: {}", barcode_data))
            })?;

        // Map to response
        let response = ProductResponse {
            id: product.id.to_string(),
            name: product.name,
            description: product.description.unwrap_or_default(),
            category: product.category,
            brand: product.brand.unwrap_or_default(),
            sku: product.sku.unwrap_or_default(),
            barcode: product.barcode.unwrap_or_default(),
            created_at: Some(product.created_at.into()),
            updated_at: Some(product.updated_at.into()),
        };

        Ok(Response::new(response))
    }
}