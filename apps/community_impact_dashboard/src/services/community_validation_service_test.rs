//! Tests for the Community Validation Service
//!
//! These tests verify that the community validation service functions correctly
//! and processes community validation workflows properly.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{UnifiedImpactData, community_validation::*};
    use crate::services::mock_data::create_sample_data;
    use uuid::Uuid;
    use chrono::Utc;
    
    #[test]
    fn test_process_interpretation() {
        // Create a community validation service
        let service = CommunityValidationService::new();
        
        // Create test data
        let impact_data = create_sample_data();
        
        // Create a community interpretation
        let interpretation = CommunityInterpretation {
            insights: vec![
                CommunityInsight {
                    id: Uuid::new_v4(),
                    description: "Learning programs build commitment to service".to_string(),
                    evidence: "Volunteer retention data shows 40% higher retention for participants who complete learning modules".to_string(),
                    contributors: vec!["Sarah M.".to_string(), "James T.".to_string()],
                    timestamp: Utc::now(),
                }
            ],
            patterns: vec![
                EmergingPattern {
                    description: "Learning → Volunteer Connection".to_string(),
                    domains: vec!["Learning".to_string(), "Volunteer".to_string()],
                    strength: PatternStrength::Moderate,
                    data_points: vec![
                        "Volunteer retention: 75% for learning participants vs 55% for non-participants".to_string(),
                        "Satisfaction index: 85% for learning participants vs 70% for non-participants".to_string(),
                    ],
                }
            ],
            consensus_areas: vec![
                ConsensusArea {
                    topic: "Learning Programs Are Effective".to_string(),
                    statement: "Our learning programs are successfully building knowledge and skills.".to_string(),
                    agreement_percentage: 0.92,
                    perspectives: vec![
                        "Programs provide practical skills".to_string(),
                        "Programs build community connections".to_string(),
                    ],
                }
            ],
            divergent_views: vec![
                DivergentView {
                    topic: "Approach to Financial Sustainability".to_string(),
                    perspectives: vec![
                        Perspective {
                            description: "Growth-Focused Perspective".to_string(),
                            holders: vec!["Finance Committee".to_string()],
                            rationale: "Expand membership and revenue streams".to_string(),
                        },
                        Perspective {
                            description: "Efficiency-Focused Perspective".to_string(),
                            holders: vec!["Operations Team".to_string()],
                            rationale: "Optimize current operations and reduce costs".to_string(),
                        }
                    ],
                    common_ground: vec![
                        "Financial sustainability is crucial".to_string(),
                        "Need to maintain cooperative values".to_string(),
                    ],
                }
            ],
            recommendations: vec![
                ActionableRecommendation {
                    description: "Create Integrated Learning-Volunteer Pathways".to_string(),
                    priority: RecommendationPriority::High,
                    expected_impact: "Increase volunteer retention by 15%".to_string(),
                    resources_needed: vec!["2 FTE".to_string(), "$5,000 budget".to_string()],
                    responsible_parties: vec!["Learning Coordinator".to_string(), "Volunteer Coordinator".to_string()],
                }
            ],
        };
        
        // Process the interpretation
        let result = service.process_interpretation(interpretation, &impact_data);
        
        // Verify the result
        assert!(result.is_ok());
        
        let processed = result.unwrap();
        assert_eq!(processed.processed_insights.len(), 1);
        assert_eq!(processed.processed_patterns.len(), 1);
        assert!(!processed.recommendations.is_empty());
    }
    
    #[test]
    fn test_process_reflection() {
        // Create a community validation service
        let service = CommunityValidationService::new();
        
        // Create test data
        let impact_data = create_sample_data();
        
        // Create a community reflection outcome
        let reflection = CommunityReflectionOutcome {
            insights: vec![
                CollectiveInsight {
                    id: Uuid::new_v4(),
                    description: "Our integrated approach strengthens community resilience".to_string(),
                    supporting_responses: vec![Uuid::new_v4(), Uuid::new_v4()],
                    emergence_pattern: EmergencePattern::Consensus,
                    timestamp: Utc::now(),
                }
            ],
            action_items: vec![
                ReflectionActionItem {
                    id: Uuid::new_v4(),
                    description: "Monthly Cross-Domain Coffee Chats".to_string(),
                    priority: ActionPriority::High,
                    assigned_to: vec!["Community Coordination Team".to_string()],
                    timeline: "Monthly starting next month".to_string(),
                    resources_needed: vec!["Virtual meeting platform".to_string()],
                    status: ActionStatus::Proposed,
                }
            ],
            participation_metrics: ParticipationMetrics {
                total_participants: 42,
                active_contributors: 31,
                response_diversity: 0.74,
            },
            emotional_climate: EmotionalClimate {
                dominant_emotions: vec![EmotionalTone::Hope, EmotionalTone::Gratitude, EmotionalTone::Challenge],
                emotional_balance: 0.68,
                emergent_themes: vec![
                    "Community Resilience".to_string(),
                    "Need for Connection".to_string(),
                    "Excitement for Future".to_string(),
                ],
            },
        };
        
        // Process the reflection
        let result = service.process_reflection(reflection, &impact_data);
        
        // Verify the result
        assert!(result.is_ok());
        
        let processed = result.unwrap();
        assert_eq!(processed.processed_insights.len(), 1);
        assert!(!processed.action_plan.items.is_empty());
    }
    
    #[test]
    fn test_save_documentation() {
        // Create a community validation service
        let service = CommunityValidationService::new();
        
        // Create test data
        let impact_data = create_sample_data();
        
        // Create a documentation record
        let documentation = CommunityDocumentationRecord {
            id: Uuid::new_v4(),
            title: "Q2 Community Interpretation: Learning-Volunteer Connection".to_string(),
            doc_type: DocumentationType::CommunityInterpretation,
            content: DocumentationContent {
                summary: "Community identified strong connection between learning programs and volunteer retention.".to_string(),
                details: "Through collaborative interpretation sessions, community members noted that participants who complete learning modules show 40% higher volunteer retention rates.".to_string(),
                insights: vec![
                    "Learning builds commitment to service".to_string(),
                    "Skills development strengthens community bonds".to_string(),
                ],
                evidence: vec![
                    SupportingEvidence {
                        description: "Volunteer retention data Q1-Q2 2025".to_string(),
                        evidence_type: EvidenceType::DataPoint,
                        source: "Community Impact Dashboard".to_string(),
                        link: None,
                    }
                ],
                related_documents: vec![],
            },
            authors: vec!["Community Interpretation Group".to_string()],
            impact_snapshot: Some(impact_data),
            tags: vec!["learning".to_string(), "volunteer".to_string(), "retention".to_string()],
            timestamp: Utc::now(),
            status: DocumentationStatus::Draft,
        };
        
        // Save the documentation
        let result = service.save_documentation(documentation);
        
        // Verify the result
        assert!(result.is_ok());
        
        let saved = result.unwrap();
        assert_eq!(saved.version, 1);
        assert_eq!(saved.status, DocumentationStatus::Draft);
    }
    
    #[test]
    fn test_validate_interpretation_with_empty_insights() {
        // Create a community validation service
        let service = CommunityValidationService::new();
        
        // Create an invalid interpretation (empty insights)
        let interpretation = CommunityInterpretation {
            insights: vec![], // Empty insights
            patterns: vec![
                EmergingPattern {
                    description: "Test Pattern".to_string(),
                    domains: vec!["Learning".to_string()],
                    strength: PatternStrength::Moderate,
                    data_points: vec!["Test data point".to_string()],
                }
            ],
            consensus_areas: vec![],
            divergent_views: vec![],
            recommendations: vec![],
        };
        
        // Create test data
        let impact_data = create_sample_data();
        
        // Try to process the interpretation
        let result = service.process_interpretation(interpretation, &impact_data);
        
        // Verify that it fails
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Interpretation must include insights");
    }
    
    #[test]
    fn test_validate_documentation_with_empty_title() {
        // Create a community validation service
        let service = CommunityValidationService::new();
        
        // Create an invalid documentation record (empty title)
        let documentation = CommunityDocumentationRecord {
            id: Uuid::new_v4(),
            title: String::new(), // Empty title
            doc_type: DocumentationType::CommunityInterpretation,
            content: DocumentationContent {
                summary: "Test summary".to_string(),
                details: "Test details".to_string(),
                insights: vec![],
                evidence: vec![],
                related_documents: vec![],
            },
            authors: vec!["Test Author".to_string()],
            impact_snapshot: None,
            tags: vec![],
            timestamp: Utc::now(),
            status: DocumentationStatus::Draft,
        };
        
        // Try to save the documentation
        let result = service.save_documentation(documentation);
        
        // Verify that it fails
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Documentation must have a title");
    }
}