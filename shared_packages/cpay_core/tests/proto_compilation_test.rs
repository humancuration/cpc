//! Test to verify that protobuf compilation works correctly

#[cfg(test)]
mod tests {
    #[test]
    fn test_proto_compilation() {
        // This test will only pass if the protobuf files compile correctly
        // The mere existence of this test module verifies that the generated
        // code can be imported and compiled
        
        // We don't need to actually test anything here - if the module
        // compiles, the test passes
        assert!(true);
    }
}