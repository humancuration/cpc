# Currency Internationalization UI Components

This document describes the UI components implemented for currency internationalization in the Finance-Sheets application.

## Component Overview

The currency internationalization feature consists of several reusable Yew components that provide a comprehensive currency management experience:

1. **CurrencySelector** - A searchable dropdown for selecting currencies
2. **CurrencyConverter** - Real-time currency conversion tool
3. **FormattingPreview** - Visual demonstration of currency formatting rules
4. **ExchangeRateManager** - Interface for managing exchange rates (admin)
5. **SearchDropdown** - Shared searchable dropdown component

## Component Usage Guidelines

### CurrencySelector Component

The `CurrencySelector` component provides a searchable dropdown for selecting currencies from a list of all supported currencies.

**Usage:**
```rust
use crate::components::currency::CurrencySelector;

<CurrencySelector 
    on_select={on_currency_select}
    selected={selected_currency}
    aria_label="Select currency"
/>
```

**Props:**
- `on_select: Callback<Currency>` - Callback when a currency is selected
- `selected: Option<Currency>` - Currently selected currency
- `aria_label: String` - ARIA label for accessibility

### CurrencyConverter Component

The `CurrencyConverter` component provides a real-time currency conversion tool.

**Usage:**
```rust
use crate::components::currency::CurrencyConverter;

<CurrencyConverter />
```

**Features:**
- Two `CurrencySelector` components for source and target currencies
- Amount input field with numeric validation
- Real-time conversion display
- Last updated timestamp
- Rate source provider information
- Responsive design

### FormattingPreview Component

The `FormattingPreview` component demonstrates how currency values are formatted based on locale and user preferences.

**Usage:**
```rust
use crate::components::currency::FormattingPreview;

<FormattingPreview />
```

**Features:**
- Shows formatted examples for common values (1, 100, 1000, 1000000)
- Toggles between symbol/code display based on user preference
- Displays locale-specific formatting rules
- Interactive locale selector to preview different regions

### ExchangeRateManager Component

The `ExchangeRateManager` component provides an interface for managing currency exchange rates.

**Usage:**
```rust
use crate::components::currency::ExchangeRateManager;

<ExchangeRateManager />
```

**Features:**
- Table view of all currency pairs with current rates
- Filtering by provider, currency pair, last updated
- Manual rate override capability
- Rate refresh button with visual loading state
- Provider status indicators
- Audit trail of rate changes

## Accessibility Compliance

All components follow accessibility best practices:

1. **Keyboard Navigation**
   - All interactive elements support keyboard navigation
   - Arrow keys for dropdown navigation
   - Enter key to select items
   - Escape key to close dropdowns

2. **ARIA Labels**
   - Proper ARIA attributes for screen readers
   - `aria-label` for all interactive elements
   - `aria-live` for dynamic content updates
   - `aria-selected` for selected items

3. **Visual Design**
   - Sufficient color contrast (minimum 4.5:1)
   - Focus indicators for all interactive elements
   - Semantic HTML structure
   - Responsive design for different screen sizes

## Internationalization Considerations

The components support internationalization through:

1. **Locale-aware Formatting**
   - Decimal separators based on locale
   - Thousand separators based on locale
   - Symbol positioning based on locale
   - Negative number formatting based on locale

2. **Currency Support**
   - All ISO 4217 currencies
   - Custom Dabloons currency
   - Proper symbol and code display
   - Decimal place handling per currency

## Error Handling Patterns

Components implement consistent error handling:

1. **User-friendly Messages**
   - Clear error messages for conversion failures
   - Validation feedback for invalid inputs
   - Network error handling with retry options

2. **Graceful Degradation**
   - Fallback to cached rates when network unavailable
   - Default values when data is missing
   - Loading states during data fetch

## Performance Characteristics

The components are optimized for performance:

1. **Data Loading**
   - Caching strategy for currency lists (expires after 24h)
   - Debounced search inputs (300ms delay)
   - Virtualized long currency lists

2. **Rendering**
   - Efficient component updates with Yew's virtual DOM
   - Memoization of expensive formatting calculations
   - Lazy loading of non-critical data

## Integration with Backend Services

The UI components communicate with backend services through the `CurrencyApiService`:

1. **API Endpoints**
   - `GET /api/currencies` - Get all supported currencies
   - `POST /api/currencies/convert` - Convert currency amounts
   - `GET /api/exchange-rates` - Get exchange rates
   - `POST /api/exchange-rates/override` - Override exchange rates

2. **Data Models**
   - Currency model with code, name, symbol, decimal places
   - Exchange rate model with from/to currencies, rate, provider
   - User preferences for currency display

## Testing Strategy

Components are tested at multiple levels:

1. **Unit Tests**
   - Currency formatting logic
   - Conversion calculations
   - Search filtering

2. **Component Tests**
   - Keyboard navigation
   - ARIA compliance
   - Responsive behavior

3. **Integration Tests**
   - API communication
   - State synchronization
   - Error handling

## Future Enhancements

Planned improvements include:

1. **Enhanced Formatting Preview**
   - More comprehensive formatting examples
   - Custom formatting rule editor
   - Live preview of formatting changes

2. **Advanced Exchange Rate Management**
   - Historical rate charts
   - Rate alert notifications
   - Automated rate updates

3. **Improved Accessibility**
   - Enhanced screen reader support
   - Keyboard shortcut customization
   - High contrast mode

4. **Performance Optimizations**
   - Web Workers for heavy calculations
   - Progressive loading of currency data
   - Enhanced caching strategies