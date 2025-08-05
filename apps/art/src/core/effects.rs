//! Layer effects system for the Art application
//!
//! This module provides functionality for applying visual effects to layers,
//! such as shadows, glows, bevels, and other enhancements.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of effects that can be applied to layers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EffectType {
    /// Drop shadow effect
    DropShadow,
    /// Inner shadow effect
    InnerShadow,
    /// Outer glow effect
    OuterGlow,
    /// Inner glow effect
    InnerGlow,
    /// Bevel and emboss effect
    BevelEmboss,
    /// Stroke/outline effect
    Stroke,
    /// Color overlay effect
    ColorOverlay,
    /// Gradient overlay effect
    GradientOverlay,
    /// Pattern overlay effect
    PatternOverlay,
}

/// Common properties for all effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectProperties {
    /// Whether the effect is enabled
    pub enabled: bool,
    /// Opacity of the effect (0.0 to 1.0)
    pub opacity: f32,
    /// Blend mode for the effect
    pub blend_mode: BlendMode,
    /// Effect-specific properties stored as JSON
    pub properties: serde_json::Value,
}

/// Blend modes for effects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
    ColorDodge,
    ColorBurn,
    Darken,
    Lighten,
    Difference,
    Exclusion,
}

/// Drop shadow effect properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShadowProperties {
    /// Distance of the shadow
    pub distance: f32,
    /// Spread of the shadow
    pub spread: f32,
    /// Size of the shadow blur
    pub size: f32,
    /// Angle of the shadow (in degrees)
    pub angle: f32,
    /// Color of the shadow
    pub color: (u8, u8, u8, u8), // RGBA
    /// Whether to use global lighting angle
    pub use_global_light: bool,
    /// Whether the shadow is knock-out (transparent)
    pub knockout: bool,
    /// Whether the shadow is noise-based
    pub noise: bool,
}

/// Inner shadow effect properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerShadowProperties {
    /// Distance of the shadow
    pub distance: f32,
    /// Choke of the shadow
    pub choke: f32,
    /// Size of the shadow blur
    pub size: f32,
    /// Angle of the shadow (in degrees)
    pub angle: f32,
    /// Color of the shadow
    pub color: (u8, u8, u8, u8), // RGBA
    /// Whether to use global lighting angle
    pub use_global_light: bool,
    /// Whether the shadow is noise-based
    pub noise: bool,
}

/// Outer glow effect properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OuterGlowProperties {
    /// Size of the glow
    pub size: f32,
    /// Spread of the glow
    pub spread: f32,
    /// Color of the glow
    pub color: (u8, u8, u8, u8), // RGBA
    /// Technique for the glow (Softer/ Precise)
    pub technique: GlowTechnique,
    /// Whether the glow is noise-based
    pub noise: bool,
    /// Jitter for the glow
    pub jitter: f32,
}

/// Inner glow effect properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerGlowProperties {
    /// Size of the glow
    pub size: f32,
    /// Source of the glow (Center/Edge)
    pub source: GlowSource,
    /// Color of the glow
    pub color: (u8, u8, u8, u8), // RGBA
    /// Technique for the glow (Softer/ Precise)
    pub technique: GlowTechnique,
    /// Whether the glow is noise-based
    pub noise: bool,
    /// Jitter for the glow
    pub jitter: f32,
}

/// Bevel and emboss effect properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BevelEmbossProperties {
    /// Style of the bevel
    pub style: BevelStyle,
    /// Technique for the bevel
    pub technique: BevelTechnique,
    /// Depth of the bevel
    pub depth: f32,
    /// Direction of the bevel
    pub direction: BevelDirection,
    /// Size of the bevel
    pub size: f32,
    /// Soften of the bevel
    pub soften: f32,
    /// Angle of the light source (in degrees)
    pub angle: f32,
    /// Altitude of the light source (in degrees)
    pub altitude: f32,
    /// Highlight color
    pub highlight_color: (u8, u8, u8, u8), // RGBA
    /// Highlight blend mode
    pub highlight_blend_mode: BlendMode,
    /// Shadow color
    pub shadow_color: (u8, u8, u8, u8), // RGBA
    /// Shadow blend mode
    pub shadow_blend_mode: BlendMode,
}

