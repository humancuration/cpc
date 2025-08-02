# Cooperative Fundraising System

This package implements the cooperative fundraising system for the CPC platform, enabling community-driven fundraising while maintaining cooperative principles.

## Overview

The cooperative fundraising system allows communities to:
1. Create membership drives for cooperative participation
2. Run donation campaigns for external needs
3. Track both monetary and volunteer contributions
4. Maintain the principle of "one person, one vote" through membership shares

## Core Concepts

### Campaign Types

1. **Cooperative Membership** - Pure participation-based campaigns for joining the cooperative
2. **Pure Donation** - GoFundMe-style campaigns for external needs
3. **Reg CF/Reg A/Reg D** - SEC-compliant campaigns for regulatory requirements

### Contribution Types

1. **Monetary Contributions** - For donation campaigns only, processed through cpay
2. **Volunteer Actions** - For all campaign types, tracked through skill_volunteering

### Membership Model

- Strictly enforced: 1 membership share per person globally
- Membership represents community participation, not financial investment
- Volunteer hours have no monetary value within the federation

## Architecture

The system follows a hexagonal architecture with clear separation of concerns:

- **Domain Layer**: Core business logic and entities
- **Application Layer**: Services coordinating use cases
- **Infrastructure Layer**: Database implementations and external integrations
- **Interface Layer**: gRPC service implementation

## Key Components

### Domain Models
- `Campaign` - Fundraising campaigns of various types
- `Contribution` - Monetary or volunteer contributions to campaigns
- `Membership` - Cooperative membership shares

### Application Services
- `CampaignService` - Campaign creation and management
- `ContributionService` - Contribution processing
- `MembershipService` - Membership management

### Infrastructure
- PostgreSQL repositories for data persistence
- gRPC service implementation
- Integrations with cpay, wallet, and skill_volunteering systems

## Database Schema

The system uses several tables to track campaigns and contributions:

- `campaigns` - Main campaign information
- `membership_requirements` - Requirements for membership campaigns
- `donation_campaigns` - Details for donation campaigns
- `contributions` - Individual contributions (monetary or volunteer)
- `user_shares` - Membership shares with proper constraint enforcement (see ADR-001)

## Critical Architectural Decisions

### ADR-001: Membership Constraint Implementation

**Problem**: Need to enforce "1 membership share per person globally" at database level.

**Options Considered**:
1. Partial unique index (not possible due to cross-table reference)
2. Application-level validation (vulnerable to race conditions)
3. Trigger-based constraint (selected solution)

**Solution**:
Implemented a trigger on `user_shares` that verifies:
```sql
CREATE OR REPLACE FUNCTION validate_membership_uniqueness()
RETURNS TRIGGER AS $$
BEGIN
  IF EXISTS (
    SELECT 1 FROM user_shares us
    JOIN campaigns c ON us.campaign_id = c.id
    WHERE us.user_id = NEW.user_id
      AND c.type = 'cooperative_membership'
      AND us.campaign_id != NEW.campaign_id
  ) THEN
    RAISE EXCEPTION 'User already has a membership share';
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE CONSTRAINT TRIGGER membership_uniqueness_trigger
AFTER INSERT OR UPDATE ON user_shares
FOR EACH ROW EXECUTE FUNCTION validate_membership_uniqueness();
```

**Benefits**:
- Enforces constraint at database level
- Handles concurrent inserts safely
- Maintains referential integrity

## Integration Points

### cpay Integration
Processes external monetary transactions and ensures compliance.

### Wallet Integration
Manages fund allocation and provides transparency into fund utilization.

### Skill Volunteering Integration
Links volunteer contributions to verified opportunities.

## Usage

The system is accessed through a gRPC service defined in `proto/cooperative_fundraising.proto`.

## Testing

The package includes domain tests and integration tests to ensure correct behavior.

## Compliance

The system maintains strict separation between internal community participation and external monetary needs, ensuring all regulatory requirements are met while preserving cooperative principles.

## Critical Refinements Implemented

### Safe Campaign Deletion Workflow

Campaigns can only be deleted when:
1. Status is `DRAFT`
2. No contributions exist
3. No membership shares are associated

Implementation follows soft deletion pattern:
```rust
pub async fn delete_campaign(&self, id: Uuid) -> Result<(), ApplicationError> {
    let campaign = self.campaign_repository.find_by_id(id).await?
        .ok_or(ApplicationError::NotFound)?;
    
    // Safety checks
    if campaign.status != CampaignStatus::DRAFT {
        return Err(ApplicationError::ValidationFailed(
            "Only DRAFT campaigns can be deleted".to_string()
        ));
    }
    
    let has_contributions = self.contribution_repository
        .exists_for_campaign(id).await?;
    if has_contributions {
        return Err(ApplicationError::ValidationFailed(
            "Cannot delete campaign with contributions".to_string()
        ));
    }
    
    // Soft delete
    self.campaign_repository.soft_delete(id).await?;
    Ok(())
}
```

### gRPC Validation Improvements

Revised contribution model using `oneof` pattern:
```proto
message Contribution {
  string id = 1;
  string campaign_id = 2;
  string user_id = 3;
  google.protobuf.Timestamp created_at = 4;
  
  oneof contribution_type {
    MonetaryContribution monetary = 5;
    VolunteerContribution volunteer = 6;
  }
}

message MonetaryContribution {
  string amount = 1;  // REQUIRED
  string currency = 2;  // REQUIRED
  string cpay_transaction_id = 3;
}

message VolunteerContribution {
  string opportunity_id = 1;
  int32 hours = 2;
  VerificationStatus verification_status = 3;
}
```

### Integration Safety Protocols

**cpay Integration**:
- Idempotency keys required for all monetary transactions
- Retry logic with exponential backoff for network failures
- Transaction validation before contribution recording

**Error Handling Pattern**:
```rust
async fn process_payment(&self, request: PaymentRequest) -> Result<Contribution, FundraisingError> {
    match self.cpay_client.process_payment(request.clone()).await {
        Ok(response) => {
            // Verify transaction details match request
            if response.amount != request.amount || response.currency != request.currency {
                return Err(FundraisingError::PaymentVerificationFailed);
            }
            self.record_contribution(response).await
        }
        Err(e) => {
            if e.is_idempotency_violation() {
                // Handle duplicate request
                self.get_existing_contribution(request.idempotency_key).await
            } else {
                Err(FundraisingError::PaymentProcessingFailed(e))
            }
        }
    }
}
```

**Skill Volunteering Integration**:
- All volunteer contributions require pre-verified opportunity IDs
- Verification status transitions enforced through state machine
- Hour validation against opportunity parameters

**Wallet Integration**:
- Fund allocation requires campaign completion
- Transparent usage tracking with immutable ledger entries
- Reconciliation process for failed allocations

## Compliance

The system now includes enhanced validation to ensure:
- Membership constraints are technically enforced
- Historical data integrity through soft deletion
- Protocol-level validation of critical fields
- Safe handling of external integrations
- Idempotent transaction processing

These improvements ensure our cooperative values are not just conceptual but technically enforced at all system levels.