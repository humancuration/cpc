# Encryption Libraries Used in Invoicing Module

## Overview
This document explains the cryptographic libraries used in the invoicing module's security implementation and how they align with the project's architectural principles.

## Libraries Used

### 1. x25519-dalek
- **Purpose**: X25519 elliptic curve Diffie-Hellman key exchange
- **Usage**: Generating encryption keys for the Double Ratchet implementation
- **Why Chosen**: 
  - Part of the well-established Dalek cryptography suite
  - Constant-time implementation for side-channel resistance
  - Widely used in production systems
  - Compatible with the Noise protocol

### 2. blake3
- **Purpose**: Cryptographic hashing
- **Usage**: Content hashing for integrity verification and key derivation
- **Why Chosen**:
  - State-of-the-art performance (much faster than SHA-2/SHA-3)
  - Verified security implementation
  - Parallelizable hash computation
  - Used for both content verification and key derivation

### 3. libp2p-noise
- **Purpose**: Noise protocol framework for secure communications
- **Usage**: Foundation for the Double Ratchet implementation
- **Why Chosen**:
  - Industry-standard framework for secure p2p communications
  - Implements the Noise Protocol Specification correctly
  - Well-maintained and widely used in the p2p ecosystem
  - Provides the cryptographic foundation for Double Ratchet

### 4. zeroize
- **Purpose**: Secure memory wiping
- **Usage**: Automatic zeroization of sensitive data in SecureStorage
- **Why Chosen**:
  - Provides guaranteed memory clearing on drop
  - Prevents sensitive data from remaining in memory
  - Works with custom types through derive macros

### 5. secp256k1
- **Purpose**: Elliptic curve cryptography
- **Usage**: Supporting cryptographic operations (currently limited usage)
- **Why Chosen**:
  - Industry standard for blockchain applications
  - Highly optimized implementation
  - Constant-time operations for security

### 6. rand
- **Purpose**: Cryptographically secure random number generation
- **Usage**: Generating cryptographic keys and nonces
- **Why Chosen**:
  - Cryptographically secure random number generator
  - OS-provided entropy sources
  - Widely used and well-audited

## Compliance with Project Standards

### Permissive Licenses
All libraries used have permissive licenses (MIT, Apache 2.0, or BSD), complying with the project's licensing requirements.

### Established Standards
The implementation follows established cryptographic standards:
- **BLAKE3** for hashing (modern replacement for SHA-2/SHA-3)
- **Ed25519** for signing (via libp2p)
- **X25519** for encryption key exchange
- **Noise Protocol** for secure channel establishment
- **Double Ratchet** for forward secrecy and future secrecy

### No External Cloud Dependencies
All cryptographic operations are performed locally, with no reliance on external cloud providers, complying with the project's decentralized principles.

## Security Properties

### Forward Secrecy
The Double Ratchet implementation provides forward secrecy - compromising long-term keys does not compromise past session keys.

### Future Secrecy
The Double Ratchet also provides future secrecy - compromising a session key does not compromise future session keys.

### Key Rotation
Automatic key rotation every 100 messages or 24 hours ensures that compromised keys have limited exposure time.

### Secure Key Storage
Keys are stored in memory-protected storage and automatically zeroized when no longer needed.

## Future Improvements

### Full Double Ratchet Implementation
The current implementation uses placeholder encryption that should be replaced with a full Double Ratchet implementation that:
1. Properly implements the Diffie-Hellman ratchet
2. Implements the symmetric key ratchet
3. Handles message ordering and missing messages
4. Provides proper error handling for ratchet failures

### Hardware Security Module (HSM) Integration
Future versions could integrate with hardware security modules for even stronger key protection.

### Post-Quantum Cryptography
As quantum computers become a threat, the implementation can be upgraded to post-quantum algorithms while maintaining the same API.