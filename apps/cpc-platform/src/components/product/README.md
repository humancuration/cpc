# Product Display Component

This directory contains the implementation of the Product Display Component for the CPC Platform.

## Overview

The Product Display Component provides a comprehensive view of product information including:
- Product header with name and description
- Detailed product information
- Cost breakdown visualization using plotters-rs
- Supply chain visualization
- Validation status with real-time updates

## Components

1. **ProductDisplay** - Main component that orchestrates all subcomponents
2. **ProductHeader** - Displays product name and description
3. **ProductDetails** - Shows detailed product information
4. **CostBreakdown** - Visualizes cost distribution using plotters-rs
5. **SupplyChainDisplay** - Shows supply chain information
6. **ValidationStatus** - Displays validation status and updates

## Implementation Details

### Frontend (Yew/Rust)
- Built using Yew functional components
- Uses `plotters-rs` with a `CanvasBackend` for robust data visualization.
- Implements real-time updates via event-driven GraphQL subscriptions.
- Responsive design for all device sizes.

### Backend Integration
- GraphQL queries for product data
- GraphQL subscriptions for real-time validation updates
- Direct Tauri command integration with `cpc-core` services for optimal performance on desktop.
- A dedicated service layer (`ProductDisplayService`) in `cpc-core` encapsulates business logic.

## Data Flow

```
GraphQL Query/Subscription → Service Layer → Component Props → Yew Components
```

## Features

- **Real-time Validation Updates**: Subscribes to product validation status changes
- **Interactive Visualizations**: Cost breakdown pie chart using plotters-rs
- **Responsive Design**: Works on desktop and mobile devices
- **Error Handling**: Graceful error display with retry options
- **Loading States**: Smooth loading indicators during data fetch

## Routes

- `/products2` - Product catalog listing
- `/products2/[id]` - Individual product display

## Future Enhancements

- Supply chain visualization enhancements.
- Additional product metadata display.