# API Reference

This document provides technical documentation for the Unified Community Impact Dashboard's internal APIs and integration interfaces. While the dashboard primarily functions as a client-side application, it provides several internal APIs for component communication and data management.

## Overview

The Unified Community Impact Dashboard uses a component-based architecture with well-defined interfaces between different parts of the system. This documentation covers the public APIs exposed by various modules and services within the application.

## Data Models API

### Impact Domain Models

#### LearningDomain
Represents the learning impact domain and associated metrics.

```rust
pub struct LearningDomain {
    pub engagement_rate: f64,
    pub course_completion_rate: f64,
    pub skill_development: Vec<SkillMetric>,
    pub knowledge_sharing: KnowledgeSharingMetric,
    pub community_teaching: CommunityTeachingMetric,
}

pub struct SkillMetric {
    pub skill_name: String,
    pub proficiency_level: ProficiencyLevel,
    pub community_impact: f64,
}

pub struct KnowledgeSharingMetric {
    pub resources_shared: u32,
    pub collaborative_learning_sessions: u32,
    pub mentoring_relationships: u32,
}

pub struct CommunityTeachingMetric {
    pub workshops_led: u32,
    pub curriculum_contributions: u32,
    pub peer_teaching_hours: f64,
}
```

#### VolunteerDomain
Represents the volunteer impact domain and associated metrics.

```rust
pub struct VolunteerDomain {
    pub participation_rate: f64,
    pub retention_rate: f64,
    pub task_completion_rate: f64,
    pub leadership_opportunities: u32,
    pub community_initiatives: Vec<CommunityInitiative>,
}

pub struct CommunityInitiative {
    pub name: String,
    pub impact_score: f64,
    pub volunteer_hours: f64,
    pub community_reach: u32,
}
```

#### FinancialDomain
Represents the financial impact domain and associated metrics.

```rust
pub struct FinancialDomain {
    pub resource_distribution_score: f64,
    pub financial_health_indicator: FinancialHealth,
    pub sustainability_metrics: SustainabilityMetrics,
    pub community_investment: CommunityInvestment,
}

pub struct FinancialHealth {
    pub overall_score: f64,
    pub income_stability: f64,
    pub expense_management: f64,
    pub reserve_health: f64,
}

pub struct SustainabilityMetrics {
    pub long_term_viability: f64,
    pub resource_efficiency: f64,
    pub environmental_impact: f64,
}

pub struct CommunityInvestment {
    pub local_business_support: f64,
    pub community_project_funding: f64,
    pub member_financial_security: f64,
}
```

#### CauseDomain
Represents the cause advocacy impact domain and associated metrics.

```rust
pub struct CauseDomain {
    pub engagement_rate: f64,
    pub advocacy_effectiveness: f64,
    pub social_impact_score: f64,
    pub community_mobilization: CommunityMobilization,
    pub policy_influence: PolicyInfluence,
}

pub struct CommunityMobilization {
    pub awareness_campaigns: u32,
    pub participation_events: u32,
    pub collective_action_initiatives: u32,
}

pub struct PolicyInfluence {
    pub policy_proposals: u32,
    pub advocacy_success_rate: f64,
    pub legislative_impact: f64,
}
```

### Community Wellbeing Models

#### CommunityWellbeing
Represents overall community wellbeing and transformation metrics.

```rust
pub struct CommunityWellbeing {
    pub overall_score: f64,
    pub domain_scores: DomainScores,
    pub cooperative_goals_progress: Vec<GoalProgress>,
    pub historical_timeline: Vec<HistoricalWellbeing>,
}

pub struct DomainScores {
    pub learning: f64,
    pub volunteer: f64,
    pub financial: f64,
    pub cause: f64,
}

pub struct GoalProgress {
    pub goal_id: String,
    pub goal_name: String,
    pub target_value: f64,
    pub current_value: f64,
    pub progress_percentage: f64,
    pub estimated_completion: DateTime<Utc>,
}

pub struct HistoricalWellbeing {
    pub timestamp: DateTime<Utc>,
    pub overall_score: f64,
    pub domain_scores: DomainScores,
}
```

