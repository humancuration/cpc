# Configuration Specification

## Config Structure
```rust
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub log_level: String,
    pub jwt_secret: String,
    pub encryption_key: String,
    pub environment: Environment,
    pub rate_limit: u32, // requests per second
    pub refresh_token_exp: u64, // expiration in seconds
    pub access_token_exp: u64, // expiration in seconds
}

pub enum Environment {
    Dev,
    Test,
    Prod,
}
```

## Environment Variables
| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| DATABASE_URL | Yes | | PostgreSQL connection string |
| PORT | No | 8080 | Server port |
| LOG_LEVEL | No | "info" | Log verbosity (error, warn, info, debug, trace) |
| JWT_SECRET | Yes | | 32+ character secret for JWT signing |
| ENCRYPTION_KEY | Yes | | Base64-encoded 32-byte key for file encryption |
| ENVIRONMENT | No | "dev" | Runtime environment (dev, test, prod) |
| RATE_LIMIT | No | 100 | Requests per second per IP |
| REFRESH_TOKEN_EXP | No | 2592000 | Refresh token expiration (30 days) |
| ACCESS_TOKEN_EXP | No | 3600 | Access token expiration (1 hour) |

## Profile-based Configuration
```rust
impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        
        match cfg.get_string("environment")?.as_str() {
            "prod" => {
                cfg.set_default("log_level", "warn")?;
                cfg.set_default("rate_limit", 50)?;
            }
            "test" => {
                cfg.set_default("log_level", "debug")?;
            }
            _ => {
                cfg.set_default("log_level", "trace")?;
            }
        }
        
        cfg.try_into()
    }
}
```

## Validation Rules
1. JWT_SECRET must be ≥32 characters
2. ENCRYPTION_KEY must be valid base64
3. PORT must be between 1024-65535
4. All expiration times must be ≥300 seconds

## Security Considerations
- Never commit .env files to version control
- Use separate secrets for each environment
- Rotate secrets quarterly or after security incidents
- Validate config on application startup