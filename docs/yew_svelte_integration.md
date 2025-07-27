# Svelte + Yew Integration Pattern

This document outlines the integration pattern for using Yew components within the SvelteKit frontend of the CPC platform.

## Overview

The application employs a Server-Side Rendering (SSR) approach for Yew components. Instead of compiling Yew components to a WebAssembly (WASM) bundle for the client, they are rendered to an HTML string on the Rust backend. The Svelte frontend then calls a Tauri command to fetch this HTML and inject it into the DOM.

This pattern was chosen to:

*   Reduce the client-side bundle size by avoiding a large WASM payload.
*   Allow for a unified Rust codebase across the backend and UI components.
*   Simplify communication between the frontend and backend.

## The Rendering Process

1.  **Svelte Component Request**: A Svelte component that needs to display a Yew component calls a Tauri command.
2.  **Tauri Command Execution**: The command is defined in `apps/cpc-platform/src-tauri/src/main.rs`. It calls a rendering function in the Rust code.
3.  **Yew Server-Side Rendering**: The rendering function, located in `apps/cpc-platform/src-tauri/src/app.rs`, uses `yew::ServerRenderer` to render a Yew component to an HTML string.
4.  **Routing**: The component to be rendered is determined by a Yew router, defined in `apps/cpc-platform/src-tauri/src/router.rs`. The router matches the requested path and returns the corresponding Yew component.
5.  **HTML Injection**: The Svelte component receives the HTML string from the Tauri command and uses the `{@html ...}` directive to inject it into the page.

## How to Mount a Yew Component in Svelte

To mount a Yew component, you need to:

1.  **Define a Yew Component and Route**: Create your Yew component and add a corresponding route to `apps/cpc-platform/src-tauri/src/router.rs`.
2.  **Modify the Tauri Command (If Necessary)**: The existing `render_yew_component` is basic. To support different components, you'll need to modify it to accept a route or component identifier.
3.  **Call the Command from Svelte**: In your Svelte component, call the Tauri command and inject the returned HTML.

### Example: Rendering the `ProductDisplay` Component

Here's how to render the `ProductDisplay` component in `apps/cpc-platform/src/routes/products2/[id]/+page.svelte`.

**1. Update the Yew Router**

First, add the `ProductDisplay` component and a new route to `apps/cpc-platform/src-tauri/src/router.rs`.

```rust
// apps/cpc-platform/src-tauri/src/router.rs

use yew::prelude::*;
use yew_router::prelude::*;
use cpc_core::components::product::display::ProductDisplay; // Assuming this is the path
use crate::forum::community_browser::ui::CommunityBrowser;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/c/:slug")]
    Community { slug: String },
    #[at("/products/:id")] // New route
    Product { id: String },
}

// ... existing code ...

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <CommunityBrowser /> },
        AppRoute::Community { slug } => html! { <CommunityHome slug={slug} /> },
        AppRoute::Product { id } => html! { <ProductDisplay product_id={id} /> }, // New match arm
    }
}
```

**2. Modify the Tauri Command**

Now, update the `render_yew_component` command in `apps/cpc-platform/src-tauri/src/main.rs` to accept a route.

```rust
// apps/cpc-platform/src-tauri/src/main.rs

// ... imports ...

#[tauri::command]
async fn render_yew_component(route: String) -> Result<String, ()> {
    // This is a simplified example. In a real app, you would want a more
    // robust way to set the route for the renderer.
    Ok(app::render_to_string_with_route(&route).await)
}

// ... in main() ...
.invoke_handler(tauri::generate_handler![
    // ... other commands
    render_yew_component,
])
```

You'll also need to update `app.rs` to handle the route.

```rust
// apps/cpc-platform/src-tauri/src/app.rs

// ... imports ...

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    html! {
        <StaticRouter location={props.route.clone()}>
            <Switch<AppRoute> render={switch} />
        </StaticRouter>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct AppProps {
    pub route: String,
}

pub async fn render_to_string_with_route(route: &str) -> String {
    let props = AppProps { route: route.to_string() };
    let renderer = ServerRenderer::<App>::with_props(props);
    renderer.render().await
}
```

**3. Call from Svelte**

Finally, in your Svelte component, call the Tauri command and render the HTML.

```svelte
<!-- apps/cpc-platform/src/routes/products2/[id]/+page.svelte -->

<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { invoke } from '@tauri-apps/api/tauri';

  let yewHtml = '';

  onMount(async () => {
    const route = `/products/${$page.params.id}`;
    yewHtml = await invoke('render_yew_component', { route });
  });
</script>

{#if yewHtml}
  {@html yewHtml}
{:else}
  <p>Loading component...</p>
{/if}

```

This example provides a complete, end-to-end solution for rendering a Yew component in Svelte. You can adapt this pattern for the `InvoiceList` component and other future components.