//! Community Validation Data Models
//!
//! This module defines data structures for community validation workflows
//! including collaborative interpretation, reflection sessions, and documentation.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Community interpretation structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityInterpretation {
    /// Key insights from the community
    pub insights: Vec<CommunityInsight>,
    
    /// Emerging patterns
    pub patterns: Vec<EmergingPattern>,
    
    /// Consensus areas
    pub consensus_areas: Vec<ConsensusArea>,
    
    /// Divergent perspectives
    pub divergent_views: Vec<DivergentView>,
    
    /// Actionable recommendations
    pub recommendations: Vec<ActionableRecommendation>,
}

/// Community insight structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityInsight {
    /// Unique identifier
    pub id: Uuid,
    
    /// Insight description
    pub description: String,
    
    /// Supporting evidence
    pub evidence: String,
    
    /// Community members who contributed
    pub contributors: Vec<String>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Emerging pattern structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergingPattern {
    /// Pattern description
    pub description: String,
    
    /// Domains involved
    pub domains: Vec<String>,
    
    /// Strength of pattern
    pub strength: PatternStrength,
    
    /// Supporting data points
    pub data_points: Vec<String>,
}

/// Pattern strength levels
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum PatternStrength {
    Weak,
    Moderate,
    Strong,
}

/// Consensus area structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsensusArea {
    /// Topic of consensus
    pub topic: String,
    
    /// Consensus statement
    pub statement: String,
    
    /// Percentage agreement
    pub agreement_percentage: f64,
    
    /// Supporting perspectives
    pub perspectives: Vec<String>,
}

/// Divergent view structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct DivergentView {
    /// Topic of divergence
    pub topic: String,
    
    /// Different perspectives
    pub perspectives: Vec<Perspective>,
    
    /// Areas of common ground
    pub common_ground: Vec<String>,
}

/// Perspective structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Perspective {
    /// Description of perspective
    pub description: String,
    
    /// Community members holding this view
    pub holders: Vec<String>,
    
    /// Supporting rationale
    pub rationale: String,
}

/// Actionable recommendation structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionableRecommendation {
    /// Recommendation description
    pub description: String,
    
    /// Priority level
    pub priority: RecommendationPriority,
    
    /// Expected impact
    pub expected_impact: String,
    
    /// Resources needed
    pub resources_needed: Vec<String>,
    
    /// Responsible parties
    pub responsible_parties: Vec<String>,
}

/// Recommendation priority levels
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Community reflection outcome structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityReflectionOutcome {
    /// Key insights
    pub insights: Vec<CollectiveInsight>,
    
    /// Action items
    pub action_items: Vec<ReflectionActionItem>,
    
    /// Participation metrics
    pub participation_metrics: ParticipationMetrics,
    
    /// Emotional climate
    pub emotional_climate: EmotionalClimate,
}

/// Collective insight structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectiveInsight {
    /// Unique identifier
    pub id: Uuid,
    
    /// Insight description
    pub description: String,
    
    /// Supporting responses
    pub supporting_responses: Vec<Uuid>,
    
    /// Emergence pattern
    pub emergence_pattern: EmergencePattern,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Emergence patterns
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum EmergencePattern {
    Consensus,
    Tension,
    Innovation,
    Healing,
    Transformation,
}

/// Action item from reflection
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ReflectionActionItem {
    /// Unique identifier
    pub id: Uuid,
    
    /// Action description
    pub description: String,
    
    /// Priority level
    pub priority: ActionPriority,
    
    /// Assigned to
    pub assigned_to: Vec<String>,
    
    /// Timeline
    pub timeline: String,
    
    /// Resources needed
    pub resources_needed: Vec<String>,
    
    /// Status
    pub status: ActionStatus,
}

/// Action priority levels
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Action status
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionStatus {
    Proposed,
    Approved,
    InProgress,
    Completed,
    Deferred,
}

/// Participation metrics
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticipationMetrics {
    /// Total participants
    pub total_participants: usize,
    
    /// Active contributors
    pub active_contributors: usize,
    
    /// Response diversity
    pub response_diversity: f64,
}

/// Emotional climate
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct EmotionalClimate {
    /// Dominant emotions
    pub dominant_emotions: Vec<EmotionalTone>,
    
    /// Emotional balance
    pub emotional_balance: f64,
    
    /// Emergent themes
    pub emergent_themes: Vec<String>,
}

/// Emotional tone of a response
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum EmotionalTone {
    Joy,
    Gratitude,
    Concern,
    Challenge,
    Hope,
    Reflection,
}

/// Community documentation record structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityDocumentationRecord {
    /// Unique identifier
    pub id: Uuid,
    
    /// Title of the documentation
    pub title: String,
    
    /// Type of documentation
    pub doc_type: DocumentationType,
    
    /// Content of the documentation
    pub content: DocumentationContent,
    
    /// Authors/contributors
    pub authors: Vec<String>,
    
    /// Related impact data snapshot
    pub impact_snapshot: Option<UnifiedImpactData>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Status
    pub status: DocumentationStatus,
}

/// Documentation types
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum DocumentationType {
    CommunityInterpretation,
    ReflectionOutcome,
    Recommendation,
    ActionItem,
    LearningResource,
    BestPractice,
}

/// Documentation content structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentationContent {
    /// Summary of the documentation
    pub summary: String,
    
    /// Detailed content
    pub details: String,
    
    /// Key insights
    pub insights: Vec<String>,
    
    /// Supporting evidence
    pub evidence: Vec<SupportingEvidence>,
    
    /// Related documents
    pub related_documents: Vec<Uuid>,
}