### Member Profile Models

#### MemberProfile
Represents an individual member's impact profile and position in the ecosystem.

```rust
pub struct MemberProfile {
    pub member_id: String,
    pub name: String,
    pub ecosystem_position: EcosystemPosition,
    pub impact_assessment: ImpactAssessment,
    pub personalized_suggestions: Vec<Suggestion>,
    pub impact_evolution: ImpactEvolution,
}

pub struct EcosystemPosition {
    pub domain_engagement: DomainEngagement,
    pub community_connections: Vec<CommunityConnection>,
    pub unique_contribution_patterns: Vec<ContributionPattern>,
}

pub struct DomainEngagement {
    pub learning: EngagementLevel,
    pub volunteer: EngagementLevel,
    pub financial: EngagementLevel,
    pub cause: EngagementLevel,
}

pub enum EngagementLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

pub struct CommunityConnection {
    pub connected_member_id: String,
    pub connection_strength: f64,
    pub shared_domains: Vec<Domain>,
}

pub struct ContributionPattern {
    pub pattern_type: PatternType,
    pub frequency: u32,
    pub impact_amplification: f64,
}

pub enum PatternType {
    CrossDomain,
    Consistent,
    Emerging,
    Leadership,
}

pub struct ImpactAssessment {
    pub quantity: ImpactQuantity,
    pub quality: ImpactQuality,
    pub reciprocity: ReciprocityScore,
    pub sustainability: SustainabilityScore,
}

pub struct ImpactQuantity {
    pub total_contributions: u32,
    pub time_invested: f64,
    pub resources_contributed: f64,
}

pub struct ImpactQuality {
    pub effectiveness_score: f64,
    pub innovation_score: f64,
    pub leadership_score: f64,
}

pub struct ReciprocityScore {
    pub score: f64,
    pub supporting_others: f64,
    pub receiving_support: f64,
}

pub struct SustainabilityScore {
    pub long_term_commitment: f64,
    pub pattern_consistency: f64,
    pub growth_trajectory: f64,
}

pub struct Suggestion {
    pub suggestion_id: String,
    pub title: String,
    pub description: String,
    pub recommended_action: RecommendedAction,
    pub impact_potential: f64,
}

pub enum RecommendedAction {
    EngageInDomain(Domain),
    ConnectWithMember(String),
    DevelopSkill(String),
    TakeLeadershipRole(String),
}

pub struct ImpactEvolution {
    pub milestones: Vec<Milestone>,
    pub pattern_recognition: Vec<Pattern>,
    pub predictive_suggestions: Vec<Suggestion>,
}

pub struct Milestone {
    pub milestone_id: String,
    pub title: String,
    pub description: String,
    pub achieved_date: DateTime<Utc>,
    pub celebration_status: CelebrationStatus,
}

pub enum CelebrationStatus {
    Pending,
    Celebrated,
    Acknowledged,
}

pub struct Pattern {
    pub pattern_id: String,
    pub pattern_type: PatternType,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}
```

## Services API

### Data Integration Service

The data integration service handles connectivity with all four impact systems.

#### Public Methods

```rust
impl DataIntegrationService {
    /// Fetches data from all impact systems
    pub async fn fetch_all_impact_data(&self) -> Result<ImpactData, IntegrationError> { ... }

    /// Fetches data for a specific domain
    pub async fn fetch_domain_data(&self, domain: Domain) -> Result<DomainData, IntegrationError> { ... }

    /// Synchronizes data with all systems
    pub async fn sync_all_data(&self) -> Result<SyncResult, IntegrationError> { ... }

    /// Checks the status of all data integrations
    pub async fn check_integration_status(&self) -> Result<IntegrationStatus, IntegrationError> { ... }
}
```

