//! Community Route Optimization shared package
//!
//! This module provides shared functionality for route optimization in community events,
//! including algorithms to minimize transportation costs and environmental impact.

pub mod optimization;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}