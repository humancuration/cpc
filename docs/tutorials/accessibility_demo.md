# Accessibility Demo Tutorial

This tutorial demonstrates advanced accessibility features in the CPC visualization system through practical examples.

## Prerequisites

Before starting this tutorial, ensure you have:

- Completed the [Basic Chart Implementation Tutorial](./basic_chart_implementation.md)
- Understanding of accessibility concepts
- Familiarity with screen readers (NVDA, JAWS, VoiceOver)
- Access to a CPC development environment

## Learning Objectives

By the end of this tutorial, you will be able to:

1. Implement comprehensive accessibility metadata
2. Create screen reader-optimized visualizations
3. Design keyboard-navigable interfaces
4. Test accessibility features effectively
5. Understand compliance requirements for accessibility

## Step 1: Project Setup

Create a new Rust project for our accessibility demo:

```bash
cargo new accessibility_demo
cd accessibility_demo
```

Add the required dependencies to `Cargo.toml`:

```toml
[dependencies]
visualization_context = { path = "../../packages/visualization_context" }
bevy = "0.16"
plotters = "0.3"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
accesskit = "0.10"
accesskit_unix = { version = "0.7", optional = true }
accesskit_windows = { version = "0.12", optional = true }
accesskit_macos = { version = "0.7", optional = true }
```

## Step 2: Create Accessibility Manager

Create `src/accessibility.rs`:

```rust
use visualization_context::AccessibilityMetadata;
use serde_json::Value;
use std::collections::HashMap;

pub struct AccessibilityManager;

#[derive(Debug, Clone)]
pub struct AccessibilityConfig {
    pub mode: AccessibilityMode,
    pub language: String,
    pub text_size: TextSize,
    pub color_contrast: ColorContrast,
    pub motion_preference: MotionPreference,
}

#[derive(Debug, Clone)]
pub enum AccessibilityMode {
    Standard,
    ScreenReader,
    HighContrast,
    CognitiveSupport,
}

#[derive(Debug, Clone)]
pub enum TextSize {
    Small,
    Normal,
    Large,
    ExtraLarge,
}

#[derive(Debug, Clone)]
pub enum ColorContrast {
    Standard,
    High,
    Enhanced,
}

#[derive(Debug, Clone)]
pub enum MotionPreference {
    Full,
    Reduced,
    None,
}

impl AccessibilityManager {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate_alt_text(
        &self,
        chart_type: &str,
        data_labels: &[String],
        data_values: &[f64],
        config: &AccessibilityConfig,
    ) -> String {
        match config.mode {
            AccessibilityMode::ScreenReader => {
                self.generate_screen_reader_alt_text(chart_type, data_labels, data_values)
            }
            AccessibilityMode::CognitiveSupport => {
                self.generate_cognitive_support_alt_text(chart_type, data_labels, data_values)
            }
            _ => {
                self.generate_standard_alt_text(chart_type, data_labels, data_values)
            }
        }
    }
    
    fn generate_standard_alt_text(
        &self,
        chart_type: &str,
        data_labels: &[String],
        data_values: &[f64],
    ) -> String {
        match chart_type {
            "bar" => {
                let max_value = data_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let min_value = data_values.iter().cloned().fold(f64::INFINITY, f64::min);
                
                format!(
                    "Bar chart showing {} data points. Values range from {} to {}. Highest value is {} in {}.",
                    data_labels.len(),
                    min_value,
                    max_value,
                    max_value,
                    data_labels[data_values.iter().position(|&x| x == max_value).unwrap_or(0)]
                )
            }
            "line" => {
                let first_value = data_values.first().unwrap_or(&0.0);
                let last_value = data_values.last().unwrap_or(&0.0);
                let trend = if last_value > first_value { "increasing" } else { "decreasing" };
                
                format!(
                    "Line chart with {} data points showing {} trend. Starting value: {}, ending value: {}.",
                    data_labels.len(),
                    trend,
                    first_value,
                    last_value
                )
            }
            "pie" => {
                format!(
                    "Pie chart with {} segments showing proportional data.",
                    data_labels.len()
                )
            }
            _ => "Chart visualization".to_string(),
        }
    }
    
    fn generate_screen_reader_alt_text(
        &self,
        chart_type: &str,
        data_labels: &[String],
        data_values: &[f64],
    ) -> String {
        match chart_type {
            "bar" => {
                let descriptions: Vec<String> = data_labels
                    .iter()
                    .zip(data_values.iter())
                    .map(|(label, value)| format!("{}: {}", label, value))
                    .collect();
                
                format!(
                    "Bar chart with {} categories. Data points: {}. Use keyboard arrows to navigate between bars.",
                    data_labels.len(),
                    descriptions.join(", ")
                )
            }
            "line" => {
                let first_value = data_values.first().unwrap_or(&0.0);
                let last_value = data_values.last().unwrap_or(&0.0);
                let change = last_value - first_value;
                let percentage_change = if *first_value != 0.0 {
                    (change / first_value) * 100.0
                } else {
                    0.0
                };
                
                format!(
                    "Line chart with {} data points. Starting at {}, ending at {}. Net change of {} ({:+.1}%). Use keyboard arrows to follow the trend line.",
                    data_labels.len(),
                    first_value,
                    last_value,
                    change,
                    percentage_change
                )
            }
            "pie" => {
                let descriptions: Vec<String> = data_labels
                    .iter()
                    .zip(data_values.iter())
                    .map(|(label, value)| format!("{}: {:.1}%", label, value))
                    .collect();
                
                format!(
                    "Pie chart with {} segments. Data distribution: {}. Use keyboard arrows to navigate between segments.",
                    data_labels.len(),
                    descriptions.join(", ")
                )
            }
            _ => "Interactive chart visualization. Use keyboard navigation to explore data points.".to_string(),
        }
    }
    
    fn generate_cognitive_support_alt_text(
        &self,
        chart_type: &str,
        data_labels: &[String],
        data_values: &[f64],
    ) -> String {
        match chart_type {
            "bar" => {
                let max_value = data_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let max_label = data_labels[data_values.iter().position(|&x| x == max_value).unwrap_or(0)].clone();
                
                format!(
                    "Simple bar chart. The highest bar is {} with a value of {}. This shows the largest amount in the data.",
                    max_label,
                    max_value
                )
            }
            "line" => {
                let first_value = data_values.first().unwrap_or(&0.0);
                let last_value = data_values.last().unwrap_or(&0.0);
                let trend = if last_value > first_value { 
                    "going up" 
                } else if last_value < first_value { 
                    "going down" 
                } else { 
                    "staying the same" 
                };
                
                format!(
                    "Simple line chart. The line is {}. This shows how the values change over time.",
                    trend
                )
            }
            "pie" => {
                let max_value = data_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let max_label = data_labels[data_values.iter().position(|&x| x == max_value).unwrap_or(0)].clone();
                let percentage = max_value;
                
                format!(
                    "Simple pie chart. The largest piece is {} taking up {:.1}% of the chart. This shows the biggest part of the whole.",
                    max_label,
                    percentage
                )
            }
            _ => "Simple chart to show data clearly.".to_string(),
        }
    }
    
    pub fn create_navigation_map(
        &self,
        data_labels: &[String],
        config: &AccessibilityConfig,
    ) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        
        // Add title navigation
        map.insert("T".to_string(), serde_json::json!({
            "label": "Chart Title",
            "position": [0, 3, 0]
        }));
        
        // Add legend navigation
        map.insert("L".to_string(), serde_json::json!({
            "label": "Legend",
            "position": [-3, 0, 0]
        }));
        
        // Add data navigation based on accessibility mode
        match config.mode {
            AccessibilityMode::ScreenReader => {
                // For screen readers, provide detailed navigation
                for (index, label) in data_labels.iter().enumerate() {
                    let key = format!("D{}", index + 1);
                    map.insert(key, serde_json::json!({
                        "label": format!("Data point: {}", label),
                        "position": [index as i32, 0, 0],
                        "value": data_values.get(index).unwrap_or(&0.0)
                    }));
                }
            }
            AccessibilityMode::CognitiveSupport => {
                // For cognitive support, simplify navigation
                map.insert("D".to_string(), serde_json::json!({
                    "label": "Data Points",
                    "position": [0, 0, 0]
                }));
            }
            _ => {
                // Standard navigation
                map.insert("D".to_string(), serde_json::json!({
                    "label": "Data Points",
                    "position": [0, 0, 0]
                }));
            }
        }
        
        map
    }
    
    pub fn create_accessibility_metadata(
        &self,
        chart_type: &str,
        data_labels: &[String],
        data_values: &[f64],
        config: &AccessibilityConfig,
    ) -> AccessibilityMetadata {
        AccessibilityMetadata {
            alt_text: self.generate_alt_text(chart_type, data_labels, data_values, config),
            navigation_map: self.create_navigation_map(data_labels, config),
            live_region: match config.mode {
                AccessibilityMode::ScreenReader => "polite".to_string(),
                _ => "off".to_string(),
            },
        }
    }
    
    pub fn apply_color_contrast(
        &self,
        colors: &[String],
        contrast_level: ColorContrast,
    ) -> Vec<String> {
        match contrast_level {
            ColorContrast::High => {
                // Convert to high contrast colors
                colors.iter().map(|_| "#000000".to_string()).collect()
            }
            ColorContrast::Enhanced => {
                // Convert to enhanced contrast colors
                colors.iter().map(|_| "#FFFFFF".to_string()).collect()
            }
            _ => colors.to_vec(),
        }
    }
    
    pub fn apply_text_scaling(
        &self,
        base_font_size: u32,
        text_size: TextSize,
    ) -> u32 {
        match text_size {
            TextSize::Small => (base_font_size as f32 * 0.8) as u32,
            TextSize::Large => (base_font_size as f32 * 1.2) as u32,
            TextSize::ExtraLarge => (base_font_size as f32 * 1.5) as u32,
            _ => base_font_size,
        }
    }
    
    pub fn should_reduce_motion(&self, preference: MotionPreference) -> bool {
        match preference {
            MotionPreference::Reduced | MotionPreference::None => true,
            _ => false,
        }
    }
}
```