#### Data Structures

```rust
pub struct ImpactData {
    pub learning: LearningDomain,
    pub volunteer: VolunteerDomain,
    pub financial: FinancialDomain,
    pub cause: CauseDomain,
    pub timestamp: DateTime<Utc>,
    pub source_metadata: Vec<DataSourceMetadata>,
}

pub struct DomainData {
    pub domain: Domain,
    pub metrics: DomainMetrics,
    pub timestamp: DateTime<Utc>,
    pub source_metadata: DataSourceMetadata,
}

pub struct SyncResult {
    pub successful_syncs: u32,
    pub failed_syncs: u32,
    pub errors: Vec<SyncError>,
    pub timestamp: DateTime<Utc>,
}

pub struct IntegrationStatus {
    pub learning_system: SystemStatus,
    pub volunteer_system: SystemStatus,
    pub financial_system: SystemStatus,
    pub cause_system: SystemStatus,
    pub last_checked: DateTime<Utc>,
}

pub enum SystemStatus {
    Operational,
    Degraded,
    Offline,
    Maintenance,
}

pub struct DataSourceMetadata {
    pub source_id: String,
    pub source_name: String,
    pub last_updated: DateTime<Utc>,
    pub data_quality_score: f64,
}
```

### Visualization Service

The visualization service provides charting and graphing capabilities for impact data.

#### Public Methods

```rust
impl VisualizationService {
    /// Generates a circular flow diagram for interconnected impact
    pub fn generate_impact_flow_diagram(&self, data: &ImpactData) -> Result<FlowDiagram, VizError> { ... }

    /// Creates a wellbeing timeline chart
    pub fn create_wellbeing_timeline(&self, wellbeing: &CommunityWellbeing) -> Result<Chart, VizError> { ... }

    /// Generates domain comparison charts
    pub fn generate_domain_comparison(&self, data: &ImpactData) -> Result<Chart, VizError> { ... }

    /// Creates member impact visualization
    pub fn create_member_impact_viz(&self, profile: &MemberProfile) -> Result<MemberViz, VizError> { ... }
}
```

#### Data Structures

```rust
pub struct FlowDiagram {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub strength_indicators: Vec<StrengthIndicator>,
    pub evidence_markers: Vec<EvidenceMarker>,
}

pub struct Node {
    pub id: String,
    pub domain: Domain,
    pub position: (f64, f64),
    pub size: f64,
}

pub struct Connection {
    pub source: String,
    pub target: String,
    pub strength: f64,
    pub evidence_count: u32,
}

pub struct StrengthIndicator {
    pub connection_id: String,
    pub strength_level: StrengthLevel,
    pub trend: Trend,
}

pub enum StrengthLevel {
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}

pub struct EvidenceMarker {
    pub connection_id: String,
    pub evidence_type: EvidenceType,
    pub count: u32,
}

pub enum EvidenceType {
    Quantitative,
    Qualitative,
    Anecdotal,
    Research,
}

pub struct Chart {
    pub chart_type: ChartType,
    pub data: Vec<DataPoint>,
    pub axes: Axes,
    pub annotations: Vec<Annotation>,
}

pub enum ChartType {
    Line,
    Bar,
    Pie,
    Scatter,
    Area,
}

pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    pub label: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

pub struct Axes {
    pub x_axis: Axis,
    pub y_axis: Axis,
}

pub struct Axis {
    pub label: String,
    pub min: f64,
    pub max: f64,
    pub scale: ScaleType,
}

pub enum ScaleType {
    Linear,
    Logarithmic,
    Time,
}

pub struct Annotation {
    pub position: (f64, f64),
    pub text: String,
    pub style: AnnotationStyle,
}

pub struct AnnotationStyle {
    pub color: String,
    pub font_size: u32,
    pub background: Option<String>,
}
```

### Community Validation Service

The community validation service handles collaborative interpretation workflows.

#### Public Methods

