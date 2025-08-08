//! Impact Interconnection Model
//!
//! This module defines data structures that represent the interconnections
//! between the four impact domains.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::impact_data::ImpactDomain;

/// Impact Interconnection
/// 
/// Represents a connection between two impact domains showing how
/// engagement in one area strengthens the community across all areas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactInterconnection {
    /// Unique identifier for this interconnection
    pub id: Uuid,
    
    /// Timestamp when this interconnection was measured
    pub timestamp: DateTime<Utc>,
    
    /// Source domain (the domain that influences)
    pub source_domain: ImpactDomain,
    
    /// Target domain (the domain that is influenced)
    pub target_domain: ImpactDomain,
    
    /// Strength of the interconnection (0.0 to 1.0)
    pub strength: f64,
    
    /// Description of how the interconnection works
    pub description: String,
    
    /// Evidence supporting this interconnection
    pub evidence: Vec<InterconnectionEvidence>,
    
    /// Cooperative values alignment of this interconnection
    pub values_alignment: Vec<String>,
}

/// Interconnection Evidence
/// 
/// Evidence supporting an impact interconnection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterconnectionEvidence {
    /// Evidence ID
    pub id: Uuid,
    
    /// Type of evidence
    pub evidence_type: EvidenceType,
    
    /// Description of the evidence
    pub description: String,
    
    /// Statistical significance if applicable
    pub significance: Option<f64>,
    
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
}

/// Evidence Type
/// 
/// Types of evidence supporting interconnections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    /// Correlation data between domains
    Correlation,
    
    /// Causal relationship evidence
    Causation,
    
    /// Community validation
    CommunityValidation,
    
    /// Expert analysis
    ExpertAnalysis,
    
    /// Case study
    CaseStudy,
}

/// Circular Impact Flow
/// 
/// Represents the complete circular impact flow showing all interconnections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircularImpactFlow {
    /// Unique identifier for this flow
    pub id: Uuid,
    
    /// Timestamp when this flow was measured
    pub timestamp: DateTime<Utc>,
    
    /// All interconnections in the circular flow
    pub interconnections: Vec<ImpactInterconnection>,
    
    /// Overall strength of the circular flow (0.0 to 1.0)
    pub overall_strength: f64,
    
    /// Bottlenecks in the flow
    pub bottlenecks: Vec<ImpactBottleneck>,
    
    /// Amplification points in the flow
    pub amplifications: Vec<ImpactAmplification>,
}

/// Impact Bottleneck
/// 
/// A point in the circular impact flow where the strength is weak
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactBottleneck {
    /// Bottleneck ID
    pub id: Uuid,
    
    /// Domain where the bottleneck occurs
    pub domain: ImpactDomain,
    
    /// Strength at this bottleneck (0.0 to 1.0)
    pub strength: f64,
    
    /// Suggested solutions to address the bottleneck
    pub solutions: Vec<String>,
}

/// Impact Amplification
/// 
/// A point in the circular impact flow where the strength is amplified
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAmplification {
    /// Amplification ID
    pub id: Uuid,
    
    /// Source domain that amplifies
    pub source_domain: ImpactDomain,
    
    /// Target domain that is amplified
    pub target_domain: ImpactDomain,
    
    /// Amplification factor (1.0 means no amplification, >1.0 means amplification)
    pub factor: f64,
    
    /// Explanation of why amplification occurs
    pub explanation: String,
}

impl ImpactInterconnection {
    /// Create a new ImpactInterconnection
    pub fn new(
        source_domain: ImpactDomain,
        target_domain: ImpactDomain,
        strength: f64,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source_domain,
            target_domain,
            strength,
            description,
            evidence: Vec::new(),
            values_alignment: Vec::new(),
        }
    }
    
    /// Add evidence to support this interconnection
    pub fn add_evidence(mut self, evidence: InterconnectionEvidence) -> Self {
        self.evidence.push(evidence);
        self
    }
    
    /// Add cooperative values alignment
    pub fn add_values_alignment(mut self, value: String) -> Self {
        self.values_alignment.push(value);
        self
    }
}

impl CircularImpactFlow {
    /// Create a new CircularImpactFlow
    pub fn new(interconnections: Vec<ImpactInterconnection>) -> Self {
        let overall_strength = interconnections.iter()
            .map(|i| i.strength)
            .sum::<f64>() / interconnections.len() as f64;
            
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            interconnections,
            overall_strength,
            bottlenecks: Vec::new(),
            amplifications: Vec::new(),
        }
    }
    
    /// Add a bottleneck to this flow
    pub fn add_bottleneck(mut self, bottleneck: ImpactBottleneck) -> Self {
        self.bottlenecks.push(bottleneck);
        self
    }
    
    /// Add an amplification to this flow
    pub fn add_amplification(mut self, amplification: ImpactAmplification) -> Self {
        self.amplifications.push(amplification);
        self
    }
}