## Step 3: Create Accessible Visualization Components

Create `src/components.rs`:

```rust
use crate::accessibility::{AccessibilityManager, AccessibilityConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibleBarChart {
    pub title: String,
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub colors: Vec<String>,
    pub accessibility_config: AccessibilityConfig,
}

impl AccessibleBarChart {
    pub fn new(
        title: String,
        labels: Vec<String>,
        values: Vec<f64>,
        accessibility_config: AccessibilityConfig,
    ) -> Self {
        // Default colors - in a real app, these would be more sophisticated
        let colors = vec![
            "#FF6B6B".to_string(),
            "#4ECDC4".to_string(),
            "#45B7D1".to_string(),
            "#96CEB4".to_string(),
            "#FFEAA7".to_string(),
        ];
        
        Self {
            title,
            labels,
            values,
            colors,
            accessibility_config,
        }
    }
    
    pub fn generate_accessibility_metadata(&self) -> visualization_context::AccessibilityMetadata {
        let manager = AccessibilityManager::new();
        manager.create_accessibility_metadata(
            "bar",
            &self.labels,
            &self.values,
            &self.accessibility_config,
        )
    }
    
    pub fn get_contrast_colors(&self) -> Vec<String> {
        let manager = AccessibilityManager::new();
        manager.apply_color_contrast(
            &self.colors,
            self.accessibility_config.color_contrast.clone(),
        )
    }
    
    pub fn get_scaled_font_size(&self, base_size: u32) -> u32 {
        let manager = AccessibilityManager::new();
        manager.apply_text_scaling(base_size, self.accessibility_config.text_size.clone())
    }
    
    pub fn should_animate(&self) -> bool {
        let manager = AccessibilityManager::new();
        !manager.should_reduce_motion(self.accessibility_config.motion_preference.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibleLineChart {
    pub title: String,
    pub x_labels: Vec<String>,
    pub y_values: Vec<f64>,
    pub accessibility_config: AccessibilityConfig,
}

impl AccessibleLineChart {
    pub fn new(
        title: String,
        x_labels: Vec<String>,
        y_values: Vec<f64>,
        accessibility_config: AccessibilityConfig,
    ) -> Self {
        Self {
            title,
            x_labels,
            y_values,
            accessibility_config,
        }
    }
    
    pub fn generate_accessibility_metadata(&self) -> visualization_context::AccessibilityMetadata {
        let manager = AccessibilityManager::new();
        manager.create_accessibility_metadata(
            "line",
            &self.x_labels,
            &self.y_values,
            &self.accessibility_config,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessiblePieChart {
    pub title: String,
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub accessibility_config: AccessibilityConfig,
}

impl AccessiblePieChart {
    pub fn new(
        title: String,
        labels: Vec<String>,
        values: Vec<f64>,
        accessibility_config: AccessibilityConfig,
    ) -> Self {
        Self {
            title,
            labels,
            values,
            accessibility_config,
        }
    }
    
    pub fn generate_accessibility_metadata(&self) -> visualization_context::AccessibilityMetadata {
        let manager = AccessibilityManager::new();
        manager.create_accessibility_metadata(
            "pie",
            &self.labels,
            &self.values,
            &self.accessibility_config,
        )
    }
}
```

## Step 4: Implement Keyboard Navigation

Create `src/keyboard.rs`:

