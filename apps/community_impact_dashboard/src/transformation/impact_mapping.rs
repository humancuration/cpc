//! Impact Connection Mapping System
//!
//! This module provides tools for mapping and visualizing how communities
//! discover and understand connections between different impact domains.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};

use crate::models::impact_data::ImpactDomain;
use crate::models::interconnection::ImpactInterconnection;

/// Impact connection mapping system
pub struct ImpactConnectionMapping {
    /// Stored connection maps
    connection_maps: HashMap<Uuid, ConnectionMap>,
    
    /// Connection patterns discovered across communities
    global_patterns: Vec<ConnectionPattern>,
    
    /// Visualization templates
    visualization_templates: Vec<VisualizationTemplate>,
}

impl ImpactConnectionMapping {
    /// Create a new impact connection mapping system
    pub fn new() -> Self {
        Self {
            connection_maps: HashMap::new(),
            global_patterns: Vec::new(),
            visualization_templates: Self::create_default_templates(),
        }
    }

    /// Initialize the impact connection mapping system
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize with default templates
        Ok(())
    }

    /// Create a new connection map
    pub fn create_connection_map(
        &mut self,
        community_id: String,
        name: String,
        description: String,
    ) -> Uuid {
        let connection_map = ConnectionMap {
            id: Uuid::new_v4(),
            community_id,
            name,
            description,
            connections: Vec::new(),
            discovered_at: Utc::now(),
            last_updated: Utc::now(),
            evolution_history: Vec::new(),
            visualization_settings: VisualizationSettings::default(),
        };
        
        let map_id = connection_map.id;
        self.connection_maps.insert(map_id, connection_map);
        map_id
    }

    /// Get a connection map by ID
    pub fn get_connection_map(&self, map_id: Uuid) -> Option<&ConnectionMap> {
        self.connection_maps.get(&map_id)
    }

    /// Get all connection maps for a community
    pub fn get_community_maps(&self, community_id: &str) -> Vec<&ConnectionMap> {
        self.connection_maps
            .values()
            .filter(|map| map.community_id == community_id)
            .collect()
    }

    /// Add a discovered connection to a map
    pub fn add_discovered_connection(
        &mut self,
        map_id: Uuid,
        connection: DiscoveredConnection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(map) = self.connection_maps.get_mut(&map_id) {
            map.connections.push(connection);
            map.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Connection map not found".into())
        }
    }

    /// Record evolution of connection understanding
    pub fn record_evolution(
        &mut self,
        map_id: Uuid,
        evolution: ConnectionEvolution,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(map) = self.connection_maps.get_mut(&map_id) {
            map.evolution_history.push(evolution);
            map.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Connection map not found".into())
        }
    }

    /// Analyze connection patterns across communities
    pub fn analyze_patterns(&mut self) -> Vec<ConnectionPattern> {
        let mut patterns = Vec::new();
        
        // Analyze frequency of domain connections
        let mut connection_counts: HashMap<(String, String), usize> = HashMap::new();
        
        for map in self.connection_maps.values() {
            for connection in &map.connections {
                let key = (connection.source_domain.clone(), connection.target_domain.clone());
                *connection_counts.entry(key).or_insert(0) += 1;
            }
        }
        
        // Identify common patterns
        for ((source, target), count) in connection_counts {
            if count >= 3 { // Threshold for pattern recognition
                patterns.push(ConnectionPattern {
                    id: Uuid::new_v4(),
                    source_domain: source,
                    target_domain: target,
                    frequency: count,
                    description: format!("Common connection between {} and {}", source, target),
                    communities_observed: self.get_communities_with_connection(&source, &target),
                    first_observed: self.get_first_observation(&source, &target),
                });
            }
        }
        
        self.global_patterns = patterns.clone();
        patterns
    }

    /// Get communities that have a specific connection
    fn get_communities_with_connection(&self, source: &str, target: &str) -> Vec<String> {
        let mut communities = HashSet::new();
        
        for map in self.connection_maps.values() {
            for connection in &map.connections {
                if connection.source_domain == source && connection.target_domain == target {
                    communities.insert(map.community_id.clone());
                }
            }
        }
        
        communities.into_iter().collect()
    }

    /// Get first observation of a connection
    fn get_first_observation(&self, source: &str, target: &str) -> Option<DateTime<Utc>> {
        let mut earliest = None;
        
        for map in self.connection_maps.values() {
            for connection in &map.connections {
                if connection.source_domain == source && connection.target_domain == target {
                    match earliest {
                        None => earliest = Some(connection.discovered_at),
                        Some(time) => {
                            if connection.discovered_at < time {
                                earliest = Some(connection.discovered_at);
                            }
                        }
                    }
                }
            }
        }
        
        earliest
    }

    /// Create default visualization templates
    fn create_default_templates() -> Vec<VisualizationTemplate> {
        vec![
            VisualizationTemplate {
                id: Uuid::new_v4(),
                name: "Network Graph".to_string(),
                description: "Interactive network graph showing domain connections".to_string(),
                visualization_type: VisualizationType::NetworkGraph,
                default_settings: HashMap::from([
                    ("layout".to_string(), "force_directed".to_string()),
                    ("node_size".to_string(), "dynamic".to_string()),
                    ("edge_width".to_string(), "strength_based".to_string()),
                ]),
            },
            VisualizationTemplate {
                id: Uuid::new_v4(),
                name: "Circular Flow".to_string(),
                description: "Circular flow diagram showing impact circulation".to_string(),
                visualization_type: VisualizationType::CircularFlow,
                default_settings: HashMap::from([
                    ("arrangement".to_string(), "circular".to_string()),
                    ("flow_direction".to_string(), "clockwise".to_string()),
                    ("animation".to_string(), "enabled".to_string()),
                ]),
            },
            VisualizationTemplate {
                id: Uuid::new_v4(),
                name: "Heat Map".to_string(),
                description: "Heat map showing connection strength between domains".to_string(),
                visualization_type: VisualizationType::HeatMap,
                default_settings: HashMap::from([
                    ("color_scheme".to_string(), "viridis".to_string()),
                    ("cell_size".to_string(), "adaptive".to_string()),
                    ("show_values".to_string(), "true".to_string()),
                ]),
            },
        ]
    }
}

