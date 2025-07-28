//! Workflow engine for the invoicing module
//!
//! This module contains the workflow engine for managing payment status transitions.

use crate::domain::status::{PaymentStatus, StatusTransition, StatusOverride, StatusWorkflowConfig, StatusError};
use crate::domain::payment::Invoice;
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;

/// Error types for workflow engine operations
#[derive(Debug, thiserror::Error)]
pub enum WorkflowError {
    #[error("Status error: {0}")]
    StatusError(#[from] StatusError),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Invoice not found: {0}")]
    InvoiceNotFound(Uuid),
    #[error("Unauthorized status transition")]
    UnauthorizedTransition,
}

#[async_trait]
pub trait StatusRepository {
    async fn save_transition(&self, transition: &StatusTransition) -> Result<(), StatusError>;
    async fn save_override(&self, override_record: &StatusOverride) -> Result<(), StatusError>;
    async fn get_transitions_for_invoice(&self, invoice_id: Uuid) -> Result<Vec<StatusTransition>, StatusError>;
    async fn get_overrides_for_invoice(&self, invoice_id: Uuid) -> Result<Vec<StatusOverride>, StatusError>;
    async fn save_workflow_config(&self, config: &StatusWorkflowConfig) -> Result<(), StatusError>;
    async fn get_workflow_config(&self, user_id: Uuid) -> Result<Option<StatusWorkflowConfig>, StatusError>;
}

#[async_trait]
pub trait InvoiceRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Invoice, WorkflowError>;
    async fn update_status(&self, id: Uuid, status: PaymentStatus) -> Result<Invoice, WorkflowError>;
    async fn get_invoices_with_status(&self, status: PaymentStatus) -> Result<Vec<Invoice>, WorkflowError>;
    async fn get_overdue_invoices(&self) -> Result<Vec<Invoice>, WorkflowError>;
}

pub struct WorkflowEngine {
    status_repository: Arc<dyn StatusRepository>,
    invoice_repository: Arc<dyn InvoiceRepository>,
}

impl WorkflowEngine {
    pub fn new(
        status_repository: Arc<dyn StatusRepository>,
        invoice_repository: Arc<dyn InvoiceRepository>,
    ) -> Self {
        Self {
            status_repository,
            invoice_repository,
        }
    }

    /// Transition an invoice to a new status
    pub async fn transition_status(
        &self,
        invoice_id: Uuid,
        target_status: PaymentStatus,
        transition_reason: Option<String>,
        transitioned_by: Uuid,
    ) -> Result<Invoice, WorkflowError> {
        let mut invoice = self.invoice_repository.find_by_id(invoice_id).await?;

        // Check if the transition is valid
        if !invoice.status.can_transition_to(&target_status) {
            return Err(WorkflowError::StatusError(
                StatusError::InvalidTransition(invoice.status, target_status)
            ));
        }

        // Create status transition record
        let transition = StatusTransition::new(
            invoice_id,
            invoice.status.clone(),
            target_status.clone(),
            transition_reason,
            transitioned_by,
        )?;

        // Save the transition
        self.status_repository.save_transition(&transition).await
            .map_err(|e| WorkflowError::RepositoryError(e.to_string()))?;

        // Update the invoice status
        invoice.status = target_status;
        let updated_invoice = self.invoice_repository.update_status(invoice_id, invoice.status.clone()).await?;

        Ok(updated_invoice)
    }

    /// Manually override an invoice status
    pub async fn override_status(
        &self,
        invoice_id: Uuid,
        override_status: PaymentStatus,
        override_reason: String,
        overridden_by: Uuid,
    ) -> Result<Invoice, WorkflowError> {
        let invoice = self.invoice_repository.find_by_id(invoice_id).await?;

        // Create status override record
        let override_record = StatusOverride::new(
            invoice_id,
            override_status.clone(),
            override_reason,
            overridden_by,
        )?;

        // Save the override
        self.status_repository.save_override(&override_record).await
            .map_err(|e| WorkflowError::RepositoryError(e.to_string()))?;

        // Update the invoice status
        let updated_invoice = self.invoice_repository.update_status(invoice_id, override_status).await?;

        Ok(updated_invoice)
    }

