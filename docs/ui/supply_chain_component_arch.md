# Supply Chain Component: Architectural Documentation

## 1. Overview

This document outlines the architecture for the **Supply Chain Visualization Component**. This component is a vertical slice designed to provide detailed, real-time insights into a product's supply chain. It ports and significantly enhances functionality from the legacy Android application, with a strong focus on business intelligence features for cooperatives and ethical businesses.

Users will be able to:
- Visualize multi-tier supply chains geographically and chronologically.
- Inspect ethical certifications and standards at each node.
- Analyze environmental impact metrics, such as carbon footprint per transportation segment.
- Receive real-time updates on supply chain events.
- Access cooperative-specific metrics like fair wage verification and profit-sharing data.

## 2. Architectural Principles

The implementation will strictly adhere to our established architectural principles:

- **Hexagonal Architecture**: The core business logic in `cpc-core` is completely decoupled from the UI (Yew), API (GraphQL), and desktop integration (Tauri). Interactions are managed through well-defined ports (service traits).
- **Screaming Architecture**: The project structure will clearly communicate its purpose. All new core logic related to the supply chain will reside under `packages/cpc-core/src/supply_chain/`.
- **Vertical Slices**: The entire feature, from the Yew component to the p2panda data layer, is designed as a self-contained, cohesive unit to ensure high cohesion and low coupling.
- **Rust-first**: All new logic, from UI to data services, will be implemented in Rust.

## 3. High-Level Dependency Diagram

This diagram illustrates the flow of data from the p2panda network to the UI component.

```mermaid
graph TD
    subgraph Yew UI (cpc-platform)
        A[SupplyChainDisplay Component] -->|Calls| B(Tauri Command: get_supply_chain);
        A -->|Listens for| C(Tauri Event: 'supply-chain-update');
    end

    subgraph Tauri Backend (cpc-platform)
        B -->|Invokes| D{SupplyChainService};
        E[Tauri Event Emitter] -->|Receives from| C;
    end

    subgraph Core Logic (cpc-core)
        D -->|Interacts with| F[p2panda Client];
        D -->|Processes data via| G[Data Models];
        D -->|Notifies via| H(Broadcast Channel);
    end
    
    subgraph GraphQL API (backend)
        I[GraphQL Subscription] -->|Listens to| H;
        J[GraphQL Query] -->|Calls| D;
    end

    F -->|Fetches/Stores| K[(p2panda Network)];
    H -->|Pushes to| C
```

## 4. Detailed Implementation Guide

### 4.1. Core Domain: Data Models and Service

**Location**: `packages/cpc-core/src/`

1.  **Create a new module**: `supply_chain/mod.rs`. This will house all core logic for this feature.
2.  **Define Data Models** in `packages/cpc-core/src/supply_chain/models.rs`: These structs will be the canonical representation of our supply chain data.

    ```rust
    // packages/cpc-core/src/supply_chain/models.rs
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct SupplyChain {
        pub product_id: String,
        pub nodes: Vec<SupplyChainNode>,
        pub segments: Vec<TransportationSegment>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct SupplyChainNode {
        pub id: String,
        pub node_type: NodeType,
        pub name: String, // e.g., "FairTrade Coffee Cooperative"
        pub location: String, // e.g., "Antioquia, Colombia"
        pub coordinates: (f64, f64), // (latitude, longitude)
        pub certifications: Vec<EthicalCertification>,
        pub cooperative_metrics: Option<CooperativeMetrics>,
    }
    
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum NodeType { RawMaterial, Manufacturer, Distributor, Retailer }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct EthicalCertification {
        pub name: String, // e.g., "Fair Trade Certified"
        pub authority: String, // e.g., "Fairtrade International"
        pub validation_date: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CooperativeMetrics {
        pub fair_wage_verified: bool,
        pub profit_sharing_percentage: f32,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TransportationSegment {
        pub from_node_id: String,
        pub to_node_id: String,
        pub method: TransportMethod,
        pub duration_hours: u32,
        pub environmental_impact: EnvironmentalImpact,
        pub cost: TransportCost,
    }
    
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum TransportMethod { Ship, Truck, Plane, Train }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct EnvironmentalImpact {
        pub carbon_footprint_kg_co2: f64,
        // other metrics can be added here
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TransportCost {
        pub amount: f64,
        pub currency: String,
    }
    ```

