//! Selection tools module for the Art application
//!
//! This module contains implementations of various selection tools
//! that users can use to select areas of their artwork.

pub mod rectangle;
pub mod lasso;
pub mod magic_wand;

#[cfg(test)]
mod test;