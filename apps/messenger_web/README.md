# Messenger Web

Web frontend for the CPC Messenger application built with Yew and Rust.

## Features

- Real-time messaging interface
- Reaction support (like, heart, laugh, etc.)
- Threaded conversations
- Group management
- Media sharing (images, documents, audio, video)
- Responsive design

## Development

### Prerequisites

- Rust and Cargo
- Trunk (for building the web frontend)
- wasm-bindgen-cli

### Setup

1. Install Trunk:
   ```bash
   cargo install trunk
   ```

2. Install wasm-bindgen-cli:
   ```bash
   cargo install wasm-bindgen-cli
   ```

### Running the Development Server

```bash
trunk serve
```

This will start a development server at http://localhost:3001

### Building for Production

```bash
trunk build --release
```

This will generate optimized static files in the `dist` directory.

## Architecture

The web frontend is built with:

- **Yew**: Rust framework for building web applications
- **Stylist**: CSS-in-Rust styling solution
- **GraphQL**: For API communication with the backend
- **WebSockets**: For real-time messaging

The UI components are shared with the desktop application through the `cpc-messenger` crate with the `web` feature enabled.

## Project Structure

```
src/
├── components/     # Re-exports of UI components from messenger_win64
├── pages/          # Page components for routing
├── services/       # Service layers for API communication
├── lib.rs          # Main application component and routing
└── main.rs         # Entry point for the WASM application
```

## Dependencies

- `yew`: Frontend framework
- `yew-router`: Routing for Yew applications
- `stylist`: CSS-in-Rust styling
- `cpc-messenger`: Shared backend and UI components
- Various `gloo` crates for web APIs
- `wasm-bindgen` for WASM integration

## License

This project is licensed under the CPC License.