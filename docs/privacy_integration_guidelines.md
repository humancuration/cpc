# Privacy Integration Guidelines for Cross-Module Data Sharing

## Introduction

This document outlines the privacy principles and implementation guidelines for integrating the Calendar module with CRM and Invoicing systems. Our goal is to enable powerful cross-module functionality while maintaining strict privacy controls and compliance with data protection regulations.

## Privacy Principles

### 1. Consent-First Architecture

All cross-module data sharing **must** adhere to these consent requirements:

- **Explicit Opt-In**: Users must explicitly enable data sharing between modules
- **Granular Control**: Users can control data sharing at multiple levels:
  - Module level (enable/disable CRM ↔ Calendar integration)
  - Data type level (share only sales pipeline but not lead scoring)
  - Time-bound (temporary consent for specific duration)
- **Just-In-Time Notifications**: Users receive clear explanations of data usage when granting consent
- **Easy Revocation**: One-click consent revocation with immediate effect

### 2. Data Minimization

Only the minimum necessary data should flow between modules:

| Data Type | CRM → Calendar | Invoicing → Calendar | Minimization Strategy |
|-----------|----------------|----------------------|------------------------|
| Sales Pipeline | Stage transition | ❌ | Only share stage name, not full opportunity details |
| Lead Scoring | Score change threshold | ❌ | Only share delta, not full scoring history |
| Email Campaigns | Schedule timeline | ❌ | Share only campaign name and duration |
| Invoice Data | ❌ | Due date, amount | Never share full invoice details |
| Payment Status | ❌ | Status changes | Only share status transitions, not payment methods |

### 3. End-to-End Encryption

All integrated events must use Double Ratchet encryption with these requirements:

- **Key Rotation**: Keys rotate with every event update (max 7 days)
- **Separate Key Chains**: Each integration has its own key chain
- **Zero-Knowledge Proof**: Verification without revealing sensitive data
- **On-Device Processing**: All encryption/decryption happens on user's device
- **Secure Key Exchange**: Through p2panda's authenticated channels

### 4. Purpose Limitation

Data shared between modules must adhere to strict purpose limitations:

- CRM data in Calendar can only be used for:
  - Timeline visualization
  - Sales pipeline progress tracking
  - Lead follow-up scheduling
  
- Invoicing data in Calendar can only be used for:
  - Payment due date reminders
  - Payment status visualization
  - Financial timeline organization

## Implementation Requirements

### 1. Consent Management API

All integration points must implement this pattern:

```rust
pub async fn register_crm_event(
    &self,
    user_id: Uuid,
    event: CrmEvent,
    purpose: ConsentPurpose,
) -> Result<EventRegistrationResponse, CalendarError> {
    // 1. Verify consent
    if !self.consent_checker.has_consent(
        user_id, 
        Module::Crm, 
        Module::Calendar,
        purpose
    ).await? {
        return Err(CalendarError::MissingConsent);
    }

    // 2. Apply data minimization
    let minimized_event = self.minimizer.minimize_crm_event(event);

    // 3. Register the event
    self.calendar_service.register_event(user_id, minimized_event).await
}
```

### 2. Data Flow Restrictions

| Integration Direction | Allowed Data | Forbidden Data | Validation Rule |
|-----------------------|--------------|----------------|-----------------|
| CRM → Calendar | Sales stage transitions, Lead follow-up markers | Full lead details, Contact information | Contains no PII beyond stage names |
| Invoicing → Calendar | Payment due dates, Status changes | Payment methods, Bank details | Contains no financial account data |
| Calendar → CRM | Meeting outcomes (opt-in) | ❌ | Requires separate consent |
Calendar → Invoicing | Payment confirmation (opt-in) | ❌ | Requires separate consent |

### 3. Audit Trail Requirements

All cross-module data access must be logged with these fields:

```rust
pub struct DataAccessLog {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub user_id: Uuid,
    pub source_module: Module,
    pub target_module: Module,
    pub data_type: DataType,
    pub purpose: ConsentPurpose,
    pub consent_id: Option<Uuid>,
    pub data_volume: usize, // Number of data points shared
    pub anonymized_sample: String, // First 50 chars of sanitized data
}
```

