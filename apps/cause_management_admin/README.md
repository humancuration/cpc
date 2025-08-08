# Cause Management Admin Dashboard

## Overview

The Cause Management Admin Dashboard is a web-based application that provides administrators with insights into the effectiveness of cause impact visualizations across the CPC platform. It tracks engagement metrics, measures correlation between visualization usage and cause engagement rates, and provides tools for continuous improvement.

## Features

- **Impact Analytics**: Comprehensive dashboard showing engagement metrics, cause effectiveness, community impact, and feedback summaries
- **Real-time Monitoring**: Track visualization engagement, contribution effectiveness, and community validation in real-time
- **Recommendation Engine**: Automated suggestions for improving cause visualizations based on data analysis
- **Feedback Integration**: Collect and analyze community feedback on visualization effectiveness
- **Cross-platform Integration**: Connect with learning, volunteering, and financial impact metrics for holistic analysis

## Technology Stack

- **Frontend**: HTML5, CSS3, JavaScript
- **Backend**: Rust (Axum web framework)
- **Data Visualization**: Canvas API for charts and graphs
- **API**: RESTful endpoints for data retrieval and updates

## Architecture

The dashboard follows a client-server architecture:

```
┌─────────────────┐    ┌──────────────────────┐    ┌──────────────────────────┐
│   Web Browser   │◄──►│  Axum REST Server    │◄──►│  Cause Impact Tracker    │
└─────────────────┘    └──────────────────────┘    └──────────────────────────┘
                                │                              │
                                ▼                              ▼
                      ┌──────────────────────┐    ┌──────────────────────────┐
                      │  Analytics Dashboard │    │  Feedback Collection     │
                      └──────────────────────┘    └──────────────────────────┘
```

## API Endpoints

### Health Check
- `GET /health` - Check if the service is running

### Dashboard Data
- `GET /api/dashboard/summary` - Retrieve comprehensive dashboard metrics

### Tracking
- `POST /api/tracking/engagement` - Track visualization engagement
- `POST /api/tracking/cause-engagement` - Track cause engagement correlation
- `POST /api/tracking/validation` - Record community validation

## Development

### Prerequisites

- Rust toolchain (latest stable version)
- Cargo package manager

### Building

```bash
cd apps/cause_management_admin
cargo build
```

### Running

```bash
cd apps/cause_management_admin
cargo run
```

The dashboard will be available at `http://localhost:3004`

### Testing

```bash
cd apps/cause_management_admin
cargo test
```

## Integration with Cause Impact Tracker

The dashboard integrates with the `cause_impact_tracker` shared package to provide:

1. **Data Collection**: Track engagement with cause visualization components
2. **Correlation Analysis**: Measure correlation between visualization usage and cause engagement rates
3. **Effectiveness Monitoring**: Monitor contribution effectiveness and community transformation metrics
4. **Community Validation**: Record community validation of cause impact
5. **Privacy Compliance**: Ensure privacy-preserving data collection respecting consent levels

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Write tests if applicable
5. Submit a pull request

## License

This project is part of the CPC platform and is licensed under the CPC license.