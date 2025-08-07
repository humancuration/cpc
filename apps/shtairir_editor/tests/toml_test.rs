#[test]
fn test_toml_serialization() {
    // Test that we can use the toml crate
    let data = std::collections::HashMap::from([
        ("name", "test"),
        ("version", "1.0.0"),
    ]);
    
    let toml_string = toml::to_string(&data).unwrap();
    assert!(toml_string.contains("name = \"test\""));
    assert!(toml_string.contains("version = \"1.0.0\""));
    
    // Test parsing back
    let parsed: std::collections::HashMap<String, String> = toml::from_str(&toml_string).unwrap();
    assert_eq!(parsed.get("name"), Some(&"test".to_string()));
    assert_eq!(parsed.get("version"), Some(&"1.0.0".to_string()));
}