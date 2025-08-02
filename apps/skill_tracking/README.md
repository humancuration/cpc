# Skill Tracking App

A web-based application for tracking skill development, managing learning paths, and handling certifications.

## Features

- **Skill Progress Tracking**: Visualize and update your skill progress
- **Learning Path Management**: Create and follow structured learning paths
- **Certification Display**: View and verify your certifications
- **Progress Visualization**: Charts and graphs for your development journey
- **gRPC Integration**: Connects to the skill_development backend service

## Architecture

This app is built with:
- **Yew**: Rust frontend framework for web assembly
- **Tauri**: Desktop application wrapper
- **gRPC**: Communication with the skill_development backend
- **Plotters**: Data visualization for progress charts

## Components

### UI Components
- `ProgressTracker`: Displays current skill progress
- `CertificationDisplay`: Shows earned certifications
- `LearningPathCreator`: Create new learning paths
- `SkillProgressTracker`: Update skill progress
- `LearningPathVisualizer`: Visualize learning paths
- `ProgressVisualizer`: Chart-based progress visualization
- `MainDashboard`: Main dashboard combining all components

### Services
- `GrpcClient`: gRPC client for communicating with backend services

## Development

### Prerequisites
- Rust and Cargo
- Node.js and npm (for Tauri)
- skill_development backend service running

### Setup

1. Start the skill_development gRPC server:
```bash
cd ../../shared_packages/skill_development
cargo run --bin grpc_server
```

2. Run the web application:
```bash
trunk serve
```

3. Or build the Tauri desktop app:
```bash
cargo tauri dev
```

### Building

#### Web Version
```bash
trunk build
```

#### Desktop Version
```bash
cargo tauri build
```

## Dependencies

- `yew`: Frontend framework
- `tonic`: gRPC client
- `plotters`: Data visualization
- `uuid`: UUID handling
- `serde`: Serialization
- `tauri`: Desktop application framework

## Environment Variables

- `DATABASE_URL`: Connection string for the database
- `SERVER_ADDRESS`: Address of the gRPC server (default: http://127.0.0.1:50051)

## Testing

```bash
wasm-pack test --headless --firefox
```

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

This project is licensed under the CPC Cooperative License.