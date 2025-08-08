//! Integration tests for the ML Core crate

use ml_core::*;

#[test]
fn test_ml_engine_creation() {
    let engine = MLEngine::new();
    assert!(true); // Creation should succeed
}

#[test]
fn test_engine_config_default() {
    let config = engine::EngineConfig::default();
    assert_eq!(config.max_cache_size, 100);
    assert!(config.enable_caching);
}

#[test]
fn test_cache_functionality() {
    let mut engine = MLEngine::new();
    
    // Cache should be empty initially
    assert_eq!(engine.cache_stats().0, 0);
    
    // Clearing empty cache should work
    engine.clear_cache();
    assert_eq!(engine.cache_stats().0, 0);
}

#[test]
fn test_cooperative_values_default() {
    let values = cooperative_values::CooperativeValues::default();
    assert!(values.enable_bias_detection);
    assert!(values.enable_privacy_preserving);
    assert!(values.enable_explainability);
    assert_eq!(values.community_impact_weight, 0.7);
}

#[test]
fn test_fairness_constraints_default() {
    let constraints = cooperative_values::FairnessConstraints::default();
    assert!(constraints.enable_demographic_parity);
    assert!(constraints.enable_equalized_odds);
    assert!(!constraints.protected_attributes.is_empty());
}

#[test]
fn test_privacy_config_default() {
    let config = privacy::PrivacyConfig::default();
    assert!(config.enable_differential_privacy);
    assert_eq!(config.epsilon, 1.0);
}

#[test]
fn test_bias_config_default() {
    let config = bias::BiasConfig::default();
    assert!(config.enable_detection);
    assert_eq!(config.bias_threshold, 0.1);
}

#[test]
fn test_explainability_config_default() {
    let config = explainability::ExplainabilityConfig::default();
    assert!(config.enable_explainability);
}

#[test]
fn test_evaluation_config_default() {
    let config = evaluation::EvaluationConfig::default();
    assert!(config.enable_community_impact);
    assert!(config.enable_cooperative_alignment);
}

#[test]
fn test_model_type_variants() {
    let volunteer_model = models::ModelType::VolunteerImpact;
    let financial_model = models::ModelType::FinancialTrend;
    let skill_model = models::ModelType::SkillDevelopment;
    let cause_model = models::ModelType::CauseImpact;
    
    assert!(matches!(volunteer_model, models::ModelType::VolunteerImpact));
    assert!(matches!(financial_model, models::ModelType::FinancialTrend));
    assert!(matches!(skill_model, models::ModelType::SkillDevelopment));
    assert!(matches!(cause_model, models::ModelType::CauseImpact));
}

#[test]
fn test_training_config_default() {
    let config = models::TrainingConfig::default();
    assert_eq!(config.max_iterations, 1000);
    assert_eq!(config.learning_rate, 0.01);
}

#[test]
fn test_create_volunteer_impact_model() {
    let engine = MLEngine::new();
    let model = engine.create_volunteer_impact_model();
    assert!(true); // Creation should succeed
}

#[test]
fn test_create_financial_trend_model() {
    let engine = MLEngine::new();
    let model = engine.create_financial_trend_model();
    assert!(true); // Creation should succeed
}

#[test]
fn test_create_skill_development_model() {
    let engine = MLEngine::new();
    let model = engine.create_skill_development_model();
    assert!(true); // Creation should succeed
}

#[test]
fn test_create_cause_impact_model() {
    let engine = MLEngine::new();
    let model = engine.create_cause_impact_model();
    assert!(true); // Creation should succeed
}

#[test]
fn test_bias_report_default() {
    let report = bias::BiasReport::default();
    assert_eq!(report.overall_bias, 0.0);
    assert!(report.bias_by_attribute.is_empty());
    assert!(report.recommendations.is_empty());
    assert!(!report.exceeds_threshold);
}

#[test]
fn test_explanation_detail_levels() {
    let high = explainability::ExplanationDetailLevel::High;
    let medium = explainability::ExplanationDetailLevel::Medium;
    let low = explainability::ExplanationDetailLevel::Low;
    
    assert!(matches!(high, explainability::ExplanationDetailLevel::High));
    assert!(matches!(medium, explainability::ExplanationDetailLevel::Medium));
    assert!(matches!(low, explainability::ExplanationDetailLevel::Low));
}

#[test]
fn test_traditional_metrics_default() {
    let metrics = evaluation::TraditionalMetrics::default();
    assert_eq!(metrics.accuracy, 0.0);
    assert_eq!(metrics.precision, 0.0);
    assert_eq!(metrics.recall, 0.0);
    assert_eq!(metrics.f1_score, 0.0);
    assert_eq!(metrics.auc_roc, 0.0);
}

#[test]
fn test_community_metrics_default() {
    let metrics = evaluation::CommunityMetrics::default();
    assert_eq!(metrics.community_benefit, 0.0);
    assert_eq!(metrics.equity_impact, 0.0);
    assert_eq!(metrics.accessibility_improvement, 0.0);
    assert_eq!(metrics.social_cohesion, 0.0);
}

#[test]
fn test_cooperative_metrics_default() {
    let metrics = evaluation::CooperativeMetrics::default();
    assert_eq!(metrics.transparency_score, 0.0);
    assert_eq!(metrics.fairness_score, 0.0);
    assert_eq!(metrics.participation_score, 0.0);
    assert_eq!(metrics.sustainability_impact, 0.0);
}

#[test]
fn test_bias_metrics_default() {
    let metrics = evaluation::BiasMetrics::default();
    assert_eq!(metrics.demographic_parity, 0.0);
    assert_eq!(metrics.equalized_odds, 0.0);
    assert_eq!(metrics.individual_fairness, 0.0);
}

#[test]
fn test_privacy_metrics_default() {
    let metrics = evaluation::PrivacyMetrics::default();
    assert_eq!(metrics.data_minimization, 0.0);
    assert_eq!(metrics.consent_compliance, 0.0);
    assert_eq!(metrics.anonymization_effectiveness, 0.0);
    assert_eq!(metrics.privacy_budget_usage, 0.0);
}