# Finance Admin Dashboard

A web-based admin dashboard for monitoring and analyzing financial impact metrics across the CPC ecosystem.

## Overview

The Finance Admin Dashboard provides a comprehensive interface for financial coordinators to monitor the effectiveness of financial visualizations, track community engagement, analyze feedback, and implement continuous improvements. It integrates with the Financial Impact Tracker library to provide real-time insights into how financial visualizations affect community financial behaviors.

## Features

### Dashboard Overview
- Real-time financial impact metrics
- Engagement analytics and trends
- Community feedback summaries
- Improvement recommendations

### Engagement Metrics
- Visualization usage statistics
- Interaction time and quality scores
- Participation correlation analysis
- User engagement trends

### Feedback Analysis
- Community feedback aggregation
- Sentiment analysis of comments
- Helpfulness ratings and impact scores
- Common themes and suggestions

### Improvement Tools
- A/B testing framework interface
- Personalization recommendation engine
- Community template repository
- Implementation priority suggestions

### Cross-System Impact
- Integration with learning system metrics
- Volunteer coordination impact analysis
- Cause management correlation tracking
- Holistic community engagement metrics

## Architecture

The dashboard is built as a Rust application with two main components:

1. **Backend API Server** - Built with Axum, provides RESTful endpoints for data collection and dashboard metrics
2. **Frontend Web Application** - Built with Yew, provides a responsive web interface for dashboard visualization

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd apps/finance_admin_dashboard

# Build the application
cargo build --release

# Run the backend server
cargo run --bin finance-admin-dashboard

# Run the web frontend (for development)
cargo run --bin finance-admin-dashboard-web --features web
```

## API Endpoints

### Health Check
```
GET /health
```

### Dashboard Metrics
```
GET /api/dashboard/summary
```

### Tracking Endpoints
```
POST /api/tracking/engagement
POST /api/tracking/participation
POST /api/tracking/validation
```

## Development

### Prerequisites
- Rust toolchain (latest stable version)
- wasm-pack (for web frontend development)
- Node.js and npm (for web asset building)

### Building for Web

To build the web frontend for deployment:

```bash
# Install wasm-pack if not already installed
cargo install wasm-pack

# Build the web frontend
wasm-pack build --target web --out-dir pkg
```

### Running Tests

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_tests
```

## Configuration

The dashboard can be configured through environment variables:

- `PORT` - Server port (default: 3003)
- `LOG_LEVEL` - Logging level (default: info)
- `DATABASE_URL` - Database connection string (if using persistent storage)

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting pull requests.

## License

This project is licensed under the CPC License - see the LICENSE file for details.