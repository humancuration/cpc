# API Gateway Documentation

## Overview

The API Gateway serves as the single entry point for all client requests in the CPC platform. It routes requests to the appropriate backend services, handles authentication, rate limiting, caching, and protocol translation.

## Architecture

The gateway follows a modular architecture where each service integration is implemented as a separate module:

```
API Gateway
├── Visualization Module
├── Authentication Module
├── Rate Limiting Module
├── Caching Module
└── Monitoring Module
```

## Visualization Integration

The visualization module handles all requests related to data visualization, including:

- 3D scene data generation
- Image rendering
- WebSocket streaming for real-time updates
- Accessibility metadata generation

### Routes

- `GET /visualizations/:id` - Get 3D visualization data
- `GET /visualizations/:id/image` - Get visualization as image
- `GET /visualizations/:id/ws` - WebSocket connection for live updates
- `POST /visualizations` - Create new visualization

### Request Format

```json
{
  "visualization_id": "uuid",
  "parameters": {
    "width": 800,
    "height": 600,
    "lod_level": 2,
    "accessibility_mode": "screen_reader"
  },
  "context": {
    "app_id": "dashboard",
    "user_id": "uuid",
    "session_token": "jwt"
  }
}
```

### Response Format

```json
{
  "visualization_data": {
    "type": "3d_scene|image|stream",
    "payload": "glTF JSON|base64|stream_id",
    "accessibility": {
      "alt_text": "string",
      "navigation_map": {"key": "position"},
      "live_region": "polite|assertive"
    }
  },
  "metadata": {
    "cache_ttl": 300,
    "lod_config": {"level": 2, "max_points": 1000},
    "compliance_flags": ["pii_redacted"]
  }
}
```

## Authentication

The gateway handles authentication using JWT tokens with CPC-specific claims. It validates tokens and extracts user context for authorization decisions.

## Rate Limiting

Rate limiting is implemented per resource type with configurable limits:

| Resource Type | Baseline Limit | Burst Capacity | Special Cases |
|---------------|----------------|----------------|---------------|
| Static Images | 100/min | 200/min | Dashboard: +50% |
| 3D Scene Data | 20/min | 50/min | Reporting: +30% |
| WebSocket | 5 connections | 10 connections | Collaborative: +200% |

## Caching

The gateway implements a multi-layer caching strategy:

1. **Edge Cache** - Sled-based local caching
2. **Regional Cache** - Redis-based regional caching
3. **Origin Cache** - Service-level caching

## Monitoring

The gateway collects metrics for:

- Request volume by service
- Response times
- Cache hit ratios
- Error rates
- Accessibility usage patterns

## Deployment

The gateway can be deployed as a standalone service or integrated into other applications as a library.