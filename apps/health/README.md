# Health Module

The Health module is a comprehensive health management system built with Rust, following hexagonal architecture principles with vertical slices. It provides features for vital sign tracking, condition management, and health trend analysis, with integrations for wearables and privacy-preserving data sharing.

## Features

- Vital sign tracking (heart rate, blood pressure, blood glucose, etc.)
- Health condition management
- Fitness tracker integration
- Privacy-preserving p2p data sharing
- 3D health visualizations with Bevy
- Web-based dashboard with Yew
- HIPAA-compliant data handling

## Architecture

The module follows hexagonal (ports and adapters) architecture with vertical slices:

```
apps/health/
├── Cargo.toml
├── MIGRATION_GUIDE.md  # Migration instructions from old health implementations
├── README.md           # Module documentation
└── src/
    ├── lib.rs
    ├── domain/          # Pure business models (VitalSign, HealthCondition)
    │   ├── vital_signs.rs
    │   ├── health_condition.rs
    │   ├── primitives.rs
    │   └── mod.rs
    ├── application/     # Service orchestration (HealthMonitoringService, ConditionTrackingService)
    │   ├── monitoring_service.rs
    │   ├── condition_service.rs
    │   └── mod.rs
    ├── infrastructure/  # Concrete implementations (repositories, p2p, wearables)
    │   ├── database/
    │   │   ├── models.rs
    │   │   ├── repositories.rs
    │   │   └── mod.rs
    │   ├── p2p/
    │   │   ├── data_sharing.rs
    │   │   └── mod.rs
    │   ├── wearables/
    │   │   ├── api_integration.rs
    │   │   └── mod.rs
    │   └── mod.rs
    └── presentation/    # UI components (Bevy, Yew)
        ├── bevy/
        │   ├── health_viz.rs
        │   └── mod.rs
        ├── yew/
        │   ├── components.rs
        │   └── mod.rs
        └── mod.rs
```

## Documentation

For detailed architecture documentation, see:
- [Architecture Overview](../../../docs/architecture/health.md)
- [Implementation Guide](../../../docs/health/ARCHITECTURE.md)
- [Migration Guide](MIGRATION_GUIDE.md)

## Usage

To use the health module in your application:

1. Add `cpc-health` as a dependency in your Cargo.toml
2. Initialize the module in your application startup code
3. Use the provided services, repositories, and UI components

## Contributing

Please read our contributing guidelines before submitting pull requests. All health data handling must comply with HIPAA regulations.

## License

This project will be licensed under a new type of CoopyLeft license which we will address later. This has no license for now.