/// Stroke effect properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrokeProperties {
    /// Size of the stroke
    pub size: f32,
    /// Position of the stroke
    pub position: StrokePosition,
    /// Fill type for the stroke
    pub fill_type: StrokeFillType,
    /// Color of the stroke (if solid fill)
    pub color: Option<(u8, u8, u8, u8)>, // RGBA
    /// Gradient for the stroke (if gradient fill)
    pub gradient: Option<Gradient>,
    /// Pattern for the stroke (if pattern fill)
    pub pattern: Option<Uuid>, // Pattern ID
    /// Blend mode for the stroke
    pub blend_mode: BlendMode,
    /// Opacity of the stroke
    pub opacity: f32,
}

/// Color overlay effect properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorOverlayProperties {
    /// Color for the overlay
    pub color: (u8, u8, u8, u8), // RGBA
    /// Blend mode for the overlay
    pub blend_mode: BlendMode,
    /// Opacity of the overlay
    pub opacity: f32,
}

/// Gradient overlay effect properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientOverlayProperties {
    /// Gradient for the overlay
    pub gradient: Gradient,
    /// Blend mode for the overlay
    pub blend_mode: BlendMode,
    /// Opacity of the overlay
    pub opacity: f32,
    /// Angle of the gradient (in degrees)
    pub angle: f32,
    /// Scale of the gradient
    pub scale: f32,
    /// Offset of the gradient
    pub offset: (f32, f32),
}

/// Pattern overlay effect properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternOverlayProperties {
    /// Pattern for the overlay
    pub pattern: Uuid, // Pattern ID
    /// Blend mode for the overlay
    pub blend_mode: BlendMode,
    /// Opacity of the overlay
    pub opacity: f32,
    /// Scale of the pattern
    pub scale: f32,
    /// Offset of the pattern
    pub offset: (f32, f32),
    /// Angle of the pattern (in degrees)
    pub angle: f32,
    /// Whether to link the pattern with the layer
    pub link_with_layer: bool,
}

/// Gradient definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gradient {
    /// Type of gradient
    pub gradient_type: GradientType,
    /// Colors in the gradient
    pub colors: Vec<(f32, (u8, u8, u8, u8))>, // (position, RGBA)
    /// Smoothness of the gradient
    pub smoothness: f32,
}

/// Types of gradients
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GradientType {
    Linear,
    Radial,
    Angle,
    Reflected,
    Diamond,
}

/// Techniques for glow effects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GlowTechnique {
    Softer,
    Precise,
}

/// Sources for inner glow effects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GlowSource {
    Center,
    Edge,
}

/// Styles for bevel effects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BevelStyle {
    OuterBevel,
    InnerBevel,
    Emboss,
    PillowEmboss,
    StrokeEmboss,
}

/// Techniques for bevel effects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BevelTechnique {
    Smooth,
    ChiselSoft,
    ChiselHard,
}

/// Directions for bevel effects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BevelDirection {
    Up,
    Down,
}

/// Positions for stroke effects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StrokePosition {
    Outside,
    Center,
    Inside,
}

/// Fill types for stroke effects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StrokeFillType {
    Color,
    Gradient,
    Pattern,
}

/// A layer effect that can be applied to a layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerEffect {
    /// Unique identifier for the effect
    pub id: Uuid,
    /// Name of the effect
    pub name: String,
    /// Type of the effect
    pub effect_type: EffectType,
    /// Properties of the effect
    pub properties: EffectProperties,
}

impl LayerEffect {
    /// Create a new layer effect
    pub fn new(name: String, effect_type: EffectType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            effect_type,
            properties: EffectProperties {
                enabled: true,
                opacity: 1.0,
                blend_mode: BlendMode::Normal,
                properties: serde_json::Value::Null,
            },
        }
    }
    
    /// Set whether the effect is enabled
    pub fn set_enabled(&mut self, enabled: bool) {
        self.properties.enabled = enabled;
    }
    
    /// Set the opacity of the effect
    pub fn set_opacity(&mut self, opacity: f32) {
        self.properties.opacity = opacity.clamp(0.0, 1.0);
    }
    
    /// Set the blend mode of the effect
    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.properties.blend_mode = blend_mode;
    }
    
    /// Set effect-specific properties
    pub fn set_properties(&mut self, properties: serde_json::Value) {
        self.properties.properties = properties;
    }
}