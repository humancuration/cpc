# Backend Configuration Guide

## Environment Variables

| Variable | Required | Default | Validation | Description |
|----------|----------|---------|------------|-------------|
| `CPC_BACKEND_PORT` | No | 8080 | 1-65535 | Port for backend server |
| `CPC_JWT_SECRET` | Yes | - | Min 32 chars | Secret for JWT token signing |
| `CPC_ENCRYPTION_KEY` | Yes | - | 64-char hex | Encryption key (32 bytes) |
| `CPC_ENV` | No | dev | dev/test/prod | Runtime environment |

## Security Best Practices
- Store secrets in secure vaults (not in source control)
- Rotate secrets regularly in production
- Use different secrets per environment

## Example Configurations

### Development
```bash
export CPC_JWT_SECRET="my_development_secret_at_least_32_characters_long"
export CPC_ENCRYPTION_KEY="00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff"
```

### Production
```bash
# Generate secure secrets:
export CPC_JWT_SECRET=$(openssl rand -base64 48)
export CPC_ENCRYPTION_KEY=$(openssl rand -hex 32)

# Set other variables
export CPC_BACKEND_PORT=443
export CPC_ENV=prod
```

## Future: Configuration Reloading
We're considering adding live configuration reloading using these approaches:
- SIGHUP signal handler
- Configuration version endpoint
- File watcher for config changes

Note: Encryption key changes would require special handling as existing encrypted data might become inaccessible.