#### Implementation Status
- **Missing**: Actual implementation of audit trail system in codebase
- **Required**: Add logging after consent check but before event conversion
- **Sample Implementation**:
  ```rust
  // After consent check (calendar_integration.rs)
  self.audit_logger.log_data_access(
      user_id,
      Module::Crm,
      Module::Calendar,
      DataType::LeadScoringData,
      ConsentPurpose::CrmIntegration,
      consent_record.id,
      1, // data volume
      format!("ScoreChange: {}", event.score_change)
  ).await?;
  ```

### 4. Double Ratchet Encryption Requirements

All cross-module data sharing must use Double Ratchet encryption with:

- **Key Rotation**: Keys rotate with every event update (max 7 days)
- **Separate Key Chains**: Each integration has its own key chain
- **Zero-Knowledge Proof**: Verification without revealing sensitive data
- **On-Device Processing**: All encryption/decryption happens on user's device
- **Secure Key Exchange**: Through p2panda's authenticated channels

#### Implementation Status
- **Partially Implemented**: Architecture specifies requirements but implementation details missing
- **Required**:
  - Complete implementation of `SyncableEvent` trait for CalendarEvent
  - Verify separate key chains for CRM vs Invoicing integrations
  - Implement key rotation strategy per event update

## GDPR/CCPA Compliance

### 1. Data Subject Rights Implementation

Right | Implementation Strategy |
|-------|-------------------------|
Right to Access | Users can view all cross-module data sharing in Settings → Privacy |
Right to Rectification | Users can edit or remove shared data through the module it originated from |
Right to Erasure | Deleting data in source module automatically removes it from all integrated modules |
Right to Restriction | Users can temporarily pause data sharing without deletion |
Right to Data Portability | All integrated data can be exported in standard format |

## Special Considerations for Wellness Data

When CRM module shares lead follow-up events based on wellness thresholds:

- **Anonymization**: Wellness metrics must be thresholded (e.g., "high stress" instead of exact score)
- **Aggregation**: Individual lead data should be aggregated when possible
- **Explicit Consent**: Separate consent required for wellness data integration
- **Limited Retention**: Wellness data in calendar module auto-deletes after 30 days

## Testing Requirements

All integration points must pass these tests:

1. **Consent Enforcement Test**:
   - Verify integration fails gracefully without consent
   - Verify appropriate error message is shown to user

2. **Data Minimization Test**:
   - Verify no PII beyond allowed fields is shared
   - Verify sensitive fields are properly redacted

3. **Encryption Test**:
   - Verify all data in transit is encrypted
   - Verify decryption only happens in trusted environment

4. **Audit Trail Test**:
   - **Critical**: Verify all data access is properly logged
   - **Critical**: Verify logs contain required fields

## Example Implementation: CRM Follow-Up Event

### Before Integration (CRM Module)
```rust
// Full lead data in CRM
Lead {
    id: Uuid,
    name: "John Doe",
    email: "john@example.com",
    phone: "+15551234567",
    score: 85,
    wellness: WellnessMetrics {
        stress_level: Some(75),
        focus_level: Some(60),
        burnout_risk: Some(0.7),
    },
    // ... other fields
}
```

### After Minimization (Calendar Module)
```rust
// Minimized follow-up event
CalendarEvent {
    event_type: EventType::LeadFollowUp {
        lead_id: Uuid, // Not personally identifiable
        score_change: 15, // Just the delta
        wellness_threshold: Some(70), // Only threshold met, not actual value
    },
    // ... other calendar fields
}
```

## UI/UX Guidelines

### Consent Management Interface

1. **Module Connection Panel**:
   - Visual toggle for each integration
   - Clear explanation of data usage
   - "See details" expandable section

2. **Data Preview**:
   - Before enabling integration, show example of what data will be shared
   - Highlight which fields are shared vs. protected

3. **Privacy Dashboard**:
   - Timeline of all cross-module data sharing
   - Filter by module, date, data type
   - One-click revocation for any sharing instance

## Audit and Compliance

- **Quarterly Audits**: Automated verification of data minimization
- **Consent Verification**: Random sampling of consent records
- **Breach Notification**: Immediate alert if data minimization fails
- **Compliance Dashboard**: Real-time view of privacy metrics
- **Critical**: Implementation of audit trail system required for compliance verification

This document must be reviewed and updated quarterly or whenever new integration points are added.