/// Supporting evidence structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportingEvidence {
    /// Evidence description
    pub description: String,
    
    /// Evidence type
    pub evidence_type: EvidenceType,
    
    /// Source
    pub source: String,
    
    /// Link to source (if applicable)
    pub link: Option<String>,
}

/// Evidence types
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum EvidenceType {
    DataPoint,
    CommunityFeedback,
    Observation,
    Research,
    CaseStudy,
}

/// Documentation status
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum DocumentationStatus {
    Draft,
    Review,
    Published,
    Archived,
}

// Processed data structures for service layer

/// Processed interpretation structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessedInterpretation {
    /// Unique identifier
    pub id: Uuid,
    
    /// Original interpretation
    pub original_interpretation: CommunityInterpretation,
    
    /// Processed insights
    pub processed_insights: Vec<ProcessedInsight>,
    
    /// Processed patterns
    pub processed_patterns: Vec<ProcessedPattern>,
    
    /// Consensus analysis
    pub consensus_analysis: ConsensusAnalysis,
    
    /// Generated recommendations
    pub recommendations: Vec<GeneratedRecommendation>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Processed insight structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessedInsight {
    /// Unique identifier
    pub id: Uuid,
    
    /// Insight description
    pub description: String,
    
    /// Linked evidence
    pub evidence: Vec<EvidenceLink>,
    
    /// Community members who contributed
    pub contributors: Vec<String>,
    
    /// Impact domains related to this insight
    pub impact_domains: Vec<String>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Evidence link structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceLink {
    /// Description of the evidence
    pub description: String,
    
    /// Type of evidence
    pub evidence_type: EvidenceType,
    
    /// Link to data source
    pub data_source: String,
    
    /// Confidence level
    pub confidence: f64,
}

/// Processed pattern structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessedPattern {
    /// Pattern description
    pub description: String,
    
    /// Domains involved
    pub domains: Vec<String>,
    
    /// Strength of pattern
    pub strength: PatternStrength,
    
    /// Validated data points
    pub data_points: Vec<ValidatedDataPoint>,
    
    /// Impact correlation analysis
    pub impact_correlation: ImpactCorrelation,
}

/// Validated data point structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatedDataPoint {
    /// Original data point description
    pub original: String,
    
    /// Whether the data point was validated
    pub validated: bool,
    
    /// Source of validation
    pub source: String,
}

/// Impact correlation structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpactCorrelation {
    /// Correlation coefficient between domains
    pub correlation_coefficient: f64,
    
    /// Statistical significance of correlation
    pub statistical_significance: f64,
    
    /// Sample size used for correlation calculation
    pub sample_size: usize,
}

/// Consensus analysis structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsensusAnalysis {
    /// Areas of consensus
    pub consensus_areas: Vec<ConsensusArea>,
    
    /// Divergent views
    pub divergent_views: Vec<DivergentView>,
    
    /// Overall consensus strength
    pub consensus_strength: f64,
}

/// Generated recommendation structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratedRecommendation {
    /// Recommendation description
    pub description: String,
    
    /// Priority level
    pub priority: RecommendationPriority,
    
    /// Expected impact
    pub expected_impact: String,
    
    /// Resources needed
    pub resources_needed: Vec<String>,
    
    /// Responsible parties
    pub responsible_parties: Vec<String>,
    
    /// Confidence level
    pub confidence: f64,
}

/// Processed reflection structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessedReflection {
    /// Unique identifier
    pub id: Uuid,
    
    /// Original reflection
    pub original_reflection: CommunityReflectionOutcome,
    
    /// Processed insights
    pub processed_insights: Vec<ProcessedReflectionInsight>,
    
    /// Action plan
    pub action_plan: ActionPlan,
    
    /// Emotional climate analysis
    pub emotional_climate_analysis: EmotionalClimateAnalysis,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Processed reflection insight structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessedReflectionInsight {
    /// Unique identifier
    pub id: Uuid,
    
    /// Insight description
    pub description: String,
    
    /// Supporting responses
    pub supporting_responses: Vec<Uuid>,
    
    /// Emergence pattern
    pub emergence_pattern: EmergencePattern,
    
    /// Impact domains related to this insight
    pub impact_domains: Vec<String>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Action plan structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionPlan {
    /// Action items
    pub items: Vec<ReflectionActionItem>,
    
    /// Overall timeline
    pub timeline: String,
    
    /// Resources needed for the plan
    pub resources_needed: Vec<String>,
}

/// Emotional climate analysis structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct EmotionalClimateAnalysis {
    /// Dominant emotions
    pub dominant_emotions: Vec<EmotionalTone>,
    
    /// Emotional balance
    pub emotional_balance: f64,
    
    /// Emergent themes
    pub emergent_themes: Vec<String>,
    
    /// Sentiment trend
    pub sentiment_trend: SentimentTrend,
}

/// Sentiment trend
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum SentimentTrend {
    Improving,
    Declining,
    Stable,
    Volatile,
}

/// Saved documentation structure
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedDocumentation {
    /// Unique identifier
    pub id: Uuid,
    
    /// Title of the documentation
    pub title: String,
    
    /// Type of documentation
    pub doc_type: DocumentationType,
    
    /// Content of the documentation
    pub content: DocumentationContent,
    
    /// Authors/contributors
    pub authors: Vec<String>,
    
    /// Related impact data snapshot
    pub impact_snapshot: Option<UnifiedImpactData>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Creation timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Status
    pub status: DocumentationStatus,
    
    /// Version number
    pub version: u32,
    
    /// When the documentation was saved
    pub saved_timestamp: DateTime<Utc>,
}

// We need to import UnifiedImpactData for the documentation record
// In a real implementation, this would be handled differently to avoid circular dependencies
use crate::models::UnifiedImpactData;