/// Connection map for a community
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionMap {
    /// Map identifier
    pub id: Uuid,
    
    /// Community identifier
    pub community_id: String,
    
    /// Map name
    pub name: String,
    
    /// Map description
    pub description: String,
    
    /// Discovered connections
    pub connections: Vec<DiscoveredConnection>,
    
    /// When this map was created
    pub discovered_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
    
    /// History of how understanding evolved
    pub evolution_history: Vec<ConnectionEvolution>,
    
    /// Visualization settings
    pub visualization_settings: VisualizationSettings,
}

/// Discovered connection between impact domains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredConnection {
    /// Connection identifier
    pub id: Uuid,
    
    /// Source impact domain
    pub source_domain: String,
    
    /// Target impact domain
    pub target_domain: String,
    
    /// Description of the connection
    pub description: String,
    
    /// Strength of the connection (0.0 to 1.0)
    pub strength: f64,
    
    /// How this connection was discovered
    pub discovery_method: DiscoveryMethod,
    
    /// When this connection was discovered
    pub discovered_at: DateTime<Utc>,
    
    /// Who discovered this connection
    pub discovered_by: Vec<String>,
    
    /// Evidence supporting this connection
    pub evidence: Vec<EvidenceItem>,
    
    /// Impact of understanding this connection
    pub impact: ConnectionImpact,
    
    /// Related transformation stories
    pub related_stories: Vec<Uuid>,
    
    /// Current understanding level
    pub understanding_level: UnderstandingLevel,
}

/// Methods for discovering connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    /// Dashboard visualization revealed the connection
    DashboardVisualization,
    /// Data analysis identified correlation
    DataAnalysis,
    /// Community discussion surfaced the connection
    CommunityDiscussion,
    /// Workshop or facilitated process
    FacilitatedWorkshop,
    /// Individual insight or reflection
    IndividualInsight,
    /// External research or input
    ExternalResearch,
    /// Other discovery method
    Other(String),
}

/// Evidence supporting a connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    /// Evidence description
    pub description: String,
    
    /// Type of evidence
    pub evidence_type: EvidenceType,
    
    /// Source of the evidence
    pub source: String,
    
    /// Strength of this evidence (0.0 to 1.0)
    pub strength: f64,
    
    /// When this evidence was collected
    pub collected_at: DateTime<Utc>,
}

/// Types of evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    /// Quantitative data
    QuantitativeData,
    /// Qualitative observation
    QualitativeObservation,
    /// Community testimony
    CommunityTestimony,
    /// Expert opinion
    ExpertOpinion,
    /// Historical record
    HistoricalRecord,
    /// Experimental result
    ExperimentalResult,
    /// Other evidence type
    Other(String),
}

