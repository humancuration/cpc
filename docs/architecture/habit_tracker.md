# Habit Tracker Architecture Design

## Overview
This document specifies the architecture for the Habit Tracker feature, implemented as a vertical slice within `packages/cpc-core/health/`. The implementation follows our Hexagonal Architecture principles and integrates with existing HIPAA-compliant audit systems.

## Design Principles
- **Privacy by Default**: All health data operations include audit logging
- **Optional Data Sharing**: Users control research data donation through consent levels
- **Screaming Architecture**: Clear domain-driven directory structure
- **Vertical Slice Implementation**: Complete feature implementation across all layers

## Domain Layer (`domain/`)

### New Files
`domain/habit.rs`

```rust
//! Domain models for habit tracking

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::audit_log::{AuditPurpose, AccessType};

/// Core habit definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Habit {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub category: HabitCategory,
    pub target: HabitTarget,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Categories for grouping habits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HabitCategory {
    Wellness,
    Productivity,
    Mindfulness,
    Fitness,
    Custom(String),
}

/// Frequency targets for habit completion
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HabitTarget {
    Daily,
    Weekly(i32), // Number of completions per week
    Monthly(i32), // Number of completions per month
}

/// Record of a habit completion
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HabitCompletion {
    pub id: Uuid,
    pub habit_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub value: f32, // For quantitative habits (e.g., minutes meditated)
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Anonymized version for research sharing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnonymizedHabitCompletion {
    pub id: Uuid,
    pub habit_category: HabitCategory,
    pub timestamp: DateTime<Utc>,
    pub value: f32,
    pub created_at: DateTime<Utc>,
}

impl From<&HabitCompletion> for AnonymizedHabitCompletion {
    fn from(completion: &HabitCompletion) -> Self {
        Self {
            id: completion.id,
            habit_category: HabitCategory::Custom("Anonymized".to_string()), // Will be populated based on actual category
            timestamp: completion.timestamp,
            value: completion.value,
            created_at: completion.created_at,
        }
    }
}
```

## Application Layer (`application/`)

### New Files
`application/habit_service.rs`

