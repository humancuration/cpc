#[cfg(test)]
mod tests {
    use learning_platform_server::config::AppConfig;
    use std::net::SocketAddr;
    
    #[test]
    fn test_config_from_env() {
        // This test will use default values since we're not setting env vars
        let config = AppConfig::from_env();
        assert!(config.is_ok());
        
        let config = config.unwrap();
        assert_eq!(config.database_url, "postgresql://localhost/learning_platform");
        
        let expected_addr: SocketAddr = "127.0.0.1:50051".parse().unwrap();
        assert_eq!(config.server_addr, expected_addr);
        
        assert_eq!(config.jwt_secret, "secret");
    }
}