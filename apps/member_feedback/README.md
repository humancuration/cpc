# Member Feedback App

A web application for community members to provide feedback on financial visualizations and contribute to continuous improvement efforts.

## Overview

The Member Feedback App allows community members to easily provide feedback on financial visualization tools they interact with. This feedback is crucial for the continuous improvement of financial tools and helps coordinators understand what's working well and what needs enhancement.

## Features

### Simple Feedback Collection
- Quick helpful/not helpful ratings
- Detailed feedback forms with ratings and comments
- Mobile-responsive design for easy access

### Detailed Feedback Metrics
- Overall satisfaction ratings (1-5 scale)
- Impact on financial decisions rating
- Understanding improvement rating
- Confidence improvement rating

### Privacy-Focused
- Respects user consent preferences
- Transparent data collection practices
- Secure feedback submission

## Architecture

The app is built as a Rust application with two main components:

1. **Backend API Server** - Built with Axum, provides RESTful endpoints for feedback collection
2. **Frontend Web Application** - Built with Yew, provides a responsive web interface for feedback submission

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd apps/member_feedback

# Build the application
cargo build --release

# Run the backend server
cargo run

# Run the web frontend (for development)
cargo run --features web
```

## API Endpoints

### Health Check
```
GET /health
```

### Feedback Submission
```
POST /api/feedback/quick
POST /api/feedback/detailed
```

### Feedback Statistics
```
GET /api/feedback/stats
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

The app can be configured through environment variables:

- `PORT` - Server port (default: 3004)
- `LOG_LEVEL` - Logging level (default: info)

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting pull requests.

## License

This project is licensed under the CPC License - see the LICENSE file for details.