Yew/Rust code for the UI goes here

This directory contains the Yew-based frontend components for the CPC platform. The UI is built using Rust and compiles to WebAssembly for high performance.

## Structure
- `src/` - Main Yew application code
- `components/` - Reusable Yew components
- `pages/` - Page-level components
- `hooks/` - Custom Yew hooks
- `services/` - API and service integrations
- `types/` - TypeScript-like type definitions (using Rust structs/enums)
- `styles/` - CSS/Tailwind styling

## Technology Stack
- **Yew**: Rust framework for building web applications
- **WASM**: WebAssembly compilation target
- **Tauri**: Desktop integration
- **GraphQL**: API communication layer

## Migration Notes
- All previous Svelte components have been refactored to Yew
- File extensions changed from `.svelte` to `.rs`
- State management moved from Svelte stores to Yew's `use_state` and `use_reducer`
- Component props use Yew's `Properties` derive macro instead of Svelte's export syntax