//! Example: Cooperative values integration in ML operations

use ml_core::MLEngine;
use ml_core::cooperative_values::{CooperativeValues, FairnessConstraints};
use ml_core::privacy::PrivacyPreserver;
use ml_core::bias::BiasDetector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cooperative Values Integration Example");
    println!("=====================================");
    
    // Create cooperative values configuration
    let fairness_constraints = FairnessConstraints {
        enable_demographic_parity: true,
        enable_equalized_odds: true,
        enable_individual_fairness: true,
        protected_attributes: vec![
            "age".to_string(),
            "gender".to_string(),
            "race".to_string(),
        ],
    };
    
    let cooperative_values = CooperativeValues {
        enable_bias_detection: true,
        enable_privacy_preserving: true,
        enable_explainability: true,
        enable_community_validation: true,
        fairness_constraints,
        community_impact_weight: 0.8,
        ..Default::default()
    };
    
    // Create ML engine with cooperative values
    let engine = MLEngine::with_config(cooperative_values.into());
    
    // Example: Privacy-preserving data processing
    println!("\n1. Applying privacy-preserving techniques...");
    let privacy_preserver = PrivacyPreserver::new();
    let sensitive_data = vec![0.8, 0.7, 0.9, 0.6, 0.85]; // Placeholder data
    let private_data = privacy_preserver.apply_differential_privacy(&sensitive_data)?;
    println!("   Applied differential privacy to data");
    
    // Example: Bias detection
    println!("\n2. Detecting bias in predictions...");
    let bias_detector = BiasDetector::new(cooperative_values);
    let predictions = vec![0.8, 0.7, 0.9, 0.6, 0.85]; // Placeholder data
    let protected_attributes = vec![1.0, 0.0, 1.0, 0.0, 1.0]; // Placeholder data
    let bias_report = bias_detector.detect_bias(&predictions, &protected_attributes)?;
    println!("   Overall bias score: {:.2}", bias_report.overall_bias);
    println!("   Bias exceeds threshold: {}", bias_report.exceeds_threshold);
    
    // Example: Bias mitigation
    println!("\n3. Mitigating bias in predictions...");
    let fair_predictions = bias_detector.mitigate_bias(&predictions)?;
    println!("   Applied bias mitigation techniques");
    
    // Example: Explanation generation
    println!("\n4. Generating explanation with cooperative context...");
    let features = std::collections::HashMap::from([
        ("skill_level".to_string(), 0.8),
        ("engagement".to_string(), 0.7),
        ("community_feedback".to_string(), 0.9),
    ]);
    let explanation = engine.explain_prediction(&predictions, &features)?;
    println!("   Explanation summary: {}", explanation.summary);
    println!("   Prediction confidence: {:.2}", explanation.confidence);
    
    // Example: Community validation workflow
    println!("\n5. Generating community validation workflow...");
    let workflow = engine.explanation_generator.generate_community_validation_workflow(&predictions)?;
    println!("   Created validation workflow with {} steps", workflow.steps.len());
    
    // Example: Model evaluation with cooperative metrics
    println!("\n6. Evaluating model with cooperative metrics...");
    let actuals = vec![0.75, 0.72, 0.88, 0.62, 0.82]; // Placeholder data
    let evaluation = engine.model_evaluator.evaluate_model(&predictions, &actuals)?;
    println!("   Traditional accuracy: {:.2}", evaluation.traditional_metrics.accuracy);
    println!("   Community benefit: {:.2}", evaluation.community_metrics.community_benefit);
    println!("   Fairness score: {:.2}", evaluation.cooperative_metrics.fairness_score);
    println!("   Overall score: {:.2}", evaluation.overall_score);
    
    println!("\nCooperative values integration example completed successfully!");
    Ok(())
}