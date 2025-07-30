# Invoicing & Quoting Module Architecture (v2)

## Domain Model Primitives (Enhanced)

### Invoice (Enhanced)
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
    pub payment_provider: Option<PaymentProvider>,
    pub payment_intent_id: Option<String>,
    pub next_reminder_date: Option<DateTime<Utc>>,
}

pub enum PaymentProvider {
    Stripe,
    PayPal,
    Manual,
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

## Application Service Boundaries (Enhanced)

### InvoiceService (Enhanced)
```
pub trait InvoiceRepository {
    async fn create(&self, invoice: Invoice) -> Result<Invoice, RepositoryError>;
    async fn update_status(&self, id: Uuid, status: PaymentStatus) -> Result<Invoice, RepositoryError>;
    async fn update_payment(&self, id: Uuid, provider: PaymentProvider, intent_id: String) -> Result<Invoice, RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Invoice, RepositoryError>;
    async fn get_pending_reminders(&self) -> Result<Vec<Invoice>, RepositoryError>;
}

pub struct InvoiceService {
    repo: Arc<dyn InvoiceRepository>,
    p2p_manager: Arc<P2PManager>,
    payment_processor: Arc<dyn PaymentProcessor>,
    reminder_service: Arc<ReminderService>,
}

impl InvoiceService {
    pub fn new(
        repo: Arc<dyn InvoiceRepository>,
        p2p_manager: Arc<P2PManager>,
        payment_processor: Arc<dyn PaymentProcessor>,
        reminder_service: Arc<ReminderService>
    ) -> Self {
        Self { repo, p2p_manager, payment_processor, reminder_service }
    }

    pub async fn create_invoice(&self, input: CreateInvoiceInput) -> Result<Invoice, ServiceError> {
        // Domain validation occurs here
        let mut invoice = Invoice::new(input)?;
        
        // Set next reminder date based on configuration
        invoice.next_reminder_date = self.calculate_next_reminder_date(&invoice);
        
        let invoice = self.repo.create(invoice).await?;
        self.p2p_manager.share_invoice(&invoice).await?;
        Ok(invoice)
    }

    pub async fn send_invoice(&self, id: Uuid) -> Result<Invoice, ServiceError> {
        let mut invoice = self.repo.find_by_id(id).await?;
        invoice.status = PaymentStatus::Sent;
        invoice.updated_at = Utc::now();
        
        // Schedule first reminder if needed
        if invoice.due_date - Utc::now() <= chrono::Duration::days(3) {
            self.reminder_service.schedule_reminder(&invoice).await?;
        }
        
        let updated = self.repo.update_status(invoice.id, invoice.status).await?;
        self.p2p_manager.notify_client(&updated).await?;
        Ok(updated)
    }
    
    pub async fn process_payment(&self, id: Uuid, payment_data: PaymentData) -> Result<Invoice, ServiceError> {
        let mut invoice = self.repo.find_by_id(id).await?;
        
        // Process payment through selected provider
        let payment_result = self.payment_processor.process_payment(&invoice, payment_data).await?;
        
        // Update invoice status based on payment result
        match payment_result {
            PaymentResult::Success(provider, intent_id) => {
                invoice.payment_provider = Some(provider);
                invoice.payment_intent_id = Some(intent_id);
                invoice.status = PaymentStatus::Paid;
                invoice.updated_at = Utc::now();
            }
            PaymentResult::Pending => {
                invoice.status = PaymentStatus::Pending;
            }
            PaymentResult::Failed => {
                invoice.status = PaymentStatus::PaymentFailed;
            }
        }
        
        let updated = self.repo.update(invoice).await?;
        self.p2p_manager.notify_payment_status(&updated).await?;
        Ok(updated)
    }
    
    fn calculate_next_reminder_date(&self, invoice: &Invoice) -> Option<DateTime<Utc>> {
        // Implementation based on PaymentReminderConfig
        None
    }
}

// New Payment Processor Trait
pub trait PaymentProcessor: Send + Sync {
    async fn process_payment(&self, invoice: &Invoice, payment_data: PaymentData) -> Result<PaymentResult, PaymentError>;
    async fn get_payment_status(&self, provider: PaymentProvider, intent_id: &str) -> Result<PaymentStatus, PaymentError>;
}

pub struct PaymentData {
    pub provider: PaymentProvider,
    pub token: String,
    // Additional provider-specific data
}

pub enum PaymentResult {
    Success(PaymentProvider, String), // (provider, intent_id)
    Pending,
    Failed,
}

// New Reminder Service
pub struct ReminderService {
    repo: Arc<dyn InvoiceRepository>,
    notifier: Arc<dyn NotificationService>,
}

impl ReminderService {
    pub async fn process_pending_reminders(&self) -> Result<(), ServiceError> {
        let invoices = self.repo.get_pending_reminders().await?;
        
        for invoice in invoices {
            if let Some(next_date) = invoice.next_reminder_date {
                if Utc::now() >= next_date {
                    self.send_reminder(&invoice).await?;
                    self.schedule_next_reminder(&invoice).await?;
                }
            }
        }
        
        Ok(())
    }
    
    async fn send_reminder(&self, invoice: &Invoice) -> Result<(), ServiceError> {
        self.notifier.send_invoice_reminder(invoice).await
    }
    
    async fn schedule_next_reminder(&self, invoice: &Invoice) -> Result<(), ServiceError> {
        // Update invoice with next reminder date
        Ok(())
    }
}


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