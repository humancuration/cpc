# CPC Dashboard Application

The Dashboard application provides a centralized view of key metrics and visualizations for CPC platform users.

## Features

- Customizable dashboard layouts
- Real-time data visualization
- Cross-app data integration
- Accessibility support
- Progressive loading of visualizations

## Architecture

The dashboard follows a client-server architecture where the frontend is built with Yew and communicates with the API Gateway for data and visualization requests.

```
Dashboard App
├── Frontend (Yew/WASM)
│   ├── Visualization components
│   ├── Layout manager
│   └── User preferences
└── Backend (API Gateway)
    ├── Visualization service
    ├── Data aggregation
    └── User management
```

## Visualization Integration

The dashboard integrates with the visualization system through the API Gateway:

1. **Context Propagation**: All visualization requests include context headers
2. **Progressive Loading**: Visualizations load in phases for better UX
3. **Accessibility**: Screen reader support with enhanced metadata
4. **Caching**: Edge caching for improved performance

## Visual Scripting

The dashboard includes a powerful visual scripting system for creating custom data pipelines and visualizations.

For details on creating and executing visual scripts, see the [Visual Scripting Execution Documentation](docs/visual_scripting_execution.md).

## Development

To run the dashboard application:

```bash
cd apps/dashboard
cargo build
```

For development with live reloading:

```bash
# Terminal 1 - Run API Gateway
cd apps/api_gateway
cargo run

# Terminal 2 - Run Dashboard frontend
cd apps/dashboard
trunk serve
```

## Configuration

The dashboard can be configured through environment variables:

- `API_GATEWAY_URL` - URL of the API Gateway
- `DEFAULT_DASHBOARD_LAYOUT` - Default layout for new users
- `CACHE_REFRESH_INTERVAL` - How often to refresh cached data