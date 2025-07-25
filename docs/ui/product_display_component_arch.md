# Product Display Component: Architectural Documentation

## 1. Overview

This document outlines the architecture for the **Product Display Component** within the CPC Platform. This component is a vertical slice of functionality responsible for fetching, displaying, and updating product information in a comprehensive and interactive manner.

It provides users with:
- Core product details (name, description, price).
- A visual breakdown of product costs.
- Real-time updates on data validation status.
- A view into the product's supply chain.

## 2. Architectural Principles

The implementation of this component will strictly adhere to our established architectural principles:

- **Hexagonal Architecture**: The core business logic (in `cpc-core`) will be decoupled from the delivery mechanisms (Yew UI, GraphQL API, Tauri commands). The UI will interact with the core through well-defined ports (service traits).
- **Screaming Architecture**: The project structure (`apps/cpc-platform/src/components/product/`) clearly communicates its purpose: displaying product information.
- **Vertical Slices**: The entire feature, from the UI component to the backend service call, is self-contained. This allows for high cohesion and low coupling between features.
- **Rust-first**: All new logic will be implemented in Rust, from the UI (Yew) to the backend services.

## 3. Detailed Implementation Guide

This section provides a concrete guide for developers to implement the required features.

### 3.1. Cost Breakdown Visualization with `plotters-rs`

The current `cost_breakdown.rs` component uses direct canvas 2D context calls for drawing. This will be replaced with `plotters-rs` to create more robust, maintainable, and interactive charts.

**File to Modify**: `apps/cpc-platform/src/components/product/cost_breakdown.rs`

**Implementation Steps**:

1.  **Add `plotters-canvas` dependency**: This crate provides a `plotters` backend that can draw to an HTML5 canvas element.
2.  **Refactor Drawing Logic**: Create a dedicated function `draw_chart_with_plotters` that takes the canvas element and product data as input. This isolates the plotting logic from the Yew component's rendering lifecycle.
3.  **Use `CanvasBackend`**: Inside `draw_chart_with_plotters`, create a `CanvasBackend` instance tied to the `HtmlCanvasElement` obtained from the `use_node_ref` hook.
4.  **Create the Chart**: Use `plotters`' `ChartBuilder` and pie chart series (`Pie`) to draw the cost breakdown. This is declarative and less error-prone than manual arc calculations.

**Example Code Structure**:

```rust
// in apps/cpc-platform/src/components/product/cost_breakdown.rs

use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

// ... (ProductData and other structs remain the same) ...

#[function_component(CostBreakdown)]
pub fn cost_breakdown(props: &CostBreakdownProps) -> Html {
    let canvas_ref = use_node_ref();
    let product_data = props.product.clone();

    use_effect_with_deps(
        move |(canvas_ref, data)| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                // drawing is now handled by a dedicated, testable function
                if let Err(e) = draw_chart_with_plotters(&canvas, &data) {
                    // Log the error
                    gloo_console::error!(format!("Failed to draw chart: {:?}", e));
                }
            }
            || ()
        },
        (canvas_ref.clone(), product_data),
    );

    html! {
        // ... (HTML structure remains the same, but the canvas will be controlled by plotters) ...
        <canvas ref={canvas_ref} width="400" height="300"></canvas>
        // ...
    }
}

/// Draws a pie chart onto the given canvas element using plotters.
fn draw_chart_with_plotters(
    canvas: &HtmlCanvasElement,
    product: &ProductData,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::with_canvas_object(canvas.clone())
        .ok_or("Failed to create canvas backend")?;
    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let (material_cost, labor_cost) = (
        product.material_cost.unwrap_or(0.0),
        product.labor_cost.unwrap_or(0.0),
    );

    if material_cost + labor_cost == 0.0 {
        // No data to plot, maybe draw a message
        return Ok(());
    }

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .build_pie();

    chart.draw_series(
        Pie::new(
            &[
                ("Material", material_cost),
                ("Labor", labor_cost),
            ],
            &[BLUE, GREEN], // Use plotters colors
            |&_name, &value| format!("{:.0}%", value / (material_cost + labor_cost) * 100.0),
        )
    )?;

    Ok(())
}
```

### 3.2. Real GraphQL Subscription Implementation

The mock subscription in `apps/backend/src/graphql/product.rs` will be replaced with a real implementation using `tokio::sync::broadcast` channels for an event-driven approach.