```rust
impl CommunityValidationService {
    /// Starts a new collaborative interpretation session
    pub fn start_interpretation_session(&self, participants: Vec<MemberId>) -> Result<SessionId, ValidationError> { ... }

    /// Submits interpretation data for a session
    pub fn submit_interpretation_data(&self, session_id: SessionId, data: InterpretationData) -> Result<(), ValidationError> { ... }

    /// Retrieves validation outcomes for a session
    pub fn get_validation_outcomes(&self, session_id: SessionId) -> Result<ValidationOutcomes, ValidationError> { ... }

    /// Schedules a community reflection session
    pub fn schedule_reflection_session(&self, session_details: ReflectionSessionDetails) -> Result<SessionId, ValidationError> { ... }
}
```

#### Data Structures

```rust
pub struct InterpretationData {
    pub session_id: SessionId,
    pub step: InterpretationStep,
    pub participant_id: MemberId,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

pub enum InterpretationStep {
    DataReview,
    PatternIdentification,
    ContextualAnalysis,
    MeaningMaking,
    ActionPlanning,
}

pub struct ValidationOutcomes {
    pub session_id: SessionId,
    pub insights: Vec<Insight>,
    pub decisions: Vec<Decision>,
    pub action_items: Vec<ActionItem>,
    pub lessons_learned: Vec<Lesson>,
}

pub struct Insight {
    pub insight_id: String,
    pub description: String,
    pub supporting_evidence: Vec<Evidence>,
    pub confidence_level: ConfidenceLevel,
}

pub struct Evidence {
    pub evidence_id: String,
    pub evidence_type: EvidenceType,
    pub description: String,
    pub source: String,
}

pub enum ConfidenceLevel {
    Low,
    Medium,
    High,
}

pub struct Decision {
    pub decision_id: String,
    pub description: String,
    pub rationale: String,
    pub made_by: Vec<MemberId>,
    pub timestamp: DateTime<Utc>,
}

pub struct ActionItem {
    pub action_id: String,
    pub description: String,
    pub responsible_parties: Vec<MemberId>,
    pub due_date: DateTime<Utc>,
    pub priority: Priority,
    pub status: ActionStatus,
}

pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

pub enum ActionStatus {
    NotStarted,
    InProgress,
    Completed,
    Blocked,
}

pub struct Lesson {
    pub lesson_id: String,
    pub description: String,
    pub application_suggestions: Vec<String>,
    pub related_to: Vec<String>,
}

pub struct ReflectionSessionDetails {
    pub session_id: SessionId,
    pub title: String,
    pub description: String,
    pub scheduled_time: DateTime<Utc>,
    pub duration: Duration,
    pub participants: Vec<MemberId>,
    pub facilitator: MemberId,
    pub preparation_materials: Vec<Material>,
}

pub struct Material {
    pub material_id: String,
    pub title: String,
    pub description: String,
    pub url: Option<String>,
    pub file_path: Option<String>,
}
```

## Component APIs

### Dashboard Component

The main dashboard component orchestrates the overall application.

#### Properties

```rust
pub struct DashboardProps {
    pub initial_view: DashboardView,
    pub user_profile: UserProfile,
    pub community_settings: CommunitySettings,
    pub on_view_change: Callback<DashboardView>,
    pub on_user_action: Callback<UserAction>,
}
```

#### Methods

```rust
impl DashboardComponent {
    /// Switches to a different dashboard view
    pub fn switch_view(&self, view: DashboardView) { ... }

    /// Handles user navigation requests
    pub fn handle_navigation(&self, navigation: NavigationRequest) { ... }

    /// Processes user actions and updates state
    pub fn process_user_action(&self, action: UserAction) { ... }
}
```

#### Data Structures