```rust
//! Service layer for habit tracking operations

use crate::domain::habit::{Habit, HabitCompletion, HabitCategory, HabitTarget, AnonymizedHabitCompletion};
use crate::domain::audit_log::{AuditLog, AuditPurpose, AccessType};
use crate::infrastructure::database::repositories::{HabitRepository, HabitCompletionRepository};
use crate::infrastructure::p2p::data_sharing::{P2PManager, ConsentManager, ResearchSharingLevel};
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;
use chrono::Utc;
use tracing::error;

#[derive(Debug, Error)]
pub enum HabitError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid habit category: {0}")]
    InvalidCategory(String),
    #[error("Habit not found")]
    NotFound,
    #[error("Audit log error: {0}")]
    AuditLogError(String),
}

/// Main service for habit operations
pub struct HabitService {
    habit_repository: Box<dyn HabitRepository>,
    completion_repository: Box<dyn HabitCompletionRepository>,
    audit_service: Box<dyn AuditService>,
    p2p_manager: Box<dyn P2PManager>,
    consent_manager: ConsentManager,
}

impl HabitService {
    pub fn new(
        habit_repository: Box<dyn HabitRepository>,
        completion_repository: Box<dyn HabitCompletionRepository>,
        audit_service: Box<dyn AuditService>,
        p2p_manager: Box<dyn P2PManager>,
    ) -> Self {
        Self {
            habit_repository,
            completion_repository,
            audit_service,
            p2p_manager,
            consent_manager: ConsentManager::new(),
        }
    }

    /// Create a new habit with audit logging
    pub async fn create_habit(
        &self,
        user_id: Uuid,
        name: String,
        category: HabitCategory,
        target: HabitTarget,
    ) -> Result<Habit, HabitError> {
        // Audit logging for write operation
        self.audit_service.log_audit(
            Some(user_id),
            "Habit",
            Uuid::nil(), // Will be replaced with actual ID after creation
            AccessType::Write,
            AuditPurpose::UserView,
        ).await.map_err(|e| HabitError::AuditLogError(e.to_string()))?;

        let habit = Habit {
            id: Uuid::new_v4(),
            user_id,
            name,
            category,
            target,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.habit_repository.save(&habit, user_id, false)
            .await
            .map_err(|e| match e {
                HealthError::DatabaseError(e) => HabitError::DatabaseError(e),
                _ => HabitError::DatabaseError(sqlx::Error::RowNotFound),
            })?;

        // Update audit log with actual ID
        self.audit_service.log_audit(
            Some(user_id),
            "Habit",
            habit.id,
            AccessType::Write,
            AuditPurpose::UserView,
        ).await.map_err(|e| HabitError::AuditLogError(e.to_string()))?;

        Ok(habit)
    }

    /// Record a habit completion with optional research data sharing
    pub async fn record_completion(
        &self,
        habit_id: Uuid,
        value: f32,
        notes: Option<String>,
        user_id: Uuid,
    ) -> Result<HabitCompletion, HabitError> {
        // First verify the habit exists and belongs to the user
        let habit = self.habit_repository.find_by_id(habit_id, user_id, false)
            .await
            .map_err(|_| HabitError::NotFound)?;

        // Audit logging for write operation
        self.audit_service.log_audit(
            Some(user_id),
            "HabitCompletion",
            habit_id,
            AccessType::Write,
            AuditPurpose::UserView,
        ).await.map_err(|e| HabitError::AuditLogError(e.to_string()))?;

        let completion = HabitCompletion {
            id: Uuid::new_v4(),
            habit_id,
            timestamp: Utc::now(),
            value,
            notes,
            created_at: Utc::now(),
        };

        self.completion_repository.save(&completion, user_id, false)
            .await
            .map_err(|e| match e {
                HealthError::DatabaseError(e) => HabitError::DatabaseError(e),
                _ => HabitError::DatabaseError(sqlx::Error::RowNotFound),
            })?;

        // Check if user has consented to research data sharing
        if self.consent_manager.has_consent(&user_id) {
            match self.consent_manager.research_sharing_level(&user_id) {
                ResearchSharingLevel::AggregatedOnly => {
                    // Aggregation would happen elsewhere
                }
                ResearchSharingLevel::IndividualAnonymized => {
                    let anonymized = AnonymizedHabitCompletion::from(&completion);
                    // In real implementation, we'd map to actual category
                    let anonymized = AnonymizedHabitCompletion {
                        habit_category: habit.category.clone(),
                        ..anonymized
                    };
                    
                    // Trigger P2P sharing for research
                    if let Err(e) = self.p2p_manager.share_health_data(anonymized, user_id, false).await {
                        error!("Failed to share habit data for research: {}", e);
                    }
                }
                _ => {} // IndividualIdentifiable handled separately with explicit consent
            }
        }

        Ok(completion)
    }
}

/// Audit service abstraction
#[async_trait]
pub trait AuditService: Send + Sync {
    async fn log_audit(
        &self,
        user_id: Option<Uuid>,
        data_type: &str,
        data_id: Uuid,
        access_type: AccessType,
        purpose: AuditPurpose,
    ) -> Result<(), AuditError>;
}

/// Audit error types
#[derive(Debug, Error)]
pub enum AuditError {
    #[error("Audit logging failed: {0}")]
    LoggingFailed(String),
}
```

## Infrastructure Layer (`infrastructure/`)

### Database Implementation
`infrastructure/database/habit_repository.rs`

