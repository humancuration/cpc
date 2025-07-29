# Finance-Sheets Integration Design

## Overview

This document describes the integration between the Sheets application and the Finance domain in the CPC ecosystem. The integration enables users to leverage spreadsheet functionality for financial analysis, budgeting, and expense tracking while maintaining loose coupling between domains through event-driven architecture.

## Architecture

The integration follows hexagonal architecture principles with the following key components:

### Event-Driven Communication

All cross-domain communication between Sheets and Finance domains occurs through the event bus to maintain loose coupling:

```rust
// Finance events that can be published to the event bus
pub enum FinanceEvent {
    // Existing finance events...
    BudgetCreated(Budget),
    ExpenseAdded(Expense),
    // ... other events
    
    // New events for dashboard integration
    DashboardDataRequested { 
        user_id: Uuid,
        request_id: Uuid,
        source_sheet: Uuid, // SheetId
    },
    DashboardDataUpdated {
        request_id: Uuid,
        data: serde_json::Value, // DashboardData
    },
    // Reverse flow
    DashboardCellUpdated {
        sheet_id: Uuid, // SheetId
        cell_address: String, // CellAddress as string
        new_value: serde_json::Value, // CellValue
    },
}
```

### Financial Formula Functions

The Sheets application extends its formula evaluator with financial functions:

#### PMT (Payment)
Calculates the payment for a loan based on constant payments and a constant interest rate.

**Syntax:** `PMT(rate, nper, pv, [fv], [type])`

**Parameters:**
- `rate`: The interest rate for the loan
- `nper`: The total number of payments for the loan
- `pv`: The present value, or the total amount that a series of future payments is worth now
- `fv` (optional): The future value, or a cash balance you want to attain after the last payment is made
- `type` (optional): The number 0 or 1 and indicates when payments are due

#### FV (Future Value)
Calculates the future value of an investment based on periodic, constant payments and a constant interest rate.

**Syntax:** `FV(rate, nper, pmt, [pv], [type])`

**Parameters:**
- `rate`: The interest rate per period
- `nper`: The total number of payment periods
- `pmt`: The payment made each period
- `pv` (optional): The present value, or the lump-sum amount that a series of future payments is worth right now
- `type` (optional): The number 0 or 1 and indicates when payments are due

#### NPV (Net Present Value)
Calculates the net present value of an investment based on a discount rate and a series of future payments (negative values) and income (positive values).

**Syntax:** `NPV(rate, value1, [value2], ...)`

**Parameters:**
- `rate`: The rate of discount over the length of one period
- `value1, value2, ...`: Value1 is required, subsequent values are optional. 1 to 254 arguments representing the payments and income

#### IRR (Internal Rate of Return)
Returns the internal rate of return for a series of cash flows represented by the numbers in values.

**Syntax:** `IRR(values, [guess])`

**Parameters:**
- `values`: An array or a reference to cells that contain numbers for which you want to calculate the internal rate of return
- `guess` (optional): A number that you guess is close to the result of IRR

### Budget Template System

The budget template system allows users to apply predefined budget structures to sheets and automatically create corresponding finance domain objects.

#### Template Types
- Monthly Budget
- Weekly Budget
- Project Budget
- Custom

#### Template Processing Flow
1. User selects "Apply Budget Template" on a sheet
2. TemplateService identifies the template type based on sheet structure
3. TemplateService processes the sheet data and creates budget entries in the Finance domain
4. Finance domain publishes BudgetCreated events

### Expense Import System

The expense import system allows users to import expense data from sheets into the Finance domain.

#### Column Mapping
Users can map sheet columns to expense fields:
- Date column
- Amount column
- Category column
- Description column (optional)
- Vendor column (optional)
- Account column (optional)

#### Import Process
1. User selects "Import Expenses" and provides column mapping
2. ImportProcessor reads data from the sheet
3. ImportProcessor creates expense entries in the Finance domain
4. Finance domain publishes ExpenseAdded events

### BI Dashboard Integration

The BI dashboard integration enables bidirectional data flow between sheets and finance dashboards.

#### Data Flow
1. User selects "Publish to Dashboard" on a sheet
2. Sheets app publishes DashboardDataRequested event
3. Finance domain processes sheet data through FinanceAggregator
4. Finance domain publishes DashboardDataUpdated event
5. Dashboard updates with new data

#### Reverse Flow
1. User updates data in dashboard
2. Dashboard publishes DashboardCellUpdated event
3. Sheets app updates corresponding sheet cells

## API Specifications

### Expense Import Endpoint

```
POST /api/sheets/{sheet_id}/import/expenses

Request Body:
{
  "date_column": "A",
  "amount_column": "B",
  "category_column": "C",
  "description_column": "D",
  "vendor_column": "E",
  "account_column": "F"
}

Response:
{
  "total_rows": 100,
  "successful_imports": 95,
  "failed_rows": [
    {
      "row_number": 15,
      "error": "Invalid date format",
      "data": {
        "A": "invalid-date",
        "B": "50.00",
        "C": "Food"
      }
    }
  ],
  "errors": []
}
```

## Error Handling

### Financial Calculations
Financial functions provide precise error messages for common issues:
- Invalid argument counts
- Non-numeric values where numbers are expected
- Mathematical errors (e.g., division by zero)

### Import Failures
Import operations show detailed information about problematic rows:
- Row number
- Specific error message
- Data from the failed row for troubleshooting

### Template Application
Template application is transactional - if any budget creation fails, the entire operation is rolled back.

## Mobile Optimization

All features are designed with mobile users in mind:
- Responsive template layouts
- Touch-friendly import workflows
- Mobile-optimized dashboard exports

## Implementation Sequence

1. ✅ Financial formula evaluator extensions (PMT, FV, NPV, IRR)
2. ✅ Budget template service
3. ✅ Expense import system
4. ✅ BI dashboard integration

## Future Enhancements

- Additional financial functions (XNPV, XIRR, MIRR)
- Advanced template customization
- Real-time dashboard updates
- Enhanced mobile features