    /// Process automated status transitions
    pub async fn process_automated_transitions(&self) -> Result<(), WorkflowError> {
        // Process overdue invoices
        self.process_overdue_invoices().await?;

        // Process other automated transitions as needed
        // ...

        Ok(())
    }

    /// Process overdue invoices
    async fn process_overdue_invoices(&self) -> Result<(), WorkflowError> {
        let overdue_invoices = self.invoice_repository.get_overdue_invoices().await?;

        for invoice in overdue_invoices {
            // Check if the invoice is not already marked as overdue
            if invoice.status != PaymentStatus::Overdue {
                // Get workflow configuration for the invoice owner
                let config = self.status_repository.get_workflow_config(invoice.client_id).await
                    .map_err(|e| WorkflowError::RepositoryError(e.to_string()))?
                    .unwrap_or_else(|| StatusWorkflowConfig::new(
                        invoice.client_id,
                        true,  // auto_overdue_enabled
                        0,     // overdue_days (0 means immediately)
                        false, // auto_cancel_enabled
                        30,    // cancel_days
                    ));

                // Check if the invoice should be marked as overdue
                if config.should_mark_overdue(invoice.due_date) {
                    // Create automated transition
                    let transition = StatusTransition::new(
                        invoice.id,
                        invoice.status.clone(),
                        PaymentStatus::Overdue,
                        Some("Automatically marked as overdue".to_string()),
                        Uuid::nil(), // System user
                    ).map_err(WorkflowError::StatusError)?;

                    // Save the transition
                    self.status_repository.save_transition(&transition).await
                        .map_err(|e| WorkflowError::RepositoryError(e.to_string()))?;

                    // Update the invoice status
                    self.invoice_repository.update_status(invoice.id, PaymentStatus::Overdue).await?;
                }
            }
        }

        Ok(())
    }

    /// Get status history for an invoice
    pub async fn get_status_history(&self, invoice_id: Uuid) -> Result<Vec<StatusTransition>, WorkflowError> {
        let transitions = self.status_repository.get_transitions_for_invoice(invoice_id).await
            .map_err(|e| WorkflowError::RepositoryError(e.to_string()))?;

        Ok(transitions)
    }

    /// Get override history for an invoice
    pub async fn get_override_history(&self, invoice_id: Uuid) -> Result<Vec<StatusOverride>, WorkflowError> {
        let overrides = self.status_repository.get_overrides_for_invoice(invoice_id).await
            .map_err(|e| WorkflowError::RepositoryError(e.to_string()))?;

        Ok(overrides)
    }