**Files to Modify**:
- `apps/backend/src/graphql/product.rs`
- `packages/cpc-core/src/services/product_display_service.rs`

**Implementation Steps**:

1.  **Introduce `broadcast` channel in `ProductDisplayService`**: The service will hold the `sender` half of a broadcast channel. This channel will transmit `ValidationUpdate` events.
2.  **Modify `update_product_validation` Mutation**: After successfully updating the validation status in the database, this mutation will broadcast the new `ValidationUpdate` to all active subscribers via the service's broadcast channel.
3.  **Modify `product_validation_updates` Subscription**: This subscription will obtain a `receiver` from the `ProductDisplayService` and stream events from it. This eliminates the polling (`tokio::time::interval`) and provides instant updates.

**Example `product.rs` Structure**:

```rust
// in apps/backend/src/graphql/product.rs
use async_stream::stream;
use tokio_stream::StreamExt;

// ...

#[Subscription]
impl ProductSubscriptionRoot {
    async fn product_validation_updates(
        &self,
        ctx: &Context<'_>,
        product_id: String,
    ) -> impl Stream<Item = Result<ValidationUpdate>> {
        info!("Real-time subscription for product: {}", product_id);
        let service = ctx.data_unchecked::<ProductDisplayService>();
        
        // Subscribe to the broadcast channel in the service
        let mut rx = service.get_validation_update_stream();

        stream! {
            // Stream updates as they are broadcast
            while let Ok(update) = rx.recv().await {
                // Optional: Filter for the specific product_id if the channel is for all products
                if update.product_id == product_id {
                    yield Ok(update);
                }
            }
        }
    }
}

#[Object]
impl ProductMutationRoot {
    async fn update_product_validation(
        &self,
        ctx: &Context<'_>,
        input: ProductValidationInput,
    ) -> Result<bool> {
        info!("Mutation: update_product_validation for product: {}", input.product_id);
        let service = ctx.data_unchecked::<ProductDisplayService>();
        
        // This service method will perform the DB update AND broadcast the change.
        service.update_and_notify(input).await.map_err(|e| e.into())
    }
}
```


### 3.3. Tauri Command Requirements

For the desktop client, we will use Tauri commands to interact directly with our Rust-based services in `cpc-core` for maximum performance. This avoids routing calls through a local web server.

**File to Modify**: `apps/cpc-platform/src-tauri/src/commands.rs` (or a new `apps/cpc-platform/src-tauri/src/product_commands.rs` for better organization).

**Command Signatures**:

The following commands should be defined and exported. They act as the bridge between the Yew frontend and the `cpc-core` services.

```rust
// In a new file: apps/cpc-platform/src-tauri/src/product_commands.rs

use tauri::State;
use cpc_core::services::product_display_service::{ProductDisplayService, ProductQuery, ValidationUpdate};

// This struct would wrap the actual product query result from cpc-core
// to ensure it's serializable over the Tauri bridge.
#[derive(serde::Serialize, Clone)]
pub struct ProductDataView {
    // fields...
}

// Error type that is serializable
#[derive(serde::Serialize, Clone)]
pub struct CommandError {
    message: String,
}

impl From<anyhow::Error> for CommandError {
    fn from(err: anyhow::Error) -> Self {
        CommandError { message: err.to_string() }
    }
}

#[tauri::command]
pub async fn get_product_details(
    id: String,
    service: State<'_, ProductDisplayService> // Assuming service is managed state
) -> Result<ProductDataView, CommandError> {
    // Call the core service directly
    let product = service.get_product(id).await?;
    // Map `product` to `ProductDataView`
    // Ok(product_view)
    unimplemented!()
}

#[tauri::command]
pub async fn subscribe_to_product_validation(
    window: tauri::Window,
    product_id: String,
    service: State<'_, ProductDisplayService>
) -> Result<(), CommandError> {
    let mut rx = service.get_validation_update_stream();
    
    // Asynchronously listen for updates and emit them to the frontend
    tokio::spawn(async move {
        while let Ok(update) = rx.recv().await {
            if update.product_id == product_id {
                window.emit("validation-update", update).unwrap();
            }
        }
    });

    Ok(())
}
```

**Integration**:
- These command handlers must be registered in `apps/cpc-platform/src-tauri/src/main.rs`.
- The Yew frontend will use Tauri's `invoke` function to call these commands. For the subscription, it will use `listen` to receive events.