3.  **Define the Service** in `packages/cpc-core/src/supply_chain/service.rs`: This service will implement the core business logic.

    ```rust
    // packages/cpc-core/src/supply_chain/service.rs
    use super::models::SupplyChain;
    use anyhow::Result;
    use tokio::sync::broadcast;

    // A simplified representation of the p2panda client
    struct P2PandaClient;
    impl P2PandaClient {
        async fn get_supply_chain_data(&self, product_id: &str) -> Result<SupplyChain> {
            // Logic to fetch, decrypt, and parse data from p2panda
            unimplemented!();
        }
    }

    pub struct SupplyChainService {
        p2p_client: P2PandaClient,
        // For real-time updates
        update_notifier: broadcast::Sender<SupplyChain>,
    }

    impl SupplyChainService {
        pub fn new() -> Self {
            let (tx, _rx) = broadcast::channel(100);
            Self {
                p2p_client: P2PandaClient {},
                update_notifier: tx,
            }
        }
        
        /// Retrieves the full supply chain for a given product.
        pub async fn get_supply_chain(&self, product_id: &str) -> Result<SupplyChain> {
            self.p2p_client.get_supply_chain_data(product_id).await
        }
        
        /// Subscribes to real-time updates for a supply chain.
        pub fn get_update_stream(&self) -> broadcast::Receiver<SupplyChain> {
            self.update_notifier.subscribe()
        }

        // A future method that will be called when an update occurs
        // pub async fn publish_update(&self, chain: SupplyChain) -> Result<()> {
        //     self.update_notifier.send(chain)?;
        //     Ok(())
        // }
    }
    ```

### 4.2. GraphQL API Layer

**Location**: `apps/backend/src/graphql/`

1.  **Create `supply_chain.rs`**: This new file will contain the GraphQL schema definitions for the supply chain. We will expose the core models.

    ```rust
    // apps/backend/src/graphql/supply_chain.rs
    use async_graphql::*;
    use cpc_core::supply_chain::{models, service::SupplyChainService};
    use futures::Stream;
    use async_stream::stream;

    // Expose core models via GraphQL types
    #[derive(SimpleObject)]
    struct SupplyChainNode { /* ... fields ... */ }
    // ... other object types ...

    #[derive(Default)]
    pub struct SupplyChainQueryRoot;

    #[Object]
    impl SupplyChainQueryRoot {
        async fn supply_chain(&self, ctx: &Context<'_>, product_id: String) -> Result<models::SupplyChain> {
            let service = ctx.data_unchecked::<SupplyChainService>();
            service.get_supply_chain(&product_id).await.map_err(|e| e.into())
        }
    }
    
    #[derive(Default)]
    pub struct SupplyChainMutationRoot;

    #[Object]
    impl SupplyChainMutationRoot {
        // Example: Mutation to add a new certification
        async fn add_certification(&self, /*...args...*/ ) -> Result<bool> {
            // 1. Update data in p2panda
            // 2. Broadcast the change via SupplyChainService
            unimplemented!()
        }
    }

    #[derive(Default)]
    pub struct SupplyChainSubscriptionRoot;
    
    #[Subscription]
    impl SupplyChainSubscriptionRoot {
        async fn supply_chain_updates(&self, ctx: &Context<'_>, product_id: String) -> impl Stream<Item = Result<models::SupplyChain>> {
            let service = ctx.data_unchecked::<SupplyChainService>();
            let mut rx = service.get_update_stream();

            stream! {
                while let Ok(update) = rx.recv().await {
                    if update.product_id == product_id {
                        yield Ok(update);
                    }
                }
            }
        }
    }
    ```

2.  **Integrate into `schema.rs`**:

    ```rust
    // apps/backend/src/graphql/schema.rs
    // ... other use statements
    use crate::graphql::supply_chain::{SupplyChainQueryRoot, SupplyChainMutationRoot, SupplyChainSubscriptionRoot};

    #[derive(MergedObject, Default)]
    pub struct RootQuery(AssetBrowserQuery, ImpactQuery, ProductQueryRoot, SupplyChainQueryRoot);

    #[derive(MergedObject, Default)]
    pub struct RootMutation(AssetBrowserMutation, ImpactMutation, ProductMutationRoot, SupplyChainMutationRoot);

    #[derive(MergedSubscription, Default)]
    pub struct RootSubscription(AssetBrowserSubscription, ImpactSubscription, ProductSubscriptionRoot, SupplyChainSubscriptionRoot);
    ```
    
### 4.3. Tauri Desktop Integration

**Location**: `apps/cpc-platform/src-tauri/`

