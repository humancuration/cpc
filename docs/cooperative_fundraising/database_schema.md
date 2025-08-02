# Database Schema: Cooperative Fundraising System

## Core Tables

### 1. campaigns
```sql
CREATE TABLE campaigns (
    id UUID PRIMARY KEY,
    type campaign_type NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    owner_user_id UUID NOT NULL, -- Cooperative admin
    status campaign_status NOT NULL DEFAULT 'draft'
);

-- Campaign types reflect community vs regulatory needs
CREATE TYPE campaign_type AS ENUM (
    'cooperative_membership', -- Pure participation-based
    'pure_donation',          -- GoFundMe style
    'reg_cf',                 -- SEC Regulation Crowdfunding
    'reg_a',                  -- SEC Regulation A+
    'reg_d'                   -- SEC Regulation D
);

CREATE TYPE campaign_status AS ENUM (
    'draft', 'active', 'completed', 'failed', 'cancelled'
);
```

### 2. membership_requirements (for cooperative_membership campaigns)
```sql
CREATE TABLE membership_requirements (
    campaign_id UUID PRIMARY KEY REFERENCES campaigns(id),
    max_participants INTEGER,
    required_actions JSONB NOT NULL
    -- Example: [
    --   {"type": "attend_meeting", "count": 1},
    --   {"type": "complete_training", "module": "governance"}
    -- ]
);
```

### 3. donation_campaigns (for pure_donation/reg_cf/reg_a)
```sql
CREATE TABLE donation_campaigns (
    campaign_id UUID PRIMARY KEY REFERENCES campaigns(id),
    funding_goal NUMERIC(18,2),
    external_use_case TEXT NOT NULL,
    currency VARCHAR(3) DEFAULT 'USD'
);
```

### 4. user_shares
```sql
CREATE TABLE user_shares (
    user_id UUID NOT NULL,
    campaign_id UUID NOT NULL REFERENCES campaigns(id),
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id, campaign_id),
    
    -- CRITICAL: Enforce 1 membership share per person globally
    EXCLUDE USING btree (user_id WITH =) WHERE (campaigns.type = 'cooperative_membership')
);
```

### 5. contributions
```sql
CREATE TABLE contributions (
    id UUID PRIMARY KEY,
    campaign_id UUID NOT NULL REFERENCES campaigns(id),
    user_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Monetary contributions (for donation campaigns only)
    cpay_transaction_id UUID, -- Links to cpay system
    amount NUMERIC(18,2),
    currency VARCHAR(3),
    
    -- Volunteer actions (for ALL campaign types)
    opportunity_id UUID, -- Links to skill_volunteering opportunities
    hours INTEGER,
    verification_status verification_status NOT NULL DEFAULT 'pending'
);

CREATE TYPE verification_status AS ENUM (
    'pending', 'verified', 'disputed', 'rejected'
);
```

## Integration Points with Existing Systems

### 1. skill_volunteering Integration
```sql
-- Foreign key to skill_volunteering opportunities
ALTER TABLE contributions 
ADD CONSTRAINT fk_opportunity
FOREIGN KEY (opportunity_id) 
REFERENCES skill_volunteering.opportunities(id)
ON DELETE CASCADE;
```

### 2. cpay Integration
```sql
-- Foreign key to cpay transactions (hypothetical structure)
ALTER TABLE contributions 
ADD CONSTRAINT fk_cpay
FOREIGN KEY (cpay_transaction_id) 
REFERENCES cpay.transactions(id);
```

## Critical Constraints

1. **Membership Uniqueness**:
```sql
-- Enforced at database level for cooperative_membership campaigns
EXCLUDE USING btree (user_id WITH =) 
WHERE (campaigns.type = 'cooperative_membership')
```

2. **Contribution Type Validation**:
```sql
-- For donation campaigns: must have monetary data
CREATE CONSTRAINT TRIGGER donation_contributions_check
AFTER INSERT OR UPDATE ON contributions
FOR EACH ROW
WHEN (NEW.campaign_id IN (SELECT id FROM donation_campaigns))
EXECUTE FUNCTION validate_donation_contribution();

-- For membership campaigns: must have volunteer action
CREATE CONSTRAINT TRIGGER membership_contributions_check
AFTER INSERT OR UPDATE ON contributions
FOR EACH ROW
WHEN (NEW.campaign_id IN (SELECT id FROM membership_requirements))
EXECUTE FUNCTION validate_membership_contribution();
```

## Rationale

1. **Strict Separation of Concerns**:
   - Participation-based membership (cooperative_membership) has NO monetary fields
   - Donation campaigns exist solely for external compliance needs
   - Volunteer actions tracked WITHOUT monetary valuation

2. **Enforced Cooperative Principles**:
   - Database-level constraint ensures 1 membership share per person
   - No financial value stored for volunteer hours (hours field is purely for tracking participation)
   - Clear separation between internal community participation and external monetary needs

3. **Regulatory Flexibility**:
   - Dedicated tables for regulatory campaigns (reg_cf, reg_a)
   - [TODO: Regulatory] placeholders for future compliance details
   - External use case requirement ensures transparency for monetary donations