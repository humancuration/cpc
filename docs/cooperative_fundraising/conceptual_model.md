# Revised Conceptual Model: Cooperative Fundraising System

## Critical Clarification
Per user guidance: **All labor within the federation has no cash value**. Volunteer hours are pure community participation, not "sweat equity" with monetary valuation. Monetary fundraising exists **only as necessary interface with external systems** (legal/compliance requirements).

## Core Principles
1. **Membership = Participation**: Shares represent community involvement, not financial investment
2. **Labor ≠ Currency**: Volunteer hours have no dollar value within the federation
3. **Money as Tool**: External currency is only used where absolutely necessary (legal/compliance)
4. **1 Person = 1 Vote**: Strict democratic structure regardless of contribution type

## Campaign Types
```rust
enum CampaignType {
    /// Pure community participation drive (no monetary aspect)
    /// Grants 1 membership share per participant (max 1 per person)
    CooperativeMembership {
        max_participants: Option<u32>,
        required_actions: Vec<ActionType> // e.g., attend meeting, complete onboarding
    },
    
    /// Pure donation drive (no membership implications)
    /// Like GoFundMe - funds specific external needs
    PureDonation {
        funding_goal: Option<f64>,
        external_use_case: String // Required for compliance
    },
    
    /// SEC Regulation Crowdfunding (Reg CF)
    /// [TODO: Regulatory] - External compliance only
    RegCF {
        // Regulatory-specific parameters
    },
    
    /// SEC Regulation A+ (Reg A)
    /// [TODO: Regulatory] - External compliance only
    RegA {
        // Regulatory-specific parameters
    },

    /// SEC Regulation D (Reg D)
    /// [TODO: Regulatory] - External compliance only
    RegD {
        // Regulatory-specific parameters
    }
}
```

## Contribution Types
```rust
enum Contribution {
    /// Monetary donation for external-facing needs
    /// Only relevant for PureDonation/RegCF/RegA campaigns
    Monetary {
        amount: f64,
        currency: String,
        campaign_id: Uuid,
        cpay_transaction_id: Option<Uuid> // Linked to cpay
    },
    
    /// Volunteer action within the community
    /// Relevant for ALL campaign types (tracks participation)
    VolunteerAction {
        campaign_id: Uuid,
        opportunity_id: Uuid, // Links to skill_volunteering opportunities
        hours: u32,
        skill_id: Uuid,      // From skill_volunteering system
        verification_status: VerificationStatus
    }
}
```

## Membership Model
```rust
struct Membership {
    user_id: Uuid,
    campaign_id: Uuid,
    join_date: DateTime<Utc>,
    // NO financial value - represents community participation
    // 1 per person enforced at database level
}
```

## Key Relationships
1. **Campaigns** can be:
   - Membership drives (CooperativeMembership type)
   - Fundraising drives (other types)
   
2. **VolunteerActions** always link to:
   - skill_volunteering opportunities
   - User's skill profile (via skill_id)

3. **Monetary contributions** ONLY exist for:
   - External compliance requirements
   - Necessary interfacing with outside society

## How This Aligns With Cooperative Principles
| Principle | Implementation |
|-----------|---------------|
| One person, one vote | Strict 1 membership share per person |
| No financial returns | Volunteer hours have no cash value |
| Community-first | Membership based on participation, not contribution value |
| Transparency | All contributions (time/money) fully tracked and visible |

This model maintains our core ethos while providing necessary interfaces with external regulatory requirements.