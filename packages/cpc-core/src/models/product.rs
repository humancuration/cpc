use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents a product in the system with its basic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub brand: Option<String>,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents the origin information of a product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductOrigin {
    pub id: Uuid,
    pub product_id: Uuid,
    pub country_of_origin: String,
    pub region: Option<String>,
    pub city: Option<String>,
    pub manufacturer: String,
    pub manufacturer_address: Option<String>,
    pub certification_info: Option<String>,
    pub verified: bool,
    pub verification_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents a supply chain entry for tracking product journey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChain {
    pub id: Uuid,
    pub product_id: Uuid,
    pub stage: SupplyChainStage,
    pub location: String,
    pub organization: String,
    pub timestamp: DateTime<Utc>,
    pub description: Option<String>,
    pub verification_status: VerificationStatus,
    pub previous_stage_id: Option<Uuid>,
    pub next_stage_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Enum representing different stages in the supply chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupplyChainStage {
    RawMaterial,
    Manufacturing,
    Processing,
    Packaging,
    Distribution,
    Wholesale,
    Retail,
    Consumer,
}

/// Enum representing verification status of supply chain entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
    Unverified,
    Pending,
    Verified,
    Disputed,
    Rejected,
}

impl Product {
    /// Creates a new Product instance
    pub fn new(
        name: String,
        description: Option<String>,
        category: String,
        brand: Option<String>,
        sku: Option<String>,
        barcode: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            category,
            brand,
            sku,
            barcode,
            created_at: now,
            updated_at: now,
        }
    }
}

impl ProductOrigin {
    /// Creates a new ProductOrigin instance
    pub fn new(
        product_id: Uuid,
        country_of_origin: String,
        region: Option<String>,
        city: Option<String>,
        manufacturer: String,
        manufacturer_address: Option<String>,
        certification_info: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            product_id,
            country_of_origin,
            region,
            city,
            manufacturer,
            manufacturer_address,
            certification_info,
            verified: false,
            verification_date: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Marks the product origin as verified
    pub fn verify(&mut self) {
        self.verified = true;
        self.verification_date = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

impl SupplyChain {
    /// Creates a new SupplyChain entry
    pub fn new(
        product_id: Uuid,
        stage: SupplyChainStage,
        location: String,
        organization: String,
        timestamp: DateTime<Utc>,
        description: Option<String>,
        previous_stage_id: Option<Uuid>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            product_id,
            stage,
            location,
            organization,
            timestamp,
            description,
            verification_status: VerificationStatus::Unverified,
            previous_stage_id,
            next_stage_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Updates the verification status
    pub fn set_verification_status(&mut self, status: VerificationStatus) {
        self.verification_status = status;
        self.updated_at = Utc::now();
    }

    /// Links this entry to the next stage in the supply chain
    pub fn link_next_stage(&mut self, next_stage_id: Uuid) {
        self.next_stage_id = Some(next_stage_id);
        self.updated_at = Utc::now();
    }
}