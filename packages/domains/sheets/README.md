# Sheets Application

A collaborative spreadsheet application for the CPC platform.

## Overview

The Sheets app provides a comprehensive spreadsheet tool with real-time collaboration, formula evaluation, chart generation, and format import/export capabilities. It integrates with the CPC ecosystem through:

- **BI Visualization Toolkit** for chart generation
- **Event Bus** for real-time collaboration
- **Storage Abstraction** for data persistence
- **CRDTs** for conflict-free collaborative editing

## Features

- Real-time collaborative editing with CRDTs
- Formula evaluation engine (SUM, AVERAGE, etc.)
- Chart generation using BI Visualization Toolkit
- XLSX and CSV import/export
- Permission management
- Version history

## Architecture

The app follows hexagonal architecture with vertical slices:

```
apps/sheets/
├── src/
│   ├── domain/          # Domain models and logic
│   ├── application/     # Application services
│   ├── infrastructure/  # Infrastructure implementations
│   └── presentation/    # Web interfaces
└── migrations/          # Database migrations
```

## Domain Models

- **Sheet**: Main spreadsheet container
- **Cell**: Individual cell with value and formatting
- **Formula**: Cell formulas with dependencies
- **ChartSpec**: Chart configuration
- **CellCrdt**: CRDT for collaborative editing

## Services

- **SheetService**: Sheet management
- **FormulaEvaluator**: Formula evaluation
- **ChartService**: Chart generation
- **CollaborationService**: Real-time collaboration

## Installation

Add to your Cargo.toml:

```toml
[dependencies]
cpc-sheets = { path = "../apps/sheets" }
```

## Usage

```rust
use cpc_sheets::application::SheetService;
use cpc_sheets::infrastructure::storage::SheetRepository;

// Create sheet service
let repository = SheetRepository::new();
let sheet_service = SheetService::new(repository);

// Create a new sheet
let sheet = sheet_service.create_sheet("My Sheet".to_string(), user_id)?;
```

## Testing

Run tests with:

```bash
cargo test
```

## License

This module is part of the CPC software ecosystem and is licensed under the CPC license.