```rust
pub enum DashboardView {
    Overview,
    MyImpact,
    CommunityStories,
    Validation,
    Settings,
}

pub struct UserProfile {
    pub member_id: String,
    pub name: String,
    pub role: UserRole,
    pub preferences: UserPreferences,
    pub permissions: Vec<Permission>,
}

pub enum UserRole {
    Member,
    Facilitator,
    Administrator,
    Guest,
}

pub struct UserPreferences {
    pub theme: Theme,
    pub language: Language,
    pub accessibility: AccessibilitySettings,
    pub notifications: NotificationSettings,
}

pub struct CommunitySettings {
    pub community_name: String,
    pub timezone: String,
    pub default_view: DashboardView,
    pub privacy_settings: PrivacySettings,
    pub integration_settings: IntegrationSettings,
}
```

### Visualization Components

#### ImpactVisualization Component

```rust
pub struct ImpactVisualizationProps {
    pub data: ImpactData,
    pub style: VisualizationStyle,
    pub on_domain_select: Callback<Domain>,
    pub on_connection_select: Callback<(Domain, Domain)>,
    pub accessibility_settings: AccessibilitySettings,
}

pub enum VisualizationStyle {
    Narrative,
    Comparative,
    TrendBased,
    Quantitative,
    Qualitative,
}

impl ImpactVisualization {
    /// Updates the visualization with new data
    pub fn update_data(&self, new_data: ImpactData) { ... }

    /// Changes the visualization style
    pub fn change_style(&self, new_style: VisualizationStyle) { ... }

    /// Handles domain selection events
    pub fn handle_domain_select(&self, domain: Domain) { ... }

    /// Handles connection selection events
    pub fn handle_connection_select(&self, source: Domain, target: Domain) { ... }
}
```

#### DomainCard Component

```rust
pub struct DomainCardProps {
    pub domain: Domain,
    pub metrics: DomainMetrics,
    pub wellbeing: WellbeingScore,
    pub on_select: Callback<Domain>,
    pub size: CardSize,
}

pub enum CardSize {
    Small,
    Medium,
    Large,
}

impl DomainCard {
    /// Handles card selection
    pub fn handle_select(&self) { ... }

    /// Updates displayed metrics
    pub fn update_metrics(&self, new_metrics: DomainMetrics) { ... }

    /// Animates wellbeing score changes
    pub fn animate_wellbeing_change(&self, old_score: f64, new_score: f64) { ... }
}
```

## Hooks and Utilities

### Data Fetching Hooks

#### useImpactData Hook

```rust
/// Custom hook for fetching and managing impact data
pub fn use_impact_data() -> UseImpactDataHandle {
    // Implementation details
}

pub struct UseImpactDataHandle {
    pub data: UseStateHandle<Option<ImpactData>>,
    pub loading: UseStateHandle<bool>,
    pub error: UseStateHandle<Option<String>>,
    pub refresh: Callback<()>,
    pub subscribe: Callback<Callback<ImpactData>>,
}
```

#### useMemberProfile Hook

```rust
/// Custom hook for fetching and managing member profile data
pub fn use_member_profile(member_id: &str) -> UseMemberProfileHandle {
    // Implementation details
}

pub struct UseMemberProfileHandle {
    pub profile: UseStateHandle<Option<MemberProfile>>,
    pub loading: UseStateHandle<bool>,
    pub error: UseStateHandle<Option<String>>,
    pub refresh: Callback<()>,
}
```

### State Management Utilities

#### Global State Management

```rust
/// Global application state
pub struct AppState {
    pub user: Option<UserProfile>,
    pub community: CommunitySettings,
    pub current_view: DashboardView,
    pub notifications: Vec<Notification>,
    pub pending_actions: Vec<PendingAction>,
}

/// Context provider for application state
pub struct AppContext {
    pub state: UseReducerHandle<AppState>,
    pub dispatch: UseCallback<ReducerAction>,
}

/// Actions that can be dispatched to update state
pub enum ReducerAction {
    SetUser(UserProfile),
    SetCommunity(CommunitySettings),
    ChangeView(DashboardView),
    AddNotification(Notification),
    RemoveNotification(NotificationId),
    AddPendingAction(PendingAction),
    RemovePendingAction(ActionId),
}
```

