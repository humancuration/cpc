# Finance-Sheets

Finance-Sheets is a web application that extends the base Sheets functionality with financial features, including comprehensive currency internationalization support.

## Features

- **Currency Selector**: Searchable dropdown for selecting from all supported currencies
- **Currency Converter**: Real-time currency conversion between any supported currencies
- **Formatting Preview**: Visual demonstration of how currency values are formatted based on locale
- **Exchange Rate Manager**: Interface for managing currency exchange rates (admin)
- **Mobile Optimization**: Responsive design with touch-optimized interface
- **Offline Support**: Local storage with sync capabilities
- **Performance Optimization**: Virtual scrolling and memory management

## Currency Internationalization

The application supports:

- All ISO 4217 currencies
- Custom Dabloons currency
- Locale-aware formatting (decimal separators, thousand separators, symbol positioning)
- Real-time currency conversion
- Exchange rate management

## Mobile Optimization

Finance-Sheets includes comprehensive mobile optimization features:

- Responsive design for all device sizes (Mobile, Tablet, Desktop)
- Touch-optimized interface with minimum 48px targets
- Gesture support (swipe, tap, tap-and-hold)
- Virtual scrolling for large datasets
- Offline storage with sync capabilities
- Performance monitoring and optimization

## Architecture

The application follows the CPC architectural principles:

- **Hexagonal Architecture**: Clean separation of domain logic from presentation and infrastructure
- **Vertical Slices**: Each feature is self-contained with all necessary components
- **Yew Components**: UI built with Yew functional components and stylist for styling
- **Accessibility**: Keyboard navigation, ARIA labels, and sufficient color contrast

## Components

### CurrencySelector
A reusable searchable currency dropdown that allows users to select a currency from a list of all supported currencies.

### CurrencyConverter
Real-time currency conversion tool with two currency selectors and an amount input field.

### FormattingPreview
Visual demonstration of how currency values are formatted based on locale and user preferences.

### ExchangeRateManager
Interface for managing currency exchange rates with filtering, manual overrides, and audit trails.

## Development

### Prerequisites

- Rust toolchain
- wasm-pack
- Python (for serving the application)

### Building

```bash
cd apps/finance-sheets
./build.sh
```

### Serving

```bash
python -m http.server 8000 --directory pkg
```

Then open http://localhost:8000 in your browser.

## Documentation

- [Currency UI Documentation](../../docs/features/finance/currency_ui.md) for detailed information about the currency components.
- [Mobile Optimization Progress](MOBILE_OPTIMIZATION_PROGRESS.md) for implementation status.
- [Mobile Implementation Summary](MOBILE_IMPLEMENTATION_SUMMARY.md) for technical details.