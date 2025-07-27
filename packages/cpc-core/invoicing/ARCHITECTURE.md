# Invoicing & Quoting Module Architecture

## Domain Model Primitives

### Invoice
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_name: String,
    pub client_email: String,
    pub items: Vec<InvoiceItem>,
    pub total_amount: Decimal,
    pub due_date: DateTime<Utc>,
    pub status: PaymentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: u32,
    pub unit_price: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Draft,
    Sent,
    Viewed,
    Paid,
    Overdue,
    Partial,
}
```

### Quote
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_name: String,
    pub client_email: String,
    pub items: Vec<QuoteItem>,
    pub total_amount: Decimal,
    pub validity_period: Duration,
    pub status: QuoteStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuoteStatus {
    Draft,
    Sent,
    Accepted,
    Rejected,
    Expired,
}
```

## Application Service Boundaries

### InvoiceService
```rust
pub trait InvoiceRepository {
    async fn create(&self, invoice: Invoice) -> Result<Invoice, RepositoryError>;
    async fn update_status(&self, id: Uuid, status: PaymentStatus) -> Result<Invoice, RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Invoice, RepositoryError>;
}

pub struct InvoiceService {
    repo: Arc<dyn InvoiceRepository>,
    p2p_manager: Arc<P2PManager>,
}

impl InvoiceService {
    pub fn new(repo: Arc<dyn InvoiceRepository>, p2p_manager: Arc<P2PManager>) -> Self {
        Self { repo, p2p_manager }
    }

    pub async fn create_invoice(&self, input: CreateInvoiceInput) -> Result<Invoice, ServiceError> {
        // Domain validation occurs here
        let invoice = Invoice::new(input)?;
        let invoice = self.repo.create(invoice).await?;
        self.p2p_manager.share_invoice(&invoice).await?;
        Ok(invoice)
    }

    pub async fn send_invoice(&self, id: Uuid) -> Result<Invoice, ServiceError> {
        let mut invoice = self.repo.find_by_id(id).await?;
        invoice.status = PaymentStatus::Sent;
        invoice.updated_at = Utc::now();
        let updated = self.repo.update_status(invoice.id, invoice.status).await?;
        self.p2p_manager.notify_client(&updated).await?;
        Ok(updated)
    }
}
```

### QuoteService
```rust
pub trait QuoteRepository {
    async fn create(&self, quote: Quote) -> Result<Quote, RepositoryError>;
    async fn accept(&self, id: Uuid) -> Result<Invoice, ServiceError>;
}

pub struct QuoteService {
    repo: Arc<dyn QuoteRepository>,
    invoice_service: Arc<InvoiceService>,
}

impl QuoteService {
    pub fn new(repo: Arc<dyn QuoteRepository>, invoice_service: Arc<InvoiceService>) -> Self {
        Self { repo, invoice_service }
    }

    pub async fn accept_quote(&self, id: Uuid) -> Result<Invoice, ServiceError> {
        let quote = self.repo.find_by_id(id).await?;
        let invoice = quote.convert_to_invoice();
        self.invoice_service.create_invoice(invoice).await
    }
}
```

## P2P Data Sharing Strategy
- All Invoice/Quote data shared via p2panda peer channels using Double Ratchet encryption
- Business node initiates sharing with client node using client's public key
- Data structures follow p2panda schema definitions with cryptographic verification
- Sensitive metadata (amounts, client info) encrypted before sharing
- Append-only logs with versioning for auditability
- Must implement `P2PSharable` trait from `cpc-net` for serialization

## Presentation Layer Requirements
### Bevy (3D Visualizations)
- Invoice status visualization as color-coded 3D objects
- Overdue invoices highlighted with pulsating effect
- Financial dashboard with spatial organization of invoices by status/due date
- Interactive timeline showing payment history

### Yew (Web Components)
- Invoice creation form with line item management
- Real-time status tracking with p2p updates
- Client-facing invoice view with payment button
- Quote generator with template system
- Dashboard showing summary metrics (outstanding invoices, revenue)