```rust
use std::collections::HashMap;

pub struct KeyboardNavigation {
    shortcuts: HashMap<String, String>,
    current_focus: usize,
    total_elements: usize,
}

impl KeyboardNavigation {
    pub fn new(total_elements: usize) -> Self {
        let mut nav = Self {
            shortcuts: HashMap::new(),
            current_focus: 0,
            total_elements,
        };
        
        nav.initialize_shortcuts();
        nav
    }
    
    fn initialize_shortcuts(&mut self) {
        self.shortcuts.insert("Tab".to_string(), "Move to next element".to_string());
        self.shortcuts.insert("Shift+Tab".to_string(), "Move to previous element".to_string());
        self.shortcuts.insert("Enter".to_string(), "Activate current element".to_string());
        self.shortcuts.insert("ArrowUp".to_string(), "Move up in chart".to_string());
        self.shortcuts.insert("ArrowDown".to_string(), "Move down in chart".to_string());
        self.shortcuts.insert("ArrowLeft".to_string(), "Move left in chart".to_string());
        self.shortcuts.insert("ArrowRight".to_string(), "Move right in chart".to_string());
        self.shortcuts.insert("Home".to_string(), "Go to first data point".to_string());
        self.shortcuts.insert("End".to_string(), "Go to last data point".to_string());
        self.shortcuts.insert("T".to_string(), "Go to chart title".to_string());
        self.shortcuts.insert("L".to_string(), "Go to legend".to_string());
        self.shortcuts.insert("D".to_string(), "Go to data points".to_string());
        self.shortcuts.insert("R".to_string(), "Refresh chart".to_string());
        self.shortcuts.insert("Plus".to_string(), "Zoom in".to_string());
        self.shortcuts.insert("Minus".to_string(), "Zoom out".to_string());
    }
    
    pub fn get_shortcuts(&self) -> &HashMap<String, String> {
        &self.shortcuts
    }
    
    pub fn get_shortcut_help(&self) -> String {
        let mut help = "Keyboard Navigation Shortcuts:\n".to_string();
        for (key, description) in &self.shortcuts {
            help.push_str(&format!("  {}: {}\n", key, description));
        }
        help
    }
    
    pub fn move_focus_next(&mut self) -> usize {
        self.current_focus = (self.current_focus + 1) % self.total_elements;
        self.current_focus
    }
    
    pub fn move_focus_previous(&mut self) -> usize {
        if self.current_focus == 0 {
            self.current_focus = self.total_elements - 1;
        } else {
            self.current_focus -= 1;
        }
        self.current_focus
    }
    
    pub fn move_focus_to(&mut self, index: usize) -> usize {
        if index < self.total_elements {
            self.current_focus = index;
        }
        self.current_focus
    }
    
    pub fn get_current_focus(&self) -> usize {
        self.current_focus
    }
    
    pub fn get_focus_description(&self, element_labels: &[String]) -> String {
        if self.current_focus < element_labels.len() {
            format!("Focused on: {}", element_labels[self.current_focus])
        } else {
            "Focus position out of range".to_string()
        }
    }
}
```

## Step 5: Create Accessibility Testing Framework

Create `src/testing.rs`:

```rust
use crate::accessibility::{AccessibilityManager, AccessibilityConfig, AccessibilityMode};
use crate::components::{AccessibleBarChart, AccessibleLineChart, AccessiblePieChart};
use std::fmt::Write;

pub struct AccessibilityTester {
    manager: AccessibilityManager,
}

impl AccessibilityTester {
    pub fn new() -> Self {
        Self {
            manager: AccessibilityManager::new(),
        }
    }
    
    pub fn test_bar_chart_accessibility(&self) -> String {
        let mut result = String::new();
        
        // Test data
        let labels = vec![
            "January".to_string(),
            "February".to_string(),
            "March".to_string(),
            "April".to_string(),
        ];
        let values = vec![100.0, 150.0, 200.0, 175.0];
        
        // Test standard mode
        let config = AccessibilityConfig {
            mode: AccessibilityMode::Standard,
            language: "en".to_string(),
            text_size: crate::accessibility::TextSize::Normal,
            color_contrast: crate::accessibility::ColorContrast::Standard,
            motion_preference: crate::accessibility::MotionPreference::Full,
        };
        
        let alt_text = self.manager.generate_alt_text("bar", &labels, &values, &config);
        writeln!(result, "Standard Mode Alt Text: {}", alt_text).unwrap();
        
        // Test screen reader mode
        let config = AccessibilityConfig {
            mode: AccessibilityMode::ScreenReader,
            language: "en".to_string(),
            text_size: crate::accessibility::TextSize::Normal,
            color_contrast: crate::accessibility::ColorContrast::Standard,
            motion_preference: crate::accessibility::MotionPreference::Full,
        };
        
        let alt_text = self.manager.generate_alt_text("bar", &labels, &values, &config);
        writeln!(result, "Screen Reader Mode Alt Text: {}", alt_text).unwrap();
        
        // Test cognitive support mode
        let config = AccessibilityConfig {
            mode: AccessibilityMode::CognitiveSupport,
            language: "en".to_string(),
            text_size: crate::accessibility::TextSize::Normal,
            color_contrast: crate::accessibility::ColorContrast::Standard,
            motion_preference: crate::accessibility::MotionPreference::Full,
        };
        
        let alt_text = self.manager.generate_alt_text("bar", &labels, &values, &config);
        writeln!(result, "Cognitive Support Mode Alt Text: {}", alt_text).unwrap();
        
        result
    }
    
    pub fn test_keyboard_navigation(&self) -> String {
        let mut result = String::new();
        
        let labels = vec![
            "Point A".to_string(),
            "Point B".to_string(),
            "Point C".to_string(),
            "Point D".to_string(),
        ];
        
        let nav = crate::keyboard::KeyboardNavigation::new(labels.len());
        writeln!(result, "{}", nav.get_shortcut_help()).unwrap();
        
        result
    }
    
    pub fn test_color_contrast(&self) -> String {
        let mut result = String::new();
        
        let base_colors = vec![
            "#FF0000".to_string(),
            "#00FF00".to_string(),
            "#0000FF".to_string(),
        ];
        
        let high_contrast = self.manager.apply_color_contrast(
            &base_colors,
            crate::accessibility::ColorContrast::High,
        );
        
        writeln!(result, "Base Colors: {:?}", base_colors).unwrap();
        writeln!(result, "High Contrast Colors: {:?}", high_contrast).unwrap();
        
        result
    }
    
    pub fn run_comprehensive_test(&self) -> String {
        let mut result = "=== Accessibility Comprehensive Test ===\n".to_string();
        
        writeln!(result, "\n--- Bar Chart Testing ---").unwrap();
        result.push_str(&self.test_bar_chart_accessibility());
        
        writeln!(result, "\n--- Keyboard Navigation Testing ---").unwrap();
        result.push_str(&self.test_keyboard_navigation());
        
        writeln!(result, "\n--- Color Contrast Testing ---").unwrap();
        result.push_str(&self.test_color_contrast());
        
        writeln!(result, "\n=== Test Complete ===").unwrap();
        
        result
    }
}
```

