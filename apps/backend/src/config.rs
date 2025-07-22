use std::env;
use std::str::FromStr;
use thiserror::Error;
use hex;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing environment variable: {0}")]
    MissingEnv(String),
    
    #[error("invalid value for environment variable {0}: {1}")]
    InvalidValue(String, String),
    
    #[error("hex decoding error: {0}")]
    HexError(#[from] hex::FromHexError),
}

#[derive(Debug, Clone)]
pub enum Environment {
    Dev,
    Test,
    Prod,
}

impl FromStr for Environment {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Environment::Dev),
            "test" => Ok(Environment::Test),
            "prod" => Ok(Environment::Prod),
            _ => Err(ConfigError::InvalidValue(
                "CPC_ENV".to_string(),
                format!("'{}' is not a valid environment", s)
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub jwt_secret: String,
    pub encryption_key: [u8; 32],
    pub environment: Environment,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let port_str = env::var("CPC_BACKEND_PORT")
            .unwrap_or_else(|_| "8080".to_string());
        let port = port_str.parse::<u16>()
            .map_err(|_| ConfigError::InvalidValue(
                "CPC_BACKEND_PORT".to_string(),
                "must be a number between 1-65535".to_string()
            ))?;

        // Validate port range
        if port < 1 {
            return Err(ConfigError::InvalidValue(
                "CPC_BACKEND_PORT".to_string(),
                "must be at least 1".to_string()
            ));
        }

        let jwt_secret = env::var("CPC_JWT_SECRET")
            .map_err(|_| ConfigError::MissingEnv("CPC_JWT_SECRET".to_string()))?;
        
        // Validate JWT secret length
        if jwt_secret.len() < 32 {
            return Err(ConfigError::InvalidValue(
                "CPC_JWT_SECRET".to_string(),
                "must be at least 32 characters".to_string()
            ));
        }

        let encryption_key_hex = env::var("CPC_ENCRYPTION_KEY")
            .map_err(|_| ConfigError::MissingEnv("CPC_ENCRYPTION_KEY".to_string()))?;
        
        // Validate encryption key format
        if !encryption_key_hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ConfigError::InvalidValue(
                "CPC_ENCRYPTION_KEY".to_string(),
                "must be a 64-character hex string".to_string()
            ));
        }
        
        let encryption_key_bytes = hex::decode(&encryption_key_hex)?;
        if encryption_key_bytes.len() != 32 {
            return Err(ConfigError::InvalidValue(
                "CPC_ENCRYPTION_KEY".to_string(),
                "must be exactly 32 bytes (64 hex characters)".to_string()
            ));
        }

        let mut encryption_key = [0u8; 32];
        encryption_key.copy_from_slice(&encryption_key_bytes);

        let env_str = env::var("CPC_ENV")
            .unwrap_or_else(|_| "dev".to_string());
        let environment = Environment::from_str(&env_str)?;

        Ok(Config {
            port,
            jwt_secret,
            encryption_key,
            environment,
        })
    }
}