1.  **Create `supply_chain_commands.rs`**: To keep concerns separated, we will define supply chain-specific commands here.

    ```rust
    // apps/cpc-platform/src-tauri/src/supply_chain_commands.rs
    use tauri::State;
    use cpc_core::supply_chain::{service::SupplyChainService, models::SupplyChain};
    
    // Define a serializable error type
    #[derive(serde::Serialize, Clone)]
    pub struct CommandError { message: String }
    
    impl<E: std::error::Error> From<E> for CommandError {
        fn from(err: E) -> Self { Self { message: err.to_string() } }
    }

    #[tauri::command]
    pub async fn get_supply_chain(
        product_id: String,
        service: State<'_, SupplyChainService>
    ) -> Result<SupplyChain, CommandError> {
        Ok(service.get_supply_chain(&product_id).await?)
    }

    #[tauri::command]
    pub async fn subscribe_to_supply_chain_updates(
        window: tauri::Window,
        product_id: String,
        service: State<'_, SupplyChainService>
    ) -> Result<(), CommandError> {
        let mut rx = service.get_update_stream();
        tokio::spawn(async move {
            while let Ok(update) = rx.recv().await {
                if update.product_id == product_id {
                    window.emit("supply-chain-update", Some(update)).unwrap();
                }
            }
        });
        Ok(())
    }
    ```
2. **Update `main.rs`** to register the commands and service.
    ```rust
    // In main() function
    let supply_chain_service = SupplyChainService::new();
    
    tauri::Builder::default()
        .manage(supply_chain_service) // Add service to Tauri's state
        .invoke_handler(tauri::generate_handler![
            // ... other commands
            crate::supply_chain_commands::get_supply_chain,
            crate::supply_chain_commands::subscribe_to_supply_chain_updates
        ])
    ```

### 4.4. Frontend Yew Component

**Location**: `apps/cpc-platform/src-yew/src/components/product/`

This component is built to:
1.  Receive its data from a parent component (e.g., a page-level container).
2.  Use `plotters-rs` and `plotters-canvas` to render BI visualizations.
3.  Display the rich data from our new models, including certifications and cooperative metrics.

A key feature will be the visualization. We can use `plotters` to create a scatter plot of `(longitude, latitude)` for the supply chain nodes, overlaid on a simple world map projection or a blank canvas. An interactive chart showing the carbon footprint breakdown per segment would also be a valuable addition.

## 5. List of File Modifications and Creations

**New Files:**
- `docs/ui/supply_chain_component_arch.md`
- `packages/cpc-core/src/supply_chain/mod.rs`
- `packages/cpc-core/src/supply_chain/models.rs`
- `packages/cpc-core/src/supply_chain/service.rs`
- `apps/backend/src/graphql/supply_chain.rs`
- `apps/cpc-platform/src-tauri/src/supply_chain_commands.rs`
- `apps/cpc-platform/src-yew/src/components/product/supply_chain_charts.rs`

**Modified Files:**
- `packages/cpc-core/src/services/mod.rs`: To add `pub mod supply_chain;`.
- `apps/backend/src/graphql/schema.rs`: To merge the new supply chain GraphQL roots.
- `apps/backend/src/main.rs`: To initialize and manage `SupplyChainService` in the GraphQL context.
- `apps/cpc-platform/src-tauri/src/main.rs`: To register the new commands and manage the `SupplyChainService` state.
- `apps/cpc-platform/src-yew/src/components/product/supply_chain.rs`: Complete refactor to use new data models and `plotters-rs`.
- `apps/cpc-platform/src-yew/src/pages/product/details.rs`: To fetch supply chain data via GraphQL and pass it to the display component.
- `Cargo.toml` in `apps/cpc-platform/` to add `plotters` and `plotters-canvas`.

## 6. Validation Plan

This plan focuses on verifying the business logic and data flow.

1.  **Data Integrity Validation**:
    - **Scenario**: Fetch a product with a known supply chain from p2panda.
    - **Expected Outcome**: The `SupplyChainService` correctly deserializes the data into the `SupplyChain` model without loss or corruption of data. All fields, including nested ones like `EthicalCertification`, must be correctly populated.

2.  **Business Logic Validation**:
    - **Scenario**: Create a mock `SupplyChain` object with multiple transportation segments.
    - **Expected Outcome**: The UI component must calculate and display the *total* carbon footprint by correctly summing the `carbon_footprint_kg_co2` from each `TransportationSegment`.

3.  **Real-time Update Validation**:
    - **Scenario**: While subscribed to updates for `product-A`, trigger a mutation that updates a node in `product-A`'s supply chain.
    - **Expected Outcome**: The UI should receive a `supply-chain-update` event and re-render to show the new information immediately, without requiring a manual refresh.
    - **Scenario**: Trigger a mutation for `product-B`.
    - **Expected Outcome**: The UI subscribed to `product-A` should *not* receive an event.