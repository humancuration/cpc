# Integration Guide: Cooperative Fundraising System

## Overview
This guide explains how the cooperative fundraising system integrates with existing CPC ecosystem components, with special attention to **cpay** (payment processor) and **wallet** (fund management) packages as recently clarified.

## 1. cpay Integration (External Payments)

### Core Functionality
- Processes all external monetary transactions (credit cards, bank transfers)
- Handles compliance with payment card industry standards
- Provides transaction verification and reconciliation

### Integration Points
```rust
// In contributions.rs (hypothetical)
pub struct MonetaryContribution {
    cpay_transaction_id: Uuid,
    amount: f64,
    currency: String,
    // Links to cpay's transaction record
    cpay_ref: Option<TransactionReference>,
}

// Service layer integration
async fn process_monetary_contribution(
    contribution: &MonetaryContribution,
    cpay_client: &CpayClient
) -> Result<(), CpayError> {
    // Verify transaction with cpay
    let verification = cpay_client
        .verify_transaction(contribution.cpay_transaction_id)
        .await?;
    
    // Ensure amount matches
    if verification.amount != contribution.amount {
        return Err(CpayError::AmountMismatch);
    }
    
    // Mark as verified
    Ok(())
}
```

### Key Constraints
- Monetary contributions ONLY permitted for `PureDonation`, `RegCF`, and `RegA` campaign types
- All transactions must be verified through cpay before being marked as `verified`
- cpay handles all PCI compliance - fundraising system only stores transaction IDs

## 2. wallet Integration (Fund Management)

### Core Functionality
- Tracks allocation of funds across cooperative initiatives
- Manages disbursements to external entities (vendors, services)
- Provides transparency into fund utilization

### Integration Points
```sql
-- Updated contributions table with wallet integration
ALTER TABLE contributions 
ADD COLUMN wallet_transaction_id UUID REFERENCES wallet.transactions(id);

-- New table for fund allocation
CREATE TABLE fund_allocations (
    id UUID PRIMARY KEY,
    campaign_id UUID NOT NULL REFERENCES campaigns(id),
    wallet_account_id UUID NOT NULL,
    amount NUMERIC(18,2) NOT NULL,
    purpose TEXT NOT NULL, -- Must match external_use_case
    allocated_at TIMESTAMPTZ DEFAULT NOW(),
    disbursed BOOLEAN DEFAULT FALSE
);
```

### Critical Workflows
1. **Donation Processing**:
   - cpay processes external payment → creates wallet transaction
   - Contribution record links to both cpay & wallet transactions
   - Funds remain in wallet until allocation approved

2. **Fund Disbursement**:
   - Cooperative admins propose allocations through UI
   - wallet verifies sufficient balance
   - Disbursement triggers cpay external transfer (if needed)

3. **Transparency Reporting**:
   - All fund movements visible to cooperative members
   - wallet provides immutable audit trail
   - External use cases must match campaign documentation

## 3. skill_volunteering Integration (Community Participation)

### Verified via Existing System
- All volunteer actions link to `skill_volunteering` opportunities
- Verification status flows through existing workflow:
  ```mermaid
  graph LR
    A[Volunteer Completes Task] --> B(skill_volunteering Verification)
    B -->|Verified| C[Update Contribution Status]
    B -->|Disputed| D[Resolution Workflow]
  ```

### Critical Constraints
- Volunteer hours have **NO monetary value** in system
- Verification ONLY confirms participation occurred
- Hours tracked purely for community engagement metrics

## 4. User Profile Enforcement (1 Share Per Person)

### Implementation
```rust
// In user_shares.rs
pub async fn grant_membership(
    user_id: Uuid,
    campaign_id: Uuid,
    db: &PgPool
) -> Result<(), MembershipError> {
    // Check if user already has membership share
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM user_shares 
         WHERE user_id = $1 AND campaign_id IN 
           (SELECT id FROM campaigns WHERE type = 'cooperative_membership'))",
        user_id
    )
    .fetch_one(db)
    .await?
    .unwrap_or(false);

    if exists {
        return Err(MembershipError::MaxSharesExceeded);
    }

    // Grant share
    sqlx::query!(
        "INSERT INTO user_shares (user_id, campaign_id) VALUES ($1, $2)",
        user_id,
        campaign_id
    )
    .execute(db)
    .await?;

    Ok(())
}
```

### Enforcement Points
1. At contribution creation for membership campaigns
2. During campaign completion processing
3. In all membership verification workflows

## 5. Regulatory Compliance Interfaces

### Required for RegCF/RegA Campaigns
```rust
// Regulatory interface (to be implemented later)
#[cfg(feature = "regulatory")]
mod regulatory {
    pub struct ComplianceChecker;
    
    impl ComplianceChecker {
        pub fn validate_campaign(campaign: &Campaign) -> Result<(), RegulatoryError> {
            // [TODO: Regulatory] Implementation
            Ok(())
        }
    }
}
```

### Current Placeholders
- All regulatory-specific fields marked with `[TODO: Regulatory]`
- External use case required for all monetary donations
- Full audit trail maintained via wallet and cpay integrations

## Key Integration Principles

| Principle | Implementation |
|-----------|----------------|
| **Money as Tool** | Monetary systems only interface with external world |
| **Labor ≠ Currency** | Volunteer hours never converted to monetary values |
| **Transparent Allocation** | All fund movements visible through wallet |
| **One Person, One Vote** | Strict enforcement at database and service layers |
| **Regulatory Minimalism** | Only implement compliance where absolutely necessary |

This integration strategy maintains our cooperative ethos while providing necessary interfaces with external financial systems.