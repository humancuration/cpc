# Skill Development Domain Models

## Overview
Implements core domain entities from ADR-0005 and proto definitions. Models enforce business rules and validation.

## Dependencies
- `chrono` for date/time handling
- `uuid` for ID generation
- `validator` crate for validation

## Models

### SkillProgress
```rust
pub struct SkillProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub skill_id: Uuid,
    pub current_level: u8,
    pub target_level: u8,
    pub progress_percentage: f32,
    pub total_hours_invested: u16,
    pub last_practice_date: Option<NaiveDate>,
    // ... other fields from proto
}
```
- Validation: `current_level` <= `target_level`, `progress_percentage` between 0-100

### LearningPath
```rust
pub struct LearningPath {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub creator_id: Uuid,
    pub difficulty_level: u8,
    pub estimated_duration_hours: u16,
    pub resources: Vec<LearningResource>,
    pub prerequisites: Vec<Uuid>,
}
```
- Validation: Title length 5-100 chars, difficulty 1-5

### Certification
```rust
pub struct Certification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub skill_id: Uuid,
    pub certification_type: CertificationType,
    pub level_achieved: u8,
    pub issued_by: Uuid,
    pub issued_at: DateTime<Utc>,
    pub verification_code: String,
}

pub enum CertificationType {
    Basic,
    Advanced,
    Expert,
    CommunityBuilder,
}
```

## Validation Rules
- Implement using `validator::Validate` trait
- Custom validators for business rules
- Reject invalid data at domain boundary

## Related ADRs
- [ADR-0005](../adr/0005-skill-development-tracking.md)