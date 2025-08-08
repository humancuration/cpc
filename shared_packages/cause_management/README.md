# Cause Management Service

## Overview

The Cause Management Service is a shared package within the CPC platform that provides functionality for managing causes for donations. This service allows users to create, update, delete, and list causes that can be used for charitable donations within the platform.

## Features

- Create new causes for donations
- Update existing causes
- Delete causes
- List causes with pagination
- Track total donations for each cause
- Statistical analysis of donation patterns (with `statistics` feature)
- Impact measurement for causes (with `statistics` feature)
- Donation forecasting with confidence intervals (with `statistics` feature)
- BI visualization integration (with `statistics` and `visualization` features)
- gRPC service interface for internal communication
- PostgreSQL database storage

## Components

### Models

The service defines the following core data models:

- `Cause`: Represents a charitable cause with name, description, image, and donation tracking
- `CreateCauseRequest`: Request structure for creating new causes
- `UpdateCauseRequest`: Request structure for updating existing causes
- `ListCausesRequest`: Request structure for listing causes with pagination
- `CauseError`: Error types for cause management operations

### Statistical Models (requires `statistics` feature)

- `DonationForecast`: Forecast results for donation predictions with confidence intervals
- `DonationTrend`: Donation trend analysis results
- `ImpactAnalysis`: Impact analysis results for causes
- `ImpactMetric`: Types of impact metrics (LivesImpacted, EnvironmentalBenefit, etc.)

### Repository

The `CauseRepository` trait defines the database interface for cause management:

- `create_cause`: Create a new cause in the database
- `find_cause_by_id`: Retrieve a cause by its ID
- `update_cause`: Update an existing cause
- `delete_cause`: Delete a cause
- `list_causes`: List causes with pagination
- `add_donation_to_cause`: Add a donation amount to a cause's total

The `PostgresCauseRepository` provides a PostgreSQL implementation of the repository.

### Service

The `CauseServiceImpl` implements the gRPC service interface defined in the cpay.proto file:

- `create_cause`: Create a new cause
- `get_cause`: Retrieve a specific cause
- `update_cause`: Update an existing cause
- `delete_cause`: Delete a cause
- `list_causes`: List causes with pagination
- `get_donation_forecast`: Get donation forecast for a cause (requires `statistics` feature)
- `get_impact_analysis`: Get impact analysis for a cause (requires `statistics` feature)

## Database Schema

The service uses the following database table:

```sql
CREATE TABLE causes (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    image_url VARCHAR(512),
    total_donations DECIMAL(20, 2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
```

## Protocol Buffers

The service uses the cpay.proto definition which extends the CPay service with cause management functionality:

- `CreateCauseRequest`/`CreateCauseResponse`: For creating new causes
- `GetCauseRequest`/`GetCauseResponse`: For retrieving specific causes
- `UpdateCauseRequest`/`UpdateCauseResponse`: For updating existing causes
- `DeleteCauseRequest`/`DeleteCauseResponse`: For deleting causes
- `ListCausesRequest`/`ListCausesResponse`: For listing causes with pagination

## Integration

The Cause Management Service is designed to work alongside the CPay Core service. While it provides cause management functionality, payment processing is handled by the CPay Core service. The services communicate through the shared cpay.proto interface.

## Usage

To use the Cause Management Service:

1. Set up a PostgreSQL database
2. Run the database migrations
3. Configure the database connection
4. Start the gRPC server
5. Connect to the service using a gRPC client

## Dependencies

- `tokio`: Async runtime
- `tonic`/`prost`: gRPC implementation
- `sqlx`: Database access
- `uuid`: Unique identifiers
- `chrono`: Time handling
- `rust_decimal`: Precise decimal arithmetic
- `serde`: Serialization
- `tracing`: Logging
- `cpc_statistics_core`: Statistical analysis (optional, with `statistics` feature)
- `cpc_bi_visualization`: BI visualization (optional, with `visualization` feature)

## Statistical Analysis Features

The cause management service includes advanced statistical analysis capabilities that can be enabled with the `statistics` feature flag. These features include:

- **Donation Forecasting**: Predict future donation patterns with confidence intervals
- **Trend Analysis**: Identify and analyze donation trends over time
- **Impact Measurement**: Measure the real-world impact of causes with statistical confidence
- **BI Visualization**: Integrate with the BI visualization system for interactive charts (requires both `statistics` and `visualization` features)

To enable statistical features, add the feature flag to your Cargo.toml:

```toml
[dependencies]
cause_management = { path = "../cause_management", features = ["statistics"] }
```

For visualization integration:

```toml
[dependencies]
cause_management = { path = "../cause_management", features = ["statistics", "visualization"] }
```

See `docs/statistical_analysis_causes.md` for detailed documentation on statistical features.
- `cpc_statistics_core`: Statistical analysis (optional, with `statistics` feature)
- `cpc_bi_visualization`: BI visualization (optional, with `visualization` feature)