## Step 6: Create Main Demo Application

Update `src/main.rs`:

```rust
mod accessibility;
mod components;
mod keyboard;
mod testing;

use accessibility::{AccessibilityManager, AccessibilityConfig, AccessibilityMode, TextSize, ColorContrast, MotionPreference};
use components::{AccessibleBarChart, AccessibleLineChart, AccessiblePieChart};
use keyboard::KeyboardNavigation;
use testing::AccessibilityTester;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Accessibility Demo Tutorial...");
    
    // Initialize accessibility manager
    let manager = AccessibilityManager::new();
    
    // Create accessibility configurations for different modes
    let standard_config = AccessibilityConfig {
        mode: AccessibilityMode::Standard,
        language: "en".to_string(),
        text_size: TextSize::Normal,
        color_contrast: ColorContrast::Standard,
        motion_preference: MotionPreference::Full,
    };
    
    let screen_reader_config = AccessibilityConfig {
        mode: AccessibilityMode::ScreenReader,
        language: "en".to_string(),
        text_size: TextSize::Normal,
        color_contrast: ColorContrast::High,
        motion_preference: MotionPreference::None,
    };
    
    let cognitive_config = AccessibilityConfig {
        mode: AccessibilityMode::CognitiveSupport,
        language: "en".to_string(),
        text_size: TextSize::Large,
        color_contrast: ColorContrast::Enhanced,
        motion_preference: MotionPreference::Reduced,
    };
    
    // Demo 1: Bar Chart with Different Accessibility Modes
    println!("\n=== Bar Chart Accessibility Demo ===");
    
    let bar_labels = vec![
        "Q1 Sales".to_string(),
        "Q2 Sales".to_string(),
        "Q3 Sales".to_string(),
        "Q4 Sales".to_string(),
    ];
    let bar_values = vec![120.0, 180.0, 220.0, 195.0];
    
    // Standard mode
    let standard_bar_chart = AccessibleBarChart::new(
        "Quarterly Sales Report".to_string(),
        bar_labels.clone(),
        bar_values.clone(),
        standard_config.clone(),
    );
    
    let standard_metadata = standard_bar_chart.generate_accessibility_metadata();
    println!("Standard Mode:");
    println!("  Alt Text: {}", standard_metadata.alt_text);
    println!("  Live Region: {}", standard_metadata.live_region);
    
    // Screen reader mode
    let sr_bar_chart = AccessibleBarChart::new(
        "Quarterly Sales Report".to_string(),
        bar_labels.clone(),
        bar_values.clone(),
        screen_reader_config.clone(),
    );
    
    let sr_metadata = sr_bar_chart.generate_accessibility_metadata();
    println!("\nScreen Reader Mode:");
    println!("  Alt Text: {}", sr_metadata.alt_text);
    println!("  Live Region: {}", sr_metadata.live_region);
    
    // Cognitive support mode
    let cognitive_bar_chart = AccessibleBarChart::new(
        "Quarterly Sales Report".to_string(),
        bar_labels.clone(),
        bar_values.clone(),
        cognitive_config.clone(),
    );
    
    let cognitive_metadata = cognitive_bar_chart.generate_accessibility_metadata();
    println!("\nCognitive Support Mode:");
    println!("  Alt Text: {}", cognitive_metadata.alt_text);
    println!("  Live Region: {}", cognitive_metadata.live_region);
    
    // Demo 2: Line Chart
    println!("\n=== Line Chart Accessibility Demo ===");
    
    let line_x_labels = vec![
        "Jan".to_string(),
        "Feb".to_string(),
        "Mar".to_string(),
        "Apr".to_string(),
        "May".to_string(),
        "Jun".to_string(),
    ];
    let line_y_values = vec![10.0, 25.0, 30.0, 20.0, 40.0, 35.0];
    
    let line_chart = AccessibleLineChart::new(
        "Monthly Performance".to_string(),
        line_x_labels.clone(),
        line_y_values.clone(),
        screen_reader_config.clone(),
    );
    
    let line_metadata = line_chart.generate_accessibility_metadata();
    println!("Line Chart (Screen Reader Mode):");
    println!("  Alt Text: {}", line_metadata.alt_text);
    
    // Demo 3: Pie Chart
    println!("\n=== Pie Chart Accessibility Demo ===");
    
    let pie_labels = vec![
        "Product A".to_string(),
        "Product B".to_string(),
        "Product C".to_string(),
        "Product D".to_string(),
    ];
    let pie_values = vec![30.0, 25.0, 20.0, 25.0];
    
    let pie_chart = AccessiblePieChart::new(
        "Market Share Distribution".to_string(),
        pie_labels.clone(),
        pie_values.clone(),
        screen_reader_config.clone(),
    );
    
    let pie_metadata = pie_chart.generate_accessibility_metadata();
    println!("Pie Chart (Screen Reader Mode):");
    println!("  Alt Text: {}", pie_metadata.alt_text);
    
    // Demo 4: Keyboard Navigation
    println!("\n=== Keyboard Navigation Demo ===");
    
    let nav = KeyboardNavigation::new(bar_labels.len());
    println!("{}", nav.get_shortcut_help());
    
    // Demo 5: Color Contrast
    println!("\n=== Color Contrast Demo ===");
    
    let base_colors = vec![
        "#FF5733".to_string(),
        "#33FF57".to_string(),
        "#3357FF".to_string(),
    ];
    
    let high_contrast_colors = manager.apply_color_contrast(&base_colors, ColorContrast::High);
    println!("Base Colors: {:?}", base_colors);
    println!("High Contrast Colors: {:?}", high_contrast_colors);
    
    // Demo 6: Text Scaling
    println!("\n=== Text Scaling Demo ===");
    
    let base_font_size = 16u32;
    let large_font_size = manager.apply_text_scaling(base_font_size, TextSize::Large);
    let xl_font_size = manager.apply_text_scaling(base_font_size, TextSize::ExtraLarge);
    
    println!("Base Font Size: {}px", base_font_size);
    println!("Large Font Size: {}px", large_font_size);
    println!("Extra Large Font Size: {}px", xl_font_size);
    
    // Demo 7: Motion Preferences
    println!("\n=== Motion Preferences Demo ===");
    
    let should_animate_full = !manager.should_reduce_motion(MotionPreference::Full);
    let should_animate_reduced = !manager.should_reduce_motion(MotionPreference::Reduced);
    let should_animate_none = !manager.should_reduce_motion(MotionPreference::None);
    
    println!("Full Motion Enabled: {}", should_animate_full);
    println!("Reduced Motion Enabled: {}", should_animate_reduced);
    println!("No Motion Enabled: {}", should_animate_none);
    
    // Demo 8: Comprehensive Testing
    println!("\n=== Comprehensive Accessibility Testing ===");
    
    let tester = AccessibilityTester::new();
    println!("{}", tester.run_comprehensive_test());
    
    println!("\nAccessibility Demo Tutorial completed successfully!");
    
    Ok(())
}
```

