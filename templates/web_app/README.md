# CPC Web App Template

A minimal starter template for CPC web applications.

## Overview

This template provides a basic structure for building web applications using the CPC web_core package. It includes preconfigured routing, styling, and example components.

## Features

- Preconfigured Trunk.toml for building WebAssembly applications
- Basic routing setup with Yew Router
- Example component using web_core
- Auth integration example

## Getting Started

1. Copy this template to create a new web application:
   ```bash
   cp -r templates/web_app apps/my_new_app
   ```

2. Update the `Cargo.toml` file with your app's name and details

3. Build and run the application:
   ```bash
   trunk serve
   ```

## Structure

```
templates/web_app/
├── Cargo.toml          # Project dependencies
├── Trunk.toml          # Trunk configuration
├── index.html          # HTML entry point
├── src/
│   ├── lib.rs          # Main application logic
│   └── main.rs         # WASM entry point
```

## Dependencies

- `yew` - Frontend framework
- `yew-router` - Routing for Yew applications
- `web_core` - Shared CPC web functionality

## License

This project is licensed under the CPC License.