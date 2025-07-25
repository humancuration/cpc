# P2P Networking Unification Plan

## 1. Executive Summary

This document outlines the plan to resolve the architectural inconsistency between the main P2P networking layer and the Android-specific implementation. The current system has two separate networking stacks: one using a custom `NetworkHandler` and another on Android using `p2panda` directly. This divergence prevents the Android platform from integrating with the `cpc-core` `EventSystem`.

The proposed solution is to refactor the `NetworkHandler` to become a unified wrapper around a `p2panda` `Swarm`. This will create a single, consistent P2P interface for all platforms, allowing seamless integration with the event system while encapsulating platform-specific logic.

## 2. Problem Analysis

- **`cpc-core/src/p2p/mod.rs`**: Defines a `NetworkHandler` with event broadcasting capabilities.
- **`cpc-core/src/events/mod.rs`**: The `EventSystem` singleton requires an instance of `NetworkHandler` for its creation, creating a hard dependency.
- **`cpc-core/src/p2p/android.rs`**: Implements P2P logic using a raw `p2panda::Swarm`, completely bypassing the `NetworkHandler` and, by extension, the `EventSystem`.

This split architecture leads to code duplication, maintenance overhead, and a critical functionality gap on Android.

## 3. Proposed Architecture: The Unified `NetworkHandler`

The core of this plan is to refactor `NetworkHandler` to be the sole manager of the P2P network stack across all platforms.

### 3.1. `NetworkHandler` Structure

The `NetworkHandler` will be modified to encapsulate the `p2panda::Swarm`.

```rust
// In cpc-core/src/p2p/network.rs (new or modified file)

use p2panda::{Swarm, Multiaddr, identity, PeerId};
use p2panda::ping::{Ping, PingConfig}; // Or a more complex behaviour
use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;

// The unified NetworkHandler will manage the Swarm
pub struct NetworkHandler {
    swarm: Arc<Mutex<Swarm<...>>>, // The specific behaviour will be defined
    // other fields like local_peer_id
}

// Global singleton instance for the NetworkHandler
static NETWORK_HANDLER_INSTANCE: OnceCell<Arc<NetworkHandler>> = OnceCell::new();

impl NetworkHandler {
    // A single entry point to get the handler
    pub fn get_instance() -> Arc<NetworkHandler> {
        NETWORK_HANDLER_INSTANCE.get_or_init(|| {
            // Internal logic to initialize the swarm
            let local_key = identity::Keypair::generate_ed25519();
            let local_peer_id = PeerId::from(local_key.public());
            
            let transport = p2panda::development_transport(local_key).unwrap();
            // Replace Ping with our actual, more complex network behaviour
            let behaviour = Ping::new(PingConfig::new().with_keep_alive(true)); 
            let swarm = Swarm::new(transport, behaviour, local_peer_id);

            Arc::new(NetworkHandler {
                swarm: Arc::new(Mutex::new(swarm)),
            })
        }).clone()
    }

    // Methods to interact with the swarm
    pub fn dial_address(&self, addr: Multiaddr) {
        let mut swarm = self.swarm.lock().unwrap();
        swarm.dial_addr(addr).unwrap();
    }

    pub fn broadcast(&self, message: Vec<u8>) {
        // Implementation for broadcasting to all connected peers
    }
    
    // This function will need to be implemented to poll the swarm for events
    // and forward them to the EventSystem. This should run in a background thread.
    pub async fn event_loop(&self) {
        // loop {
        //   let event = self.swarm.lock().unwrap().select_next_some().await;
        //   // handle event, forward to EventSystem
        // }
    }
}
```

### 3.2. Android JNI Layer Refactoring

The JNI functions in `cpc-core/src/p2p/android.rs` will no longer manage their own `Swarm`. Instead, they will interact with the singleton `NetworkHandler`.

```rust
// In cpc-core/src/p2p/android.rs

use super::network::NetworkHandler; // Use the unified handler

#[no_mangle]
pub extern "system" fn Java_com_cpc_P2PNetwork_init_network(
    env: JNIEnv,
    _: JClass,
    config: jni::sys::jstring
) {
    let handler = NetworkHandler::get_instance();
    
    let config_str: String = env.get_string(config.into()).unwrap().into();
    let config_value: serde_json::Value = serde_json::from_str(&config_str).unwrap();

    if let Some(addr_str) = config_value["bootstrap_node"].as_str() {
        if let Ok(addr) = addr_str.parse::<Multiaddr>() {
            handler.dial_address(addr);
            println!("Dialed bootstrap node: {}", addr_str);
        }
    }
    
    // The event loop for the swarm should be started here, likely in a new thread.
    // tokio::spawn(async move { handler.event_loop().await });
}

// Other JNI functions will be simplified to call NetworkHandler methods.
// The global SWARM static variable will be removed.
```

### 3.3. Event System Integration

The `EventSystem` initialization remains largely the same, but it will now receive the *unified* `NetworkHandler` on all platforms, including Android.

```rust
// In cpc-core/src/events/mod.rs

use crate::p2p::NetworkHandler;

impl EventSystem {
    pub fn get_instance() -> Arc<Mutex<Self>> {
        EVENT_SYSTEM_INSTANCE.get_or_init(|| {
            // Get the singleton instance of the unified NetworkHandler
            let network_handler = NetworkHandler::get_instance();
            Arc::new(Mutex::new(EventSystem::new(network_handler)))
        }).clone()
    }
    
    // ... rest of the implementation
}
```

This change ensures that when `EventSystem::broadcast_event` is called, it uses the same underlying `p2panda` swarm on Android as it does on other platforms.

## 4. Implementation Steps

1.  **Refactor `NetworkHandler`**: Move the `NetworkHandler` struct and its implementation into `cpc-core/src/p2p/network.rs`. Modify it to encapsulate the `p2panda` `Swarm` and implement it as a singleton using `once_cell`.
2.  **Create Event Loop**: Implement the `event_loop` method for the `NetworkHandler` to poll the swarm for incoming messages and network events. This will require an async runtime like `tokio`.
3.  **Update Android JNI**: Modify `cpc-core/src/p2p/android.rs` to remove the local `Swarm` and all related logic. The JNI functions should now call methods on the `NetworkHandler::get_instance()`.
4.  **Update `EventSystem`**: Ensure `EventSystem::get_instance` correctly retrieves the `NetworkHandler` singleton.
5.  **Update `p2p/mod.rs`**: Adjust the module's public exports (`pub use`) to expose the new unified `NetworkHandler`.
6.  **Update Documentation**: Amend `docs/p2p_event_system.md` to reflect the unified architecture.

## 5. Conclusion

This plan will create a more robust, maintainable, and consistent P2P networking layer across the entire `cpc` ecosystem. It resolves a critical architectural flaw and enables full feature parity for the Android platform's event handling.