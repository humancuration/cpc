# Learning Platform Frontend

A decentralized learning platform built with Rust, Yew, and Tauri.

## Features

- Course catalog with search and filtering
- Course details with modules and lessons
- Enrollment tracking with progress visualization
- Academic credential management
- Educator tipping system
- Dark/light theme support
- Responsive design for all devices

## Tech Stack

- **Frontend**: Yew (Rust/WASM)
- **Desktop**: Tauri 2.0
- **Styling**: Stylist (CSS-in-Rust)
- **State Management**: Yew Context API
- **Networking**: gRPC (Tonic)
- **Charting**: Plotters

## Architecture

The application follows a hexagonal architecture with vertical slices:

```
src/
├── main.rs          # Tauri entry point
├── lib.rs           # Application root component
├── routes/          # Application routing
├── contexts/        # Global state contexts
├── services/        # gRPC service clients
├── pages/           # Page components
├── components/      # Reusable UI components
├── types/           # Shared type definitions
└── utils/           # Utility functions
```

## Getting Started

### Prerequisites

- Rust toolchain (latest stable)
- Node.js (for Tauri build tools)
- wasm-pack

### Development

1. Install dependencies:
   ```bash
   cargo build
   ```

2. Run the development server:
   ```bash
   cargo tauri dev
   ```

### Building

To build the application for production:

```bash
cargo tauri build
```

## Project Structure

### Pages

- `CourseCatalogPage` - Displays available courses
- `CourseDetailPage` - Shows course content
- `EnrollmentPage` - Tracks user progress
- `CredentialPage` - Manages academic credentials
- `TippingPage` - Supports educators

### Components

- `CourseCard` - Summary view of a course
- `ModuleAccordion` - Expandable course modules
- `VideoPlayer` - AV1/Opus media player
- `ProgressBar` - Visual progress indicator
- `CredentialBadge` - Display academic credentials
- `TipForm` - Interface for tipping educators

## gRPC Integration

The frontend communicates with the backend through gRPC services defined in the `learning_core` shared package.

## Theming

The application supports both light and dark themes through CSS variables and a theme context.

## License

This project is part of the CPC Cooperative ecosystem and uses the CPC License.