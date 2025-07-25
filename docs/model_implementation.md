# Model Implementation Documentation

## Overview
We've implemented Rust models for Proposal, FeedItem, and Product in the shared cpc-core crate. These models are now used throughout the system and have replaced the placeholder types in the serialization module.

## Key Changes

### Proposal Model
```rust
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub votes_for: u32,
    pub votes_against: u32,
    pub author_id: String,
}
```
- Added validation for all fields
- Includes all fields from Kotlin implementation plus voting fields

### FeedItem Model
```rust
pub enum FeedItem {
    Post {
        id: String,
        content: String,
        author_id: String,
        likes: u32,
        comments: u32,
    },
    Proposal(Proposal),
}
```
- Enum with Post and Proposal variants
- Proposal variant uses the Proposal struct

### Product Model
```rust
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    // ... additional fields ...
    pub supply_chain: Option<SupplyChain>,
}
```
- Includes all fields from Kotlin implementation
- Added price field that was missing initially
- Includes comprehensive validation

### Supply Chain Models
```rust
pub struct SupplyChain {
    pub nodes: Vec<SupplyChainNode>,
    pub segments: Vec<TransportationSegment>,
}

pub struct SupplyChainNode {
    pub id: String,
    pub node_type: NodeType,
    pub location: String,
    pub company: String,
    pub timestamp: String,
    pub coordinates: Coordinates,
}

pub struct TransportationSegment {
    pub from_node_id: String,
    pub to_node_id: String,
    pub method: TransportMethod,
    pub duration_hours: u32,
    pub carbon_footprint: f32,
}
```
- Complete implementation instead of placeholder
- Matches fields used in serialization

## Serialization Updates
- All Android* prefixed types removed
- Conversion implementations updated to use new Rust models
- Supply chain serialization now uses proper enum values

## Validation
- Used `validator` crate for data integrity
- Added constraints for:
  - Required fields
  - String length limits
  - Numeric value ranges
  - Non-negative numbers

## Dependencies
Added to Cargo.toml:
```toml
validator = { version = "0.16", features = ["derive"] }