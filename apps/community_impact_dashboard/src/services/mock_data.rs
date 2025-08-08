//! Mock data for testing the Unified Community Impact Dashboard
//!
//! This module provides sample data for testing dashboard components and services.

use crate::models::*;

/// Create sample unified impact data for testing
pub fn create_sample_data() -> UnifiedImpactData {
    UnifiedImpactData {
        id: uuid::Uuid::new_v4(),
        timestamp: chrono::Utc::now(),
        learning_metrics: create_sample_learning_metrics(),
        volunteer_metrics: create_sample_volunteer_metrics(),
        financial_metrics: create_sample_financial_metrics(),
        cause_metrics: create_sample_cause_metrics(),
        interconnections: create_sample_interconnections(),
        community_wellbeing: create_sample_community_wellbeing(),
        community_stories: create_sample_community_stories(),
        member_data: None,
    }
}

/// Create sample learning metrics for testing
fn create_sample_learning_metrics() -> learning_impact_tracker::domain::LearningMetrics {
    learning_impact_tracker::domain::LearningMetrics {
        engagement_rate: 0.75,
        course_completion_rate: 0.68,
        knowledge_sharing_index: 0.82,
        skill_development_progress: 0.71,
        community_satisfaction: 0.79,
        timestamp: chrono::Utc::now(),
    }
}

/// Create sample volunteer metrics for testing
fn create_sample_volunteer_metrics() -> volunteer_impact_tracker::domain::VolunteerMetrics {
    volunteer_impact_tracker::domain::VolunteerMetrics {
        participation_rate: 0.65,
        retention_rate: 0.72,
        task_completion_rate: 0.85,
        satisfaction_index: 0.78,
        community_impact_score: 0.74,
        timestamp: chrono::Utc::now(),
    }
}

/// Create sample financial metrics for testing
fn create_sample_financial_metrics() -> financial_impact_tracker::domain::FinancialMetrics {
    financial_impact_tracker::domain::FinancialMetrics {
        financial_health: 0.81,
        resource_equity: 0.69,
        sustainability_index: 0.73,
        community_investment: 0.77,
        resource_distribution: 0.75,
        timestamp: chrono::Utc::now(),
    }
}

/// Create sample cause metrics for testing
fn create_sample_cause_metrics() -> cause_impact_tracker::domain::CauseMetrics {
    cause_impact_tracker::domain::CauseMetrics {
        engagement_rate: 0.70,
        impact_effectiveness: 0.76,
        solidarity_index: 0.80,
        advocacy_participation: 0.67,
        social_change_indicators: 0.72,
        timestamp: chrono::Utc::now(),
    }
}

/// Create sample interconnections for testing
fn create_sample_interconnections() -> Vec<ImpactInterconnection> {
    vec![
        ImpactInterconnection {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            source_domain: "Learning".to_string(),
            target_domain: "Volunteer".to_string(),
            strength: 0.75,
            description: "Learning programs increase volunteer effectiveness".to_string(),
            evidence: vec![
                InterconnectionEvidence {
                    id: uuid::Uuid::new_v4(),
                    evidence_type: "DataCorrelation".to_string(),
                    description: "Volunteer retention 40% higher for learning participants".to_string(),
                    significance: 0.001,
                    confidence: 0.95,
                }
            ],
            values_alignment: "Reciprocity".to_string(),
        },
        ImpactInterconnection {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            source_domain: "Volunteer".to_string(),
            target_domain: "Financial".to_string(),
            strength: 0.68,
            description: "Volunteer service builds trust for financial participation".to_string(),
            evidence: vec![
                InterconnectionEvidence {
                    id: uuid::Uuid::new_v4(),
                    evidence_type: "SurveyData".to_string(),
                    description: "85% of volunteers report increased financial commitment".to_string(),
                    significance: 0.01,
                    confidence: 0.90,
                }
            ],
            values_alignment: "Trust".to_string(),
        },
    ]
}

/// Create sample community wellbeing for testing
fn create_sample_community_wellbeing() -> CommunityWellbeing {
    CommunityWellbeing {
        id: uuid::Uuid::new_v4(),
        timestamp: chrono::Utc::now(),
        overall_score: 0.74,
        domain_indicators: DomainWellbeingIndicators {
            learning: LearningWellbeing {
                knowledge_sharing_rate: 0.78,
                skill_development_progress: 0.71,
                community_satisfaction: 0.79,
                innovation_index: 0.65,
            },
            volunteer: VolunteerWellbeing {
                participation_rate: 0.65,
                retention_rate: 0.72,
                satisfaction_index: 0.78,
                community_impact_score: 0.74,
            },
            financial: FinancialWellbeing {
                financial_health: 0.81,
                resource_equity: 0.69,
                sustainability_index: 0.73,
                community_investment: 0.77,
            },
            cause: CauseWellbeing {
                engagement_rate: 0.70,
                impact_effectiveness: 0.76,
                solidarity_index: 0.80,
                advocacy_participation: 0.67,
            },
        },
        cooperative_goals_progress: vec![
            CooperativeGoalProgress {
                id: uuid::Uuid::new_v4(),
                title: "Increase Community Learning Engagement".to_string(),
                description: "Improve overall community learning engagement by 20%".to_string(),
                progress: 0.65,
                target_date: chrono::Utc::now() + chrono::Duration::days(365),
                values_alignment: "Growth".to_string(),
            },
            CooperativeGoalProgress {
                id: uuid::Uuid::new_v4(),
                title: "Strengthen Volunteer Network".to_string(),
                description: "Expand our volunteer network by 30%".to_string(),
                progress: 0.58,
                target_date: chrono::Utc::now() + chrono::Duration::days(365),
                values_alignment: "Community".to_string(),
            },
        ],
        historical_progress: vec![],
        comparative_metrics: vec![],
    }
}

/// Create sample community stories for testing
fn create_sample_community_stories() -> Vec<ImpactStory> {
    vec![
        ImpactStory {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            title: "From Student to Mentor: Sarah's Journey".to_string(),
            narrative: "Sarah joined our learning program six months ago and has since become one of our most active volunteers, mentoring new members and contributing to our financial sustainability through her increased participation.".to_string(),
            author: "Sarah M.".to_string(),
            metrics: vec![],
            testimonials: vec![
                "Sarah's journey shows how our integrated approach works - learning led to volunteering which led to financial participation.".to_string()
            ],
            visual_elements: vec![],
            tags: vec!["learning".to_string(), "volunteer".to_string(), "financial".to_string()],
            values_demonstrated: vec!["Growth".to_string(), "Reciprocity".to_string()],
            community_validated: true,
            reaction_count: 24,
            featured: true,
        }
    ]
}