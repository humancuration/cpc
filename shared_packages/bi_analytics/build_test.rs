//! Build test to ensure all modules compile correctly

#[cfg(test)]
mod build_tests {
    #[test]
    fn test_module_imports() {
        // This test ensures all modules can be imported correctly
        use bi_analytics::*;
        use bi_analytics::engine::*;
        use bi_analytics::privacy::*;
        use bi_analytics::cooperative_values::*;
        use bi_analytics::pipeline::*;
        use bi_analytics::pipeline::sources::*;
        use bi_analytics::visualization::*;
        
        // If this compiles, the imports work
        assert!(true);
    }
}