/// Impact of understanding a connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionImpact {
    /// Description of the impact
    pub description: String,
    
    /// Changes in community behavior
    pub behavior_changes: Vec<String>,
    
    /// New initiatives or actions taken
    pub new_initiatives: Vec<String>,
    
    /// Shifts in community understanding
    pub understanding_shifts: Vec<String>,
    
    /// Measurable outcomes
    pub measurable_outcomes: Vec<String>,
}

/// Levels of understanding a connection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UnderstandingLevel {
    /// Initial awareness - connection noticed but not understood
    InitialAwareness,
    /// Surface understanding - basic comprehension of the connection
    SurfaceUnderstanding,
    /// Deep understanding - comprehensive grasp of the connection
    DeepUnderstanding,
    /// Applied understanding - using the connection to drive action
    AppliedUnderstanding,
    /// Transformative understanding - connection integrated into community practice
    TransformativeUnderstanding,
}

/// Evolution of connection understanding over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionEvolution {
    /// Evolution identifier
    pub id: Uuid,
    
    /// When this evolution was recorded
    pub timestamp: DateTime<Utc>,
    
    /// Description of what changed
    pub change_description: String,
    
    /// Previous understanding level
    pub previous_level: UnderstandingLevel,
    
    /// New understanding level
    pub new_level: UnderstandingLevel,
    
    /// What triggered this evolution
    pub trigger: String,
    
    /// Who was involved in this evolution
    pub participants: Vec<String>,
    
    /// Outcomes of this evolution
    pub outcomes: Vec<String>,
}

/// Connection pattern observed across communities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPattern {
    /// Pattern identifier
    pub id: Uuid,
    
    /// Source domain
    pub source_domain: String,
    
    /// Target domain
    pub target_domain: String,
    
    /// How many communities have this pattern
    pub frequency: usize,
    
    /// Pattern description
    pub description: String,
    
    /// Communities where this pattern has been observed
    pub communities_observed: Vec<String>,
    
    /// When this pattern was first observed
    pub first_observed: Option<DateTime<Utc>>,
}

/// Visualization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationSettings {
    /// Type of visualization
    pub visualization_type: VisualizationType,
    
    /// Custom settings
    pub custom_settings: HashMap<String, String>,
    
    /// Color scheme
    pub color_scheme: ColorScheme,
    
    /// Layout preferences
    pub layout_preferences: LayoutPreferences,
}

impl Default for VisualizationSettings {
    fn default() -> Self {
        Self {
            visualization_type: VisualizationType::NetworkGraph,
            custom_settings: HashMap::new(),
            color_scheme: ColorScheme::Default,
            layout_preferences: LayoutPreferences::default(),
        }
    }
}

/// Types of visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualizationType {
    /// Network graph visualization
    NetworkGraph,
    /// Circular flow diagram
    CircularFlow,
    /// Heat map
    HeatMap,
    /// Timeline visualization
    Timeline,
    /// Force-directed graph
    ForceDirected,
    /// Hierarchical tree
    HierarchicalTree,
    /// Other visualization type
    Other(String),
}

/// Color schemes for visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorScheme {
    /// Default color scheme
    Default,
    /// Vibrant colors
    Vibrant,
    /// Pastel colors
    Pastel,
    /// Grayscale
    Grayscale,
    /// Colorblind-friendly
    ColorblindFriendly,
    /// Custom color scheme
    Custom(String),
}

/// Layout preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutPreferences {
    /// Spacing between elements
    pub spacing: f64,
    
    /// Animation enabled
    pub animation_enabled: bool,
    
    /// Show labels
    pub show_labels: bool,
    
    /// Interactive elements
    pub interactive: bool,
    
    /// Responsive layout
    pub responsive: bool,
}

impl Default for LayoutPreferences {
    fn default() -> Self {
        Self {
            spacing: 1.0,
            animation_enabled: true,
            show_labels: true,
            interactive: true,
            responsive: true,
        }
    }
}

/// Visualization template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationTemplate {
    /// Template identifier
    pub id: Uuid,
    
    /// Template name
    pub name: String,
    
    /// Template description
    pub description: String,
    
    /// Type of visualization
    pub visualization_type: VisualizationType,
    
    /// Default settings
    pub default_settings: HashMap<String, String>,
}