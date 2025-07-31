# OAuth2 Crate Architecture

## Overview
Reusable authentication crate for CPC apps supporting major providers: TikTok, Google, Facebook, Twitter. Integrates with consent_manager for permission handling and supports web/Tauri flows.

## Hexagonal Architecture Layers

### Domain Layer
- **OAuthProvider**: Enum (TikTok, Google, Facebook, Twitter)
- **OAuthToken**: Struct { access_token, refresh_token, expires_at, scopes }
- **OAuthProfile**: Struct { id, name, email, provider }
- **AuthError**: Enum with provider-specific errors
- **AuthConfig**: Struct with client_id, client_secret, redirect_uris

### Application Layer
- **AuthService**: Orchestrates authentication flows
  - start_auth(provider, redirect_uri) -> AuthRequest
  - handle_callback(code, state) -> OAuthToken
  - refresh_token(token) -> OAuthToken
- **TokenService**: Manages token storage/retrieval
  - store_token(user_id, token) 
  - get_token(user_id, provider) 
  - delete_token(user_id, provider)

### Infrastructure Layer
- **Provider Adapters**: 
  - TikTokAdapter, GoogleAdapter, etc. (implement OAuthProvider trait)
  - Handle provider-specific OAuth flows
- **Storage Adapters**:
  - EncryptedSledStorage: Local storage with AES-GCM encryption
  - PostgresStorage: Centralized encrypted storage
- **API Interfaces**:
  - gRPC: AuthService gRPC implementation
  - REST: Webhook endpoints for callbacks

## Integration with consent_manager
- Before token issuance, verify scope permissions via ConsentService
- Audit events for all auth operations using audit_framework
- Consent required for sensitive scopes (email, profile)

## Web/Native Flow Support
- **Web**: Standard redirect flow with state tokens
- **Native**: Deep link handling with custom URI schemes
- Tauri-specific: Use wry for custom protocol handling

## Security
- Token encryption at rest using RustCrypto (AES-GCM)
- Secure token storage with key management
- CSRF protection via state parameters
- Rate limiting on auth endpoints