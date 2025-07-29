//! Presentation layer for invoicing and quoting

#[cfg(feature = "visualization")]
pub mod bevy;

#[cfg(feature = "web")]
pub mod yew;

// Re-export presentation components
#[cfg(feature = "visualization")]
pub use bevy::InvoicingVisualizationPlugin;

#[cfg(feature = "web")]
pub use yew::InvoicingComponents;