## Step 7: Add Additional Dependencies

Update `Cargo.toml` with additional dependencies for accessibility testing:

```toml
[dependencies]
visualization_context = { path = "../../packages/visualization_context" }
bevy = "0.16"
plotters = "0.3"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
accesskit = "0.10"
accesskit_unix = { version = "0.7", optional = true }
accesskit_windows = { version = "0.12", optional = true }
accesskit_macos = { version = "0.7", optional = true }

[features]
default = []
unix = ["accesskit_unix"]
windows = ["accesskit_windows"]
macos = ["accesskit_macos"]
```

## Step 8: Testing the Implementation

To test your implementation:

1. Run the accessibility demo application:

```bash
cargo run
```

You should see output similar to:

```
Starting Accessibility Demo Tutorial...

=== Bar Chart Accessibility Demo ===
Standard Mode:
  Alt Text: Bar chart showing 4 data points. Values range from 120 to 220. Highest value is 220 in Q3 Sales.
  Live Region: off

Screen Reader Mode:
  Alt Text: Bar chart with 4 categories. Data points: Q1 Sales: 120, Q2 Sales: 180, Q3 Sales: 220, Q4 Sales: 195. Use keyboard arrows to navigate between bars.
  Live Region: polite

Cognitive Support Mode:
  Alt Text: Simple bar chart. The highest bar is Q3 Sales with a value of 220. This shows the largest amount in the data.
  Live Region: off

=== Line Chart Accessibility Demo ===
Line Chart (Screen Reader Mode):
  Alt Text: Line chart with 6 data points. Starting at 10, ending at 35. Net change of 25 (+250.0%). Use keyboard arrows to follow the trend line.

=== Pie Chart Accessibility Demo ===
Pie Chart (Screen Reader Mode):
  Alt Text: Pie chart with 4 segments. Data distribution: Product A: 30.0%, Product B: 25.0%, Product C: 20.0%, Product D: 25.0%. Use keyboard arrows to navigate between segments.

=== Keyboard Navigation Demo ===
Keyboard Navigation Shortcuts:
  Tab: Move to next element
  Shift+Tab: Move to previous element
  Enter: Activate current element
  ArrowUp: Move up in chart
  ArrowDown: Move down in chart
  ArrowLeft: Move left in chart
  ArrowRight: Move right in chart
  Home: Go to first data point
  End: Go to last data point
  T: Go to chart title
  L: Go to legend
  D: Go to data points
  R: Refresh chart
  Plus: Zoom in
  Minus: Zoom out

=== Color Contrast Demo ===
Base Colors: ["#FF5733", "#33FF57", "#3357FF"]
High Contrast Colors: ["#000000", "#000000", "#000000"]

=== Text Scaling Demo ===
Base Font Size: 16px
Large Font Size: 19px
Extra Large Font Size: 24px

=== Motion Preferences Demo ===
Full Motion Enabled: true
Reduced Motion Enabled: false
No Motion Enabled: false

=== Comprehensive Accessibility Testing ===
=== Accessibility Comprehensive Test ===

--- Bar Chart Testing ---
Standard Mode Alt Text: Bar chart showing 4 data points. Values range from 100 to 200. Highest value is 200 in March.
Screen Reader Mode Alt Text: Bar chart with 4 categories. Data points: January: 100, February: 150, March: 200, April: 175. Use keyboard arrows to navigate between bars.
Cognitive Support Mode Alt Text: Simple bar chart. The highest bar is March with a value of 200. This shows the largest amount in the data.

--- Keyboard Navigation Testing ---
Keyboard Navigation Shortcuts:
  Tab: Move to next element
  Shift+Tab: Move to previous element
  Enter: Activate current element
  ArrowUp: Move up in chart
  ArrowDown: Move down in chart
  ArrowLeft: Move left in chart
  ArrowRight: Move right in chart
  Home: Go to first data point
  End: Go to last data point
  T: Go to chart title
  L: Go to legend
  D: Go to data points
  R: Refresh chart
  Plus: Zoom in
  Minus: Zoom out

--- Color Contrast Testing ---
Base Colors: ["#FF0000", "#00FF00", "#0000FF"]
High Contrast Colors: ["#000000", "#000000", "#000000"]

=== Test Complete ===

Accessibility Demo Tutorial completed successfully!
```

