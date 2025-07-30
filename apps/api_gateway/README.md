# CPC API Gateway

The Central API Gateway for the CPC platform serves as the single entry point for all client requests, routing them to the appropriate backend services.

## Architecture

The API Gateway implements the patterns described in the [BI Visualization Integration Strategy](../../docs/bi_visualization_integration_strategy.md):

1. **Single Entry Point**: All requests flow through the gateway
2. **Protocol Agnosticism**: Supports GraphQL, REST, and WebSocket through consistent routing
3. **Federation-Aware**: Respects CPC's cooperative principles in access control decisions

## Features

- Visualization request routing
- Authentication and authorization
- Rate limiting
- Caching
- Monitoring and metrics collection
- Protocol translation between clients and backend services

## Services Integration

The gateway integrates with the following services:

- **BI Analytics Service**: For visualization generation and management
- **Messaging Service**: For real-time communication
- **Task Management Service**: For task tracking and scheduling
- **Finance Service**: For financial data and reporting
- **Health Service**: For health data management

## Routes

### Visualization Routes

- `GET /visualizations/:id` - Get 3D visualization data
- `GET /visualizations/:id/image` - Get visualization as image
- `GET /visualizations/:id/ws` - WebSocket connection for live updates
- `POST /visualizations` - Create new visualization

## Development

To run the API Gateway:

```bash
cd apps/api_gateway
cargo run
```

The gateway will start on `http://localhost:3001`.

## Configuration

The gateway can be configured through environment variables:

- `GATEWAY_PORT` - Port to listen on (default: 3001)
- `BI_SERVICE_URL` - URL of the BI Analytics service
- `CACHE_TTL` - Default cache TTL in seconds