## Error Handling

### Common Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum DashboardError {
    #[error("Data integration error: {0}")]
    IntegrationError(#[from] IntegrationError),

    #[error("Visualization error: {0}")]
    VisualizationError(#[from] VizError),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationError),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Data parsing error: {0}")]
    ParsingError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    #[error("Connection failed to {system}: {reason}")]
    ConnectionFailed { system: String, reason: String },

    #[error("Data synchronization error: {details}")]
    SyncError { details: String },

    #[error("Authentication failed for {system}")]
    AuthFailed { system: String },

    #[error("Data quality issue: {description}")]
    DataQualityIssue { description: String },
}

#[derive(Debug, thiserror::Error)]
pub enum VizError {
    #[error("Chart generation failed: {reason}")]
    ChartGenerationFailed { reason: String },

    #[error("Invalid data format: {details}")]
    InvalidDataFormat { details: String },

    #[error("Rendering error: {description}")]
    RenderingError { description: String },
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Session not found: {session_id}")]
    SessionNotFound { session_id: String },

    #[error("Participant not authorized: {participant_id}")]
    UnauthorizedParticipant { participant_id: String },

    #[error("Invalid data submission: {reason}")]
    InvalidData { reason: String },

    #[error("Session already completed: {session_id}")]
    SessionCompleted { session_id: String },
}
```

## Testing APIs

### Test Utilities

```rust
/// Test data generators for unit tests
pub mod test_data {
    pub fn create_mock_impact_data() -> ImpactData { ... }
    pub fn create_mock_member_profile() -> MemberProfile { ... }
    pub fn create_mock_community_wellbeing() -> CommunityWellbeing { ... }
}

/// Test helpers for component testing
pub mod test_helpers {
    pub fn render_component_with_context<C>(component: C) -> RenderResult 
    where C: Component { ... }
    
    pub fn mock_service_dependencies() -> ServiceMocks { ... }
}

/// Mock service implementations for testing
pub mod mock_services {
    pub struct MockDataIntegrationService { ... }
    pub struct MockVisualizationService { ... }
    pub struct MockValidationService { ... }
}
```

## Performance Monitoring APIs

### Monitoring Service

```rust
/// Performance monitoring service
pub struct MonitoringService {
    pub fn record_render_time(&self, component: &str, duration: Duration) { ... }
    pub fn record_api_call(&self, endpoint: &str, duration: Duration, success: bool) { ... }
    pub fn record_user_interaction(&self, interaction: UserInteraction) { ... }
    pub fn get_performance_metrics(&self) -> PerformanceMetrics { ... }
}

pub struct UserInteraction {
    pub interaction_type: InteractionType,
    pub component: String,
    pub timestamp: DateTime<Utc>,
    pub duration: Option<Duration>,
}

pub enum InteractionType {
    Click,
    Hover,
    Scroll,
    Keyboard,
    Touch,
}

pub struct PerformanceMetrics {
    pub avg_render_time: HashMap<String, Duration>,
    pub api_performance: HashMap<String, ApiPerformance>,
    pub user_engagement: UserEngagementMetrics,
}

pub struct ApiPerformance {
    pub avg_response_time: Duration,
    pub success_rate: f64,
    pub error_count: u32,
}
```

## Conclusion

This API reference provides a comprehensive overview of the internal APIs and data structures used in the Unified Community Impact Dashboard. The component-based architecture with well-defined interfaces enables modularity, testability, and maintainability while supporting the dashboard's core mission of facilitating community-centered impact measurement and validation.

All APIs are designed with cooperative values in mind, emphasizing transparency, inclusivity, and collective benefit. The error handling system ensures graceful degradation and clear communication of issues to both users and developers.

For implementation details of specific components and services, please refer to the source code and other documentation files in the `docs/` directory.