## Advanced Accessibility Features

### 1. ARIA Implementation

For web-based visualizations, implement ARIA attributes:

```rust
pub struct AriaAttributes {
    pub role: String,
    pub label: String,
    pub described_by: String,
    pub controls: String,
    pub hidden: bool,
}

impl AriaAttributes {
    pub fn for_chart(title: &str, description: &str) -> Self {
        Self {
            role: "img".to_string(),
            label: title.to_string(),
            described_by: format!("{}-desc", title.to_lowercase().replace(" ", "-")),
            controls: "".to_string(),
            hidden: false,
        }
    }
    
    pub fn to_html_attributes(&self) -> String {
        format!(
            "role=\"{}\" aria-label=\"{}\" aria-describedby=\"{}\" {}",
            self.role,
            self.label,
            self.described_by,
            if self.hidden { "aria-hidden=\"true\"" } else { "" }
        )
    }
}
```

### 2. Focus Management

Implement advanced focus management:

```rust
pub struct FocusManager {
    focusable_elements: Vec<String>,
    current_focus: usize,
    focus_ring_visible: bool,
}

impl FocusManager {
    pub fn new(elements: Vec<String>) -> Self {
        Self {
            focusable_elements: elements,
            current_focus: 0,
            focus_ring_visible: true,
        }
    }
    
    pub fn move_focus_forward(&mut self) {
        self.current_focus = (self.current_focus + 1) % self.focusable_elements.len();
    }
    
    pub fn move_focus_backward(&mut self) {
        if self.current_focus == 0 {
            self.current_focus = self.focusable_elements.len() - 1;
        } else {
            self.current_focus -= 1;
        }
    }
    
    pub fn get_focused_element(&self) -> &str {
        &self.focusable_elements[self.current_focus]
    }
    
    pub fn toggle_focus_ring(&mut self) {
        self.focus_ring_visible = !self.focus_ring_visible;
    }
}
```

### 3. Screen Reader Announcements

Implement dynamic screen reader announcements:

```rust
pub struct ScreenReaderAnnouncer {
    announcements: Vec<String>,
    live_region: String,
}

impl ScreenReaderAnnouncer {
    pub fn new() -> Self {
        Self {
            announcements: Vec::new(),
            live_region: "polite".to_string(),
        }
    }
    
    pub fn announce(&mut self, message: String) {
        self.announcements.push(message);
        // In a real implementation, this would update a DOM element
        // with aria-live attribute
        println!("Screen reader announcement: {}", message);
    }
    
    pub fn announce_chart_interaction(&mut self, chart_title: &str, action: &str, details: &str) {
        let message = format!("In chart {}, {} {}", chart_title, action, details);
        self.announce(message);
    }
    
    pub fn announce_data_point(&mut self, label: &str, value: f64) {
        let message = format!("Data point: {} with value {}", label, value);
        self.announce(message);
    }
}
```

