# Enhanced p2panda Integration for CPC Document Editor

## Architecture Overview

This document details the integration of p2panda's orchestrated P2P framework into the CPC document editor to replace our current placeholder P2P implementation. The integration leverages p2panda's comprehensive suite of peer-to-peer technologies to create a secure, efficient, and robust real-time collaboration system.

### System Architecture Diagram

```
+----------------+      +---------------------+      +----------------+
|  Document UI   |      |   CRDT Document     |      |  p2panda Core  |
|                |      |      System         |      |                |
|  +-----------+ |      |  +---------------+  |      |  +-----------+ |
|  | Operation | |<---->|  | CRDTDocument  |  |<---->|  | Node      | |
|  | Generator | |      |  |               |  |      |  | Management| |
|  +-----------+ |      |  +---------------+  |      |  +-----------+ |
|        |       |      |        |            |      |        |      |
|        v       |      |        v            |      |        v      |
|  +-----------+ |      |  +---------------+  |      |  +-----------+ |
|  |  GraphQL  | |<---->|  |CRDT Operations|  |<---->|  | Transport | |
|  |   API     | |      |  |  Processing   |  |      |  | (QUIC)    | |
|  +-----------+ |      |  +---------------+  |      |  +-----------+ |
|                |      |        |            |      |        |      |
|                |      |        v            |      |        v      |
|                |      |  +---------------+  |      |  +-----------+ |
|                |      |  | Security Layer|  |<---->|  | NAT       | |
|                |      |  | (E2EE, Sig)   |  |      |  | Traversal | |
|                |      |  +---------------+  |      |  | (STUN)    | |
|                |      |        |            |      |  +-----------+ |
+----------------+      +---------------------+      +----------------+
```

### Key Components

1. **CRDT Document System**: Maintains the collaborative document state using CRDT principles
2. **Security Layer**: Implements Ed25519 signatures, BLAKE3 hashing, and Double Ratchet encryption
3. **p2panda Core**: Handles node discovery, connection management, and transport
4. **Transport Layer**: Uses QUIC for low-latency communication with STUN/TURN for NAT traversal

## Technical Specification

### 1. CRDT Operation Format

The document editor will use a modified CRDT operation format compatible with p2panda's requirements:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PandaOperation {
    pub operation: DocumentOperation,  // Original CRDT operation
    pub hash: [u8; 32],               // BLAKE3 hash of operation content
    pub signature: Vec<u8>,           // Ed25519 signature of hash
    pub timestamp: i64,               // Logical timestamp
    pub sequence_number: u64,         // Per-peer sequence number
    pub sender_id: Uuid,              // Sender's node ID
}
```

### 2. Security Implementation

#### Signature and Verification
- Each operation is hashed using BLAKE3
- Hash is signed with sender's Ed25519 private key
- Receivers verify signatures before applying operations

#### End-to-End Encryption
- Double Ratchet algorithm for forward secrecy and post-compromise security
- Session keys established during document initialization
- Each message encrypted with unique ratchet step

#### Content Addressing
- Document states are content-addressed using BLAKE3
- Enables efficient synchronization and conflict resolution

### 3. Connection Management

#### Transport Protocol
- QUIC for low-latency transport
- Connection multiplexing for efficiency
- Built-in congestion control

#### NAT Traversal
- STUN for direct peer connections when possible
- TURN fallback for symmetric NAT scenarios
- Connection reestablishment after network changes

### 4. Offline Support

- Local operation queue persists operations when offline
- Automatic resynchronization when connection restored
- Version vector comparison for conflict detection

## Integration Points

### 1. P2P Network Replacement

Replace `P2PNetwork` with `PandaNetwork`:

```rust
pub struct PandaNetwork {
    node: p2panda::Node,              // p2panda node instance
    document_subscriptions: HashMap<Uuid, p2panda::Subscription>,
    keypair: Keypair,                 // Ed25519 keypair
    ratchet_manager: RatchetManager,  // Double Ratchet manager
}
```

### 2. CRDT Operation Processing

Document operations will flow through this pipeline:

1. UI generates CRDT operation
2. Operation signed and hashed
3. Operation encrypted with Double Ratchet
4. Operation sent via p2panda to document participants
5. Receiving peers: 
   - Decrypt message
   - Verify signature
   - Apply to local CRDT document
   - Broadcast to GraphQL subscribers

### 3. Document Initialization

When a document is initialized:

1. Generate document-specific Double Ratchet session
2. Share initial ratchet state with participants via secure channel
3. Register document namespace with p2panda
4. Subscribe to document operations

## Sequence Diagram: Document Synchronization

```
User A               Document Editor A             p2panda Network             Document Editor B               User B
 |                        |                             |                            |                         |
 | Edit document          |                             |                            |                         |
 |----------------------->|                             |                            |                         |
 |                        | Generate CRDT operation     |                            |                         |
 |                        |---------------------------> |                            |                         |
 |                        | Sign & hash operation       |                            |                         |
 |                        |---------------------------> |                            |                         |
 |                        | Encrypt with Double Ratchet |                            |                         |
 |                        |---------------------------> |                            |                         |
 |                        | Publish to p2panda          |                            |                         |
 |                        |---------------------------->|                            |                         |
 |                        |                             | Propagate to connected      |                         |
 |                        |                             |---------------------------> |                         |
 |                        |                             |                            | Decrypt message         |
 |                        |                             |                            |<------------------------|
 |                        |                             |                            | Verify signature        |
 |                        |                             |                            |<------------------------|
 |                        |                             |                            | Apply to CRDT document  |
 |                        |                             |                            |<------------------------|
 |                        |                             |                            | Notify GraphQL          |
 |                        |                             |                            | subscribers             |
 |                        |                             |                            |------------------------>|
 |                        |                             |                            |                         | Update UI
 |                        |                             |                            |                         |<------------------------|
```

## Migration Plan

### Phase 1: Infrastructure Setup
- Add p2panda dependencies
- Implement core connection management
- Set up cryptographic primitives

### Phase 2: Operation Processing
- Redefine operation format with security features
- Implement signing/verification
- Add encryption/decryption layer

### Phase 3: Integration
- Replace current P2P implementation
- Update CRDT document system
- Ensure GraphQL API compatibility

### Phase 4: Testing & Optimization
- Implement comprehensive tests
- Optimize for performance
- Add monitoring and metrics

## Implementation Tasks for ougcode

1. **p2panda Core Integration**
   - Create `panda_network.rs` module implementing p2panda node
   - Implement node discovery and connection management
   - Set up QUIC transport with STUN/TURN fallback

2. **Security Layer Implementation**
   - Add BLAKE3 hashing for operation content
   - Implement Ed25519 signing and verification
   - Integrate Double Ratchet for E2EE

3. **CRDT Operation Format Update**
   - Redefine `DocumentOperation` to include security metadata
   - Implement CBOR serialization for operations
   - Create operation signing and verification methods

4. **Connection Management**
   - Implement document-specific subscriptions
   - Add session management for Double Ratchet
   - Handle connection loss and reestablishment

5. **Offline Support**
   - Create persistent operation queue
   - Implement conflict resolution on reconnect
   - Add version vector comparison

6. **GraphQL API Integration**
   - Ensure existing API contracts remain compatible
   - Update subscription handling for new P2P layer
   - Add metrics for synchronization performance

7. **Testing**
   - Create integration tests for p2panda communication
   - Implement security validation tests
   - Add performance benchmarks for sync operations

This integration will transform our document editor from a basic collaboration system into a secure, efficient, and robust peer-to-peer application that fully leverages p2panda's capabilities while maintaining our CRDT-based conflict resolution.