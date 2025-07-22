# Enhanced P2P Networking Architecture

## 1. Overview
This document outlines the enhanced peer-to-peer networking architecture for the CPC platform, designed to support decentralized social media applications. The architecture builds on libp2p to provide robust, scalable, and efficient networking capabilities.

## 2. Architecture Components

### 2.1 Custom Protocol Handler
- **Protocol ID**: `/cpc/1.0.0`
- **Message Types**:
  - `EVENT`: User-generated content
  - `DISCOVERY`: Peer information exchange
  - `CONTROL`: Connection management
- **Serialization**: Protocol Buffers for efficiency
- **Prioritization**: Three-level priority system (LOW, MEDIUM, HIGH)

### 2.2 Connection Manager
- **State Machine**:
  ```mermaid
  graph LR
    A[Disconnected] -->|Connect| B[Connecting]
    B -->|Success| C[Connected]
    C -->|Error| A
    C -->|Graceful| D[Disconnecting]
    D --> A
    A -->|Retry| B
  ```
- **Retry Strategies**:
  - Exponential backoff with jitter
  - Priority-based retry queues
- **Metrics Tracking**:
  - Latency
  - Packet loss
  - Throughput

### 2.3 Peer Discovery
- **Three-tier Approach**:
  1. Bootstrap Nodes (initial connection)
  2. Kademlia DHT (decentralized discovery)
  3. mDNS (local network discovery)
- **Peer Scoring**:
  ```rust
  struct PeerScore {
      uptime: f32,           // 0.0 - 1.0
      responsiveness: f32,   // Response time score
      stability: f32,        // Connection stability
  }
  ```

### 2.4 Message Router
- **Features**:
  - Priority queues (high/medium/low)
  - End-to-end encryption (NOISE protocol)
  - Message deduplication
  - Automatic chunking for large payloads
- **Routing Logic**:
  ```
  if peer is direct:
      send_direct(message)
  elif peer is reachable via relay:
      send_via_relay(message)
  else:
      queue_for_retry(message)
  ```

## 3. NetworkHandler Interface

```rust
pub enum Priority { Low, Medium, High }
pub enum BroadcastScope { All, Neighbors, Group }

pub struct NetworkHandler {
    swarm: Swarm<CpcProtocol>,
    connection_manager: ConnectionManager,
    discovery: DiscoveryService,
    // ...
}

impl NetworkHandler {
    /// Initialize with configuration
    pub async fn new(config: NetworkConfig) -> Result<Self> { ... }
    
    /// Send message to specific peer
    pub fn send_message(
        &mut self,
        peer_id: PeerId,
        message: Message,
        priority: Priority
    ) -> Result<()> { ... }
    
    /// Broadcast message to multiple peers
    pub fn broadcast(
        &mut self,
        message: Message,
        priority: Priority,
        scope: BroadcastScope
    ) { ... }
    
    /// Initiate peer discovery
    pub fn discover_peers(&mut self, target: DiscoverTarget) { ... }
    
    /// Get connection state for peer
    pub fn connection_state(&self, peer_id: &PeerId) -> ConnectionState { ... }
}
```

## 4. Protocol Specification

### Message Structure (Protobuf)
```protobuf
message CpcMessage {
  enum Priority {
    LOW = 0;
    MEDIUM = 1;
    HIGH = 2;
  }
  
  enum Type {
    EVENT = 0;
    DISCOVERY = 1;
    CONTROL = 2;
  }

  uint64 nonce = 1;        // Unique message ID
  Priority priority = 2;   // Message priority
  Type message_type = 3;   // Message type
  bytes payload = 4;       // Serialized payload
  bytes signature = 5;     // Cryptographic signature
}
```

## 5. Cross-Platform Support

### Android/iOS Considerations
- **Threading**: Separate networking thread with Bevy integration
- **Battery Optimization**:
  - Adaptive heartbeat intervals
  - Batch message processing
- **Background Operation**:
  - Foreground service notification (Android)
  - Background tasks (iOS)

### NAT Traversal Techniques
- **AutoNAT**: Automatic NAT detection
- **Relay Protocol**: Relay nodes for unreachable peers
- **Hole Punching**: UDP hole punching for direct connections

## 6. Performance Optimization
- **Connection Pooling**: Reuse connections when possible
- **Compression**: LZ4 for message payloads
- **Async I/O**: Tokio runtime for efficient resource usage
- **Load Shedding**: Discard low-priority messages under heavy load

## 7. Security Measures
- **Peer Authentication**: Cryptographic handshake
- **Message Signing**: Prevent tampering
- **Rate Limiting**: Prevent DoS attacks
- **Encryption**: End-to-end encryption for all messages

## 8. Integration Points
- **Bevy Events**: Convert network events to Bevy events
- **Platform-Specific Code**:
  - Android: JNI bridge
  - iOS: Swift interface
  - Desktop: Direct integration

## 9. Future Extensions
- Ephemeral messaging
- Decentralized storage integration
- Reputation systems