## Testing with Screen Readers

### Manual Testing Process

1. **NVDA (Windows)**
   - Download and install NVDA screen reader
   - Navigate through your visualizations using Tab and arrow keys
   - Verify alt text is read correctly
   - Check keyboard navigation works as expected

2. **VoiceOver (macOS)**
   - Enable VoiceOver (Cmd+F5)
   - Use VoiceOver navigation commands
   - Verify accessibility metadata is properly announced

3. **JAWS (Windows)**
   - If available, test with JAWS screen reader
   - Follow similar navigation patterns
   - Verify compatibility with different screen readers

### Automated Testing

For automated accessibility testing, consider integrating tools like:

```rust
// Example integration with accessibility testing frameworks
#[cfg(test)]
mod accessibility_tests {
    use super::*;
    
    #[test]
    fn test_alt_text_quality() {
        let manager = AccessibilityManager::new();
        let labels = vec!["A".to_string(), "B".to_string()];
        let values = vec![10.0, 20.0];
        
        let config = AccessibilityConfig {
            mode: AccessibilityMode::Standard,
            language: "en".to_string(),
            text_size: TextSize::Normal,
            color_contrast: ColorContrast::Standard,
            motion_preference: MotionPreference::Full,
        };
        
        let alt_text = manager.generate_alt_text("bar", &labels, &values, &config);
        
        // Check that alt text contains key information
        assert!(alt_text.contains("Bar chart"));
        assert!(alt_text.contains("data points"));
        assert!(alt_text.contains("10") && alt_text.contains("20"));
    }
    
    #[test]
    fn test_keyboard_shortcuts() {
        let nav = KeyboardNavigation::new(5);
        let shortcuts = nav.get_shortcuts();
        
        // Verify essential keyboard shortcuts are present
        assert!(shortcuts.contains_key("Tab"));
        assert!(shortcuts.contains_key("ArrowUp"));
        assert!(shortcuts.contains_key("T")); // Title navigation
    }
}
```

## Compliance Considerations

### WCAG 2.1 Compliance

Ensure your visualizations meet WCAG 2.1 standards:

1. **Perceivable**
   - Alt text for all non-text content (Level A)
   - Color contrast ratio of at least 4.5:1 (Level AA)
   - Text alternatives for charts and graphs (Level A)

2. **Operable**
   - Keyboard accessible (Level A)
   - Navigable interface (Level AA)
   - Sufficient time for content interaction (Level A)

3. **Understandable**
   - Predictable navigation (Level AA)
   - Input assistance (Level AA)

4. **Robust**
   - Compatible with assistive technologies (Level A)

### Section 508 Compliance

For US federal compliance:

- Ensure all electronic and information technology is accessible
- Follow Section 508 standards for web-based intranet and internet information and applications

## Troubleshooting

### Common Issues

1. **Screen Reader Not Reading Alt Text**
   - Verify alt text is properly associated with visualization elements
   - Check that elements have appropriate ARIA roles
   - Ensure content is not marked as hidden from screen readers

2. **Keyboard Navigation Not Working**
   - Verify focusable elements have proper tab indices
   - Check that event listeners are properly attached
   - Ensure no JavaScript errors are preventing navigation

3. **Color Contrast Issues**
   - Use automated tools to check contrast ratios
   - Test with different color blindness simulators
   - Provide alternative color schemes

### Debugging Tips

1. **Enable Accessibility Inspector**
   - Use browser developer tools to inspect accessibility properties
   - Verify ARIA attributes are correctly applied
   - Check computed accessibility tree

2. **Test with Multiple Screen Readers**
   - Different screen readers may interpret content differently
   - Test with NVDA, JAWS, and VoiceOver if possible
   - Verify compatibility across platforms

3. **Use Automated Testing Tools**
   - Integrate accessibility linters in your development workflow
   - Run automated tests to catch common accessibility issues
   - Perform regular accessibility audits

## Next Steps

After completing this tutorial, consider exploring:

1. [Visualization Architecture Guide](../developer/visualization_architecture.md) - Understand the system architecture
2. [Complex Dashboard Tutorial](./complex_dashboard.md) - Learn to create advanced dashboard layouts
3. [Visualization Setup Guide](../developer/visualization_setup.md) - Review setup procedures

## Conclusion

You've successfully completed the accessibility demo tutorial! You've learned how to:

- Implement comprehensive accessibility metadata for different user needs
- Create screen reader-optimized visualizations
- Design keyboard-navigable interfaces
- Test accessibility features effectively
- Understand compliance requirements for accessibility

This knowledge will help you create inclusive visualizations that work for all users, regardless of their abilities or the assistive technologies they use. Accessibility is not just a feature—it's a fundamental aspect of good design that benefits everyone.