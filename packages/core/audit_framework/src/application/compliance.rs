//! Regulatory rule engine
//! 
//! This module handles compliance checking and reporting for various regulations.

use crate::domain::{
    policy::{Regulation, RetentionPolicy, ComplianceReport},
    event::AuditEvent,
};

/// Compliance checker for various regulations
pub struct ComplianceChecker {
    /// Active retention policies
    retention_policies: Vec<RetentionPolicy>,
}

impl ComplianceChecker {
    /// Create a new compliance checker
    pub fn new(retention_policies: Vec<RetentionPolicy>) -> Self {
        Self { retention_policies }
    }
    
    /// Check compliance with HIPAA regulations
    pub fn check_hipaa_compliance(&self, events: &[AuditEvent]) -> ComplianceReport {
        // Check for required audit events
        let has_login_events = events.iter().any(|e| {
            e.domain == "auth" && 
            (e.action == crate::domain::event::AuditAction::Login || 
             e.action == crate::domain::event::AuditAction::Logout)
        });
        
        // Check for PHI access tracking
        let has_phi_access = events.iter().any(|e| {
            e.domain == "health" && 
            e.purpose == crate::domain::event::PurposeCode::ProviderAccess
        });
        
        // Check for research anonymization
        let has_anonymized_research = events.iter().any(|e| {
            e.domain == "health" && 
            e.purpose == crate::domain::event::PurposeCode::Research &&
            e.user_id.is_none()
        });
        
        if has_login_events && has_phi_access && has_anonymized_research {
            ComplianceReport::new(
                true,
                Regulation::Hipaa,
                "HIPAA compliance verified: All required audit events present".to_string(),
            )
        } else {
            let mut issues = Vec::new();
            if !has_login_events {
                issues.push("Missing authentication audit events");
            }
            if !has_phi_access {
                issues.push("Missing PHI access tracking");
            }
            if !has_anonymized_research {
                issues.push("Missing anonymized research access tracking");
            }
            
            ComplianceReport::new(
                false,
                Regulation::Hipaa,
                format!("HIPAA compliance issues: {}", issues.join(", ")),
            )
        }
    }
    
    /// Check compliance with GDPR regulations
    pub fn check_gdpr_compliance(&self, events: &[AuditEvent]) -> ComplianceReport {
        // Check for consent tracking
        let has_consent_events = events.iter().any(|e| {
            e.domain == "consent" && 
            (e.action == crate::domain::event::AuditAction::Create || 
             e.action == crate::domain::event::AuditAction::Update)
        });
        
        // Check for data export requests
        let has_export_events = events.iter().any(|e| {
            e.action == crate::domain::event::AuditAction::Export
        });
        
        // Check for data deletion requests
        let has_deletion_events = events.iter().any(|e| {
            e.action == crate::domain::event::AuditAction::Delete &&
            e.purpose == crate::domain::event::PurposeCode::UserView
        });
        
        if has_consent_events && has_export_events && has_deletion_events {
            ComplianceReport::new(
                true,
                Regulation::Gdpr,
                "GDPR compliance verified: All required audit events present".to_string(),
            )
        } else {
            let mut issues = Vec::new();
            if !has_consent_events {
                issues.push("Missing consent tracking events");
            }
            if !has_export_events {
                issues.push("Missing data export tracking");
            }
            if !has_deletion_events {
                issues.push("Missing data deletion tracking");
            }
            
            ComplianceReport::new(
                false,
                Regulation::Gdpr,
                format!("GDPR compliance issues: {}", issues.join(", ")),
            )
        }
    }
    
    /// Check compliance with PCI DSS regulations
    pub fn check_pci_dss_compliance(&self, events: &[AuditEvent]) -> ComplianceReport {
        // Check for payment processing events
        let has_payment_events = events.iter().any(|e| {
            e.domain == "payment" && 
            (e.action == crate::domain::event::AuditAction::Create || 
             e.action == crate::domain::event::AuditAction::Read)
        });
        
        // Check for security monitoring
        let has_security_events = events.iter().any(|e| {
            e.purpose == crate::domain::event::PurposeCode::SecurityMonitoring
        });
        
        if has_payment_events && has_security_events {
            ComplianceReport::new(
                true,
                Regulation::PciDss,
                "PCI DSS compliance verified: All required audit events present".to_string(),
            )
        } else {
            let mut issues = Vec::new();
            if !has_payment_events {
                issues.push("Missing payment processing tracking");
            }
            if !has_security_events {
                issues.push("Missing security monitoring events");
            }
            
            ComplianceReport::new(
                false,
                Regulation::PciDss,
                format!("PCI DSS compliance issues: {}", issues.join(", ")),
            )
        }
    }
    
    /// Get retention policy for a regulation
    pub fn get_retention_policy(&self, regulation: &Regulation) -> Option<&RetentionPolicy> {
        self.retention_policies.iter().find(|p| &p.regulation == regulation)
    }
}