```rust
//! Database implementation for habit repositories

use crate::domain::habit::{Habit, HabitCompletion};
use crate::domain::audit_log::{AuditLog, AuditPurpose, AccessType};
use crate::infrastructure::database::audit_log_repository::AuditLogRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;
use tracing::error;

#[derive(Debug, Error)]
pub enum HealthError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Habit record not found")]
    NotFound,
    #[error("Audit log error: {0}")]
    AuditLogError(String),
}

/// Repository trait for habit operations
#[async_trait]
pub trait HabitRepository: Send + Sync {
    async fn save(&self, habit: &Habit, requester_id: Uuid, is_admin: bool) -> Result<(), HealthError>;
    async fn find_by_id(&self, id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<Habit, HealthError>;
    async fn find_by_user(&self, user_id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<Vec<Habit>, HealthError>;
}

/// Repository trait for habit completion operations
#[async_trait]
pub trait HabitCompletionRepository: Send + Sync {
    async fn save(&self, completion: &HabitCompletion, requester_id: Uuid, is_admin: bool) -> Result<(), HealthError>;
    async fn find_by_habit(&self, habit_id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<Vec<HabitCompletion>, HealthError>;
}

/// Implementation of HabitRepository for PostgreSQL
pub struct HabitRepositoryImpl {
    pool: PgPool,
    audit_log_repository: Box<dyn AuditLogRepository>,
}

impl HabitRepositoryImpl {
    pub fn new(pool: PgPool, audit_log_repository: Box<dyn AuditLogRepository>) -> Self {
        Self { pool, audit_log_repository }
    }
    
    /// Log an audit entry for habit access
    async fn log_audit(&self, user_id: Option<Uuid>, data_id: Uuid, access_type: AccessType, purpose: AuditPurpose) {
        let audit_log = AuditLog::new(
            user_id,
            "Habit",
            data_id,
            access_type.as_str(),
            purpose.as_str(),
            None,
            None,
        );
        
        if let Err(e) = self.audit_log_repository.create(audit_log).await {
            error!("Failed to create audit log: {}", e);
        }
    }
}

#[async_trait]
impl HabitRepository for HabitRepositoryImpl {
    async fn save(&self, habit: &Habit, requester_id: Uuid, is_admin: bool) -> Result<(), HealthError> {
        self.log_audit(
            Some(habit.user_id),
            habit.id,
            AccessType::Write,
            AuditPurpose::UserView,
        ).await;
        
        sqlx::query!(
            r#"
            INSERT INTO habits (
                id, user_id, name, category, target_frequency, 
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                user_id = EXCLUDED.user_id,
                name = EXCLUDED.name,
                category = EXCLUDED.category,
                target_frequency = EXCLUDED.target_frequency,
                updated_at = EXCLUDED.updated_at
            "#,
            habit.id,
            habit.user_id,
            habit.name,
            format!("{:?}", habit.category), // Enum conversion
            format!("{:?}", habit.target),   // Enum conversion
            habit.created_at,
            habit.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<Habit, HealthError> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, name, category, target_frequency, created_at, updated_at
            FROM habits
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        let row = match row {
            Some(row) => row,
            None => return Err(HealthError::NotFound),
        };
        
        self.log_audit(
            Some(row.user_id),
            row.id,
            AccessType::Read,
            AuditPurpose::UserView,
        ).await;
        
        // Convert string enums back to Rust enums
        let category = match row.category.as_str() {
            "Wellness" => HabitCategory::Wellness,
            "Productivity" => HabitCategory::Productivity,
            "Mindfulness" => HabitCategory::Mindfulness,
            "Fitness" => HabitCategory::Fitness,
            c => HabitCategory::Custom(c.to_string()),
        };
        
        let target = match row.target_frequency.as_str() {
            "Daily" => HabitTarget::Daily,
            t if t.starts_with("Weekly") => {
                let count = t.trim_start_matches("Weekly(").trim_end_matches(")").parse().unwrap_or(1);
                HabitTarget::Weekly(count)
            }
            t if t.starts_with("Monthly") => {
                let count = t.trim_start_matches("Monthly(").trim_end_matches(")").parse().unwrap_or(1);
                HabitTarget::Monthly(count)
            }
            _ => HabitTarget::Daily,
        };
        
        Ok(Habit {
            id: row.id,
            user_id: row.user_id,
            name: row.name,
            category,
            target,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
    
    async fn find_by_user(&self, user_id: Uuid, requester_id: Uuid, is_admin: bool) -> Result<Vec<Habit>, HealthError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, name, category, target_frequency, created_at, updated_at
            FROM habits
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        self.log_audit(
            Some(user_id),
            Uuid::nil(),
            AccessType::Read,
            AuditPurpose::UserView,
        ).await;
        
        let habits = rows.into_iter().map(|row| {
            // Same enum conversion as in find_by_id
            let category = match row.category.as_str() {
                "Wellness" => HabitCategory::Wellness,
                "Productivity" => HabitCategory::Productivity,
                "Mindfulness" => HabitCategory::Mindfulness,
                "Fitness" => HabitCategory::Fitness,
                c => HabitCategory::Custom(c.to_string()),
            };
            
            let target = match row.target_frequency.as_str() {
                "Daily" => HabitTarget::Daily,
                t if t.starts_with("Weekly") => {
                    let count = t.trim_start_matches("Weekly(").trim_end_matches(")").parse().unwrap_or(1);
                    HabitTarget::Weekly(count)
                }
                t if t.starts_with("Monthly") => {
                    let count = t.trim_start_matches("Monthly(").trim_end_matches(")").parse().unwrap_or(1);
                    HabitTarget::Monthly(count)
                }
                _ => HabitTarget::Daily,
            };
            
            Habit {
                id: row.id,
                user_id: row.user_id,
                name: row.name,
                category,
                target,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }
        }).collect();
        
        Ok(habits)
    }
}

// Similar implementation for HabitCompletionRepositoryImpl would follow
```

