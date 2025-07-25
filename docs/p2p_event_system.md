# P2P Event System Design

## 1. Event Propagation Flowchart

This section details the flow of events from a client (e.g., Android UI or Bevy Engine), through the unified Rust `NetworkHandler`, across the P2P network, and to other clients. The architecture is now unified across all platforms.

```mermaid
graph TD
    A[Client UI<br>(Android, Bevy, etc.)] -- JNI/FFI Call --> B{Unified Rust Core};
    B -- Event --> C[EventSystem];
    C --- D[NetworkHandler];
    D -- Forwards Event --> E[p2panda Swarm];
    E -- Gossip/Direct Send --> F[Other Peers];
    F -- Receives Event --> G[p2panda Swarm];
    G -- Forwards Event --> H[NetworkHandler];
    H --- I[EventSystem];
    I -- Processes Event --> J{Unified Rust Core};
    J -- Callback/Update --> K[Client UI];
```

## 2. Message Format Specification

Events will be serialized using Protocol Buffers for efficiency and cross-platform compatibility.

```proto
// events.proto
syntax = "proto3";

package p2p_events;

enum EventType {
  UI_INTERACTION = 0;
  GAME_STATE_UPDATE = 1;
  NETWORK_COMMAND = 2;
}

message P2PEvent {
  // Unique identifier for the event, can be a hash of the content.
  string event_id = 1;
  // Lamport timestamp or other logical clock for ordering.
  fixed64 timestamp = 2;
  // Type of the event.
  EventType type = 3;
  // Identifier of the source device.
  string source_device_id = 4;
  // The actual event data, serialized.
  bytes payload = 5;
  // Vector clock for conflict detection and resolution.
  map<string, uint64> vector_clock = 6;
}
```

## 3. Conflict Resolution Matrix

This matrix will define how to handle event collisions based on event type and vector clocks.

| Event Type          | Conflict Detection | Resolution Strategy                                |
|---------------------|--------------------|----------------------------------------------------|
| UI Interaction      | Vector Clock       | Last Write Wins (based on timestamp)               |
| Game State Update   | Vector Clock       | State-specific merge logic; may require user input |
| Network Command     | Vector Clock       | Prioritize command from authoritative peer         |

## 4. Latency Compensation Strategies

This section will outline techniques to mitigate network latency.

- **Client-Side Prediction**: The UI will predict the outcome of an action and update immediately, rolling back if a conflict is detected later.
- **Event Prioritization**: Events will be processed based on their type, with UI interactions having higher priority than background state updates.
- **Incremental Compression**: For frequent updates, only the deltas will be sent to reduce payload size.
- **Event Deduplication**: The receiver will keep track of recent event IDs to discard duplicates.
## 5. Synchronization Protocol

The synchronization and reconciliation protocol is a critical component for ensuring data consistency across the P2P network. It defines the mechanisms for conflict resolution, event ordering, and state convergence.

A detailed specification for this protocol is available in the [P2P Synchronization and Reconciliation Protocol](./synchronization_protocol.md) document.

### 5.1. Consistency Levels

- **Eventual Consistency:** This is the default guarantee. The system ensures that all peers will eventually converge to the same state, assuming network partitions are eventually healed.
- **Strong Consistency:** Not guaranteed by the base protocol. Use cases requiring strong consistency will need specialized implementations, potentially involving authority nodes or consensus algorithms.

### 5.2. Convergence Criteria

An individual peer is considered "converged" when it has successfully processed all causally preceding events and has no outstanding conflicts in its pending event queue. The network as a whole is considered converged when all peers have reached this state.