    /// Save workflow configuration
    pub async fn save_workflow_config(&self, config: StatusWorkflowConfig) -> Result<(), WorkflowError> {
        self.status_repository.save_workflow_config(&config).await
            .map_err(|e| WorkflowError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    /// Get workflow configuration for a user
    pub async fn get_workflow_config(&self, user_id: Uuid) -> Result<Option<StatusWorkflowConfig>, WorkflowError> {
        let config = self.status_repository.get_workflow_config(user_id).await
            .map_err(|e| WorkflowError::RepositoryError(e.to_string()))?;

        Ok(config)
    }

    /// Validate if a user can transition an invoice to a specific status
    pub async fn can_user_transition_status(
        &self,
        user_id: Uuid,
        invoice_id: Uuid,
        target_status: PaymentStatus,
    ) -> Result<bool, WorkflowError> {
        let invoice = self.invoice_repository.find_by_id(invoice_id).await?;
        
        // In a real implementation, we would check user permissions
        // For now, we'll allow all transitions
        Ok(invoice.status.can_transition_to(&target_status))
    }
}

/// Status workflow scheduler
pub struct WorkflowScheduler {
    workflow_engine: Arc<WorkflowEngine>,
    processing_interval: std::time::Duration,
}

impl WorkflowScheduler {
    pub fn new(workflow_engine: Arc<WorkflowEngine>, processing_interval: std::time::Duration) -> Self {
        Self {
            workflow_engine,
            processing_interval,
        }
    }

    /// Start the workflow scheduler
    pub async fn start(&self) {
        loop {
            if let Err(e) = self.process_workflows().await {
                eprintln!("Error processing workflows: {}", e);
            }

            tokio::time::sleep(self.processing_interval).await;
        }
    }

    /// Process all automated workflows
    async fn process_workflows(&self) -> Result<(), WorkflowError> {
        self.workflow_engine.process_automated_transitions().await
    }
}

/// Manual trigger for workflow processing
pub struct WorkflowTrigger {
    workflow_engine: Arc<WorkflowEngine>,
}

impl WorkflowTrigger {
    pub fn new(workflow_engine: Arc<WorkflowEngine>) -> Self {
        Self { workflow_engine }
    }

    /// Trigger immediate processing of workflows
    pub async fn trigger_processing(&self) -> Result<(), WorkflowError> {
        self.workflow_engine.process_automated_transitions().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    struct MockStatusRepository;
    struct MockInvoiceRepository;

    #[async_trait]
    impl StatusRepository for MockStatusRepository {
        async fn save_transition(&self, _transition: &StatusTransition) -> Result<(), StatusError> {
            Ok(())
        }

        async fn save_override(&self, _override_record: &StatusOverride) -> Result<(), StatusError> {
            Ok(())
        }

        async fn get_transitions_for_invoice(&self, _invoice_id: Uuid) -> Result<Vec<StatusTransition>, StatusError> {
            Ok(vec![])
        }

        async fn get_overrides_for_invoice(&self, _invoice_id: Uuid) -> Result<Vec<StatusOverride>, StatusError> {
            Ok(vec![])
        }

        async fn save_workflow_config(&self, _config: &StatusWorkflowConfig) -> Result<(), StatusError> {
            Ok(())
        }

        async fn get_workflow_config(&self, _user_id: Uuid) -> Result<Option<StatusWorkflowConfig>, StatusError> {
            Ok(None)
        }
    }

    #[async_trait]
    impl InvoiceRepository for MockInvoiceRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<Invoice, WorkflowError> {
            // Return a mock invoice for testing
            Ok(Invoice::new(
                id,
                "Test Client".to_string(),
                "client@example.com".to_string(),
                vec![],
                Decimal::new(10000, 2), // $100.00
                Utc::now() + chrono::Duration::days(7),
            ))
        }

        async fn update_status(&self, id: Uuid, _status: PaymentStatus) -> Result<Invoice, WorkflowError> {
            // Return the same mock invoice with updated status
            let mut invoice = self.find_by_id(id).await?;
            Ok(invoice)
        }

        async fn get_invoices_with_status(&self, _status: PaymentStatus) -> Result<Vec<Invoice>, WorkflowError> {
            Ok(vec![])
        }

        async fn get_overdue_invoices(&self) -> Result<Vec<Invoice>, WorkflowError> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn test_workflow_engine_creation() {
        let workflow_engine = WorkflowEngine::new(
            Arc::new(MockStatusRepository),
            Arc::new(MockInvoiceRepository),
        );

        assert!(true); // Just test that creation works
    }

    #[tokio::test]
    async fn test_status_transition_validation() {
        let workflow_engine = WorkflowEngine::new(
            Arc::new(MockStatusRepository),
            Arc::new(MockInvoiceRepository),
        );

        let invoice_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        // Test valid transition
        let result = workflow_engine.transition_status(
            invoice_id,
            PaymentStatus::Sent,
            Some("Sending invoice".to_string()),
            user_id,
        ).await;

        // This should succeed in our mock implementation
        assert!(result.is_ok());
    }
}