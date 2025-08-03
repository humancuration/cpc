# EthicalScanner

EthicalScanner is a CPC application that combines product health scoring with supply chain transparency. It allows users to scan product barcodes to get information about nutritional value, ingredient quality, and ethical sourcing.

## Features

- Barcode/QR code scanning using device camera
- Health/nutrition scoring algorithms
- Supply chain tracking and ethical scoring
- Alternative product recommendations
- GraphQL API for public access
- gRPC services for internal communication
- Integration with Consent Manager for data permissions

## Architecture

The app follows hexagonal architecture with vertical slices for each major feature:

1. **Scanner Module** - Barcode/QR recognition using device camera
2. **Health Engine** - Nutrition scoring algorithms
3. **Supply Chain** - Ethical scoring and tracking
4. **Suggestions** - Alternative recommendations
5. **API Layer** - GraphQL and gRPC interfaces

## Database Schema

The application uses PostgreSQL for persistent storage with the following tables:

- `products` - Product information
- `ingredients` - Product ingredients
- `nutritional_facts` - Nutritional information
- `suppliers` - Supplier information
- `supply_chain_nodes` - Supply chain tracking
- `alternative_suggestions` - Product recommendations

## Getting Started

### Prerequisites

- Rust 1.60 or higher
- PostgreSQL 17.5
- Tauri 2.0 development environment

### Building

```bash
cd apps/ethical_scanner
cargo build
```

### Running

```bash
cd apps/ethical_scanner
cargo run
```

## API Documentation

### GraphQL Endpoints

- `product(barcode: String)` - Get product information by barcode
- `searchProducts(name: String)` - Search for products by name
- `supplyChain(productId: String)` - Get supply chain information for a product
- `alternatives(productId: String)` - Get alternative product suggestions

### gRPC Services

- `Scan` - Process product scanning
- `GetAlternatives` - Get alternative product suggestions
- `GetSupplyChain` - Get supply chain information

## Contributing

This is a CPC (Cooperative Public License) project. All contributions are welcome under the terms of the CPC license.

## License

This project is licensed under the CPC License - see the LICENSE file for details.