### Database Schema Changes
Add these tables to our PostgreSQL schema:

```sql
CREATE TABLE habits (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR(255) NOT NULL,
    category VARCHAR(50) NOT NULL,
    target_frequency VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE habit_completions (
    id UUID PRIMARY KEY,
    habit_id UUID NOT NULL REFERENCES habits(id),
    timestamp TIMESTAMPTZ NOT NULL,
    value FLOAT NOT NULL DEFAULT 1.0,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL
);
```

## Presentation Layer (`presentation/`)

### Yew Components
`presentation/yew/habit_components.rs`

```rust
//! Yew components for habit tracking UI

use yew::prelude::*;
use stylist::yew::styled_component;
use crate::domain::habit::{Habit, HabitCategory, HabitTarget};

#[derive(Properties, PartialEq)]
pub struct HabitFormProps {
    pub on_submit: Callback<HabitFormData>,
}

#[derive(Clone, PartialEq)]
pub struct HabitFormData {
    pub name: String,
    pub category: HabitCategory,
    pub target: HabitTarget,
}

#[styled_component(HabitCreationForm)]
pub fn habit_creation_form(props: &HabitFormProps) -> Html {
    let name = use_state(|| String::new());
    let category = use_state(|| HabitCategory::Wellness);
    let target = use_state(|| HabitTarget::Daily);
    
    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };
    
    let on_submit = {
        let name = name.clone();
        let category = category.clone();
        let target = target.clone();
        let on_submit = props.on_submit.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let form_data = HabitFormData {
                name: (*name).clone(),
                category: (*category).clone(),
                target: (*target).clone(),
            };
            
            on_submit.emit(form_data);
        })
    };
    
    html! {
        <form {on_submit}>
            <div class="form-group">
                <label for="habit-name">{"Habit Name"}</label>
                <input 
                    type="text" 
                    id="habit-name"
                    value={(*name).clone()}
                    oninput={on_name_change}
                    placeholder="e.g., Morning Meditation"
                    required=true
                />
            </div>
            
            <div class="form-group">
                <label for="habit-category">{"Category"}</label>
                <select id="habit-category">
                    <option value="wellness">{"Wellness"}</option>
                    <option value="productivity">{"Productivity"}</option>
                    <option value="mindfulness">{"Mindfulness"}</option>
                    <option value="fitness">{"Fitness"}</option>
                </select>
            </div>
            
            <div class="form-group">
                <label for="habit-target">{"Target Frequency"}</label>
                <select id="habit-target">
                    <option value="daily">{"Daily"}</option>
                    <option value="weekly">{"Weekly"}</option>
                    <option value="monthly">{"Monthly"}</option>
                </select>
            </div>
            
            <button type="submit">{"Create Habit"}</button>
        </form>
    }
}

#[derive(Properties, PartialEq)]
pub struct HabitCalendarProps {
    pub habit: Habit,
    pub completions: Vec<HabitCompletion>,
}

#[styled_component(HabitCalendarView)]
pub fn habit_calendar_view(props: &HabitCalendarProps) -> Html {
    // Implementation would render a calendar showing habit completions
    html! {
        <div class="habit-calendar">
            <h2>{&props.habit.name}</h2>
            // Calendar grid would be rendered here
        </div>
    }
}
```

### Bevy Visualization
`presentation/bevy/habit_viz.rs`

```rust
//! Bevy visualization for habit streaks and progress

use bevy::prelude::*;
use plotters::prelude::*;
use crate::domain::habit::{Habit, HabitCompletion};

pub struct HabitVisualizationPlugin;

impl Plugin for HabitVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, visualize_habit_streaks);
    }
}

fn visualize_habit_streaks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // System would create visualization entities
    // For actual implementation, would integrate with Plotters
}

/// Generate streak visualization for a habit
pub fn generate_streak_visualization(
    habit: &Habit,
    completions: &[HabitCompletion],
) -> Vec<u8> {
    // Create in-memory PNG of streak visualization
    let mut buffer = Vec::<u8>::new();
    
    {
        let root = BitMapBackend::with_buffer(&mut buffer, (800, 600)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        
        let min_date = completions.iter()
            .map(|c| c.timestamp.date_naive())
            .min()
            .unwrap_or_else(|| Utc::now().date_naive());
        
        let max_date = Utc::now().date_naive();
        
        let mut chart = ChartBuilder::on(&root)
            .caption(habit.name.clone(), ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(min_date..max_date, 0.0..1.5)
            .unwrap();
        
        chart.configure_mesh().draw().unwrap();
        
        // Plot completion streaks
        chart.draw_series(
            completions.iter().map(|c| {
                Circle::new(
                    (c.timestamp.date_naive(), 1.0),
                    5,
                    BLUE.filled(),
                )
            })
        ).unwrap();
        
        root.present().unwrap();
    }
    
    buffer
}
```

## Integration Points

### 1. Audit System Integration
- All habit operations log to audit system using `AuditPurpose::UserView`
- Research data sharing uses `AuditPurpose::Research` with `user_id: None`
- Follows existing pattern from `repositories.rs` (lines 64-80)

### 2. Data Sharing Integration
- Uses existing `P2PManager` implementation from `data_sharing.rs`
- Respects `ConsentManager` settings for research sharing
- Creates `AnonymizedHabitCompletion` similar to `AnonymizedVitalSign`

### 3. GraphQL API Integration
The Habit Tracker extends the existing health GraphQL API with:

```graphql
# In packages/cpc-core/health/src/presentation/graphql/habit_resolvers.rs

type Habit {
    id: ID!
    name: String!
    category: HabitCategory!
    target: HabitTarget!
    createdAt: DateTime!
    updatedAt: DateTime!
    completions: [HabitCompletion!]!
}

type HabitCompletion {
    id: ID!
    timestamp: DateTime!
    value: Float!
    notes: String
}

enum HabitCategory {
    Wellness
    Productivity
    Mindfulness
    Fitness
    Custom
}

enum HabitTarget {
    Daily
    Weekly
    Monthly
}

type Mutation {
    createHabit(input: CreateHabitInput!): Habit!
    recordHabitCompletion(input: RecordHabitCompletionInput!): HabitCompletion!
}

type Query {
    habits: [Habit!]!
    habit(id: ID!): Habit
}
```

### 4. Privacy Considerations
- Default behavior never shares identifiable data
- Research sharing requires explicit user consent
- All research data is properly anonymized per HIPAA requirements
- Audit logs track all data access including research sharing

## Next Steps
1. Implement the database schema changes
2. Create the domain, application, and infrastructure components
3. Build Yew and Bevy presentation layers
4. Extend GraphQL API with habit resolvers
5. Add to consent management system for habit-specific sharing preferences