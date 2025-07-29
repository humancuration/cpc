# Financial Formula Reference

## Overview

This document provides detailed documentation for the financial functions available in the CPC Sheets application. These functions enable users to perform complex financial calculations directly within spreadsheets.

## PMT Function

Calculates the payment for a loan based on constant payments and a constant interest rate.

### Syntax
```
PMT(rate, nper, pv, [fv], [type])
```

### Parameters
- **rate** (required): The interest rate for the loan
- **nper** (required): The total number of payments for the loan
- **pv** (required): The present value, or the total amount that a series of future payments is worth now
- **fv** (optional): The future value, or a cash balance you want to attain after the last payment is made. If omitted, defaults to 0.
- **type** (optional): The number 0 or 1 and indicates when payments are due. If omitted, defaults to 0.
  - 0 = Payment at the end of the period
  - 1 = Payment at the beginning of the period

### Return Value
The payment amount for the loan.

### Examples
```
=PMT(0.05/12, 60, 10000)
// Calculates the monthly payment for a $10,000 loan with 5% annual interest rate over 5 years

=PMT(0.08, 10, 0, 10000, 1)
// Calculates the annual payment needed to save $10,000 in 10 years with 8% annual interest, payments at the beginning of each year
```

### Notes
- Cash paid out (such as deposits to savings) is represented by negative numbers.
- Cash received (such as dividend checks) is represented by positive numbers.

## FV Function

Calculates the future value of an investment based on periodic, constant payments and a constant interest rate.

### Syntax
```
FV(rate, nper, pmt, [pv], [type])
```

### Parameters
- **rate** (required): The interest rate per period
- **nper** (required): The total number of payment periods
- **pmt** (required): The payment made each period
- **pv** (optional): The present value, or the lump-sum amount that a series of future payments is worth right now. If omitted, defaults to 0.
- **type** (optional): The number 0 or 1 and indicates when payments are due. If omitted, defaults to 0.
  - 0 = Payment at the end of the period
  - 1 = Payment at the beginning of the period

### Return Value
The future value of the investment.

### Examples
```
=FV(0.06/12, 120, -200)
// Calculates the future value of saving $200 per month for 10 years at 6% annual interest

=FV(0.12, 30, -1000, -5000, 1)
// Calculates the future value of an investment with an initial deposit of $5,000, annual contributions of $1,000 for 30 years at 12% annual interest, with payments at the beginning of each year
```

### Notes
- Cash paid out (such as deposits to savings) is represented by negative numbers.
- Cash received (such as dividend checks) is represented by positive numbers.

## NPV Function

Calculates the net present value of an investment based on a discount rate and a series of future payments (negative values) and income (positive values).

### Syntax
```
NPV(rate, value1, [value2], ...)
```

### Parameters
- **rate** (required): The rate of discount over the length of one period
- **value1, value2, ...** (required): 1 to 254 arguments representing the payments and income
  - value1 is required, subsequent values are optional
  - Values must be equally spaced in time and occur at the end of each period

### Return Value
The net present value of the investment.

### Examples
```
=NPV(0.1, -10000, 3000, 4200, 6800)
// Calculates the net present value of an investment with a 10% discount rate and cash flows of -$10,000 (initial investment), $3,000, $4,200, and $6,800

=NPV(0.08, A2:A6)
// Calculates the net present value using cash flows in cells A2 through A6 with an 8% discount rate
```

### Notes
- NPV is similar to the PV function (present value) except that PV allows cash flows to begin either at the end or at the beginning of a period.
- NPV assumes that all cash flows occur at the end of each period.
- The order of values is important; be sure to enter your payment and income values in the correct sequence.

## IRR Function

Returns the internal rate of return for a series of cash flows represented by the numbers in values.

### Syntax
```
IRR(values, [guess])
```

### Parameters
- **values** (required): An array or a reference to cells that contain numbers for which you want to calculate the internal rate of return
  - Values must contain at least one positive value and one negative value to calculate the internal rate of return
  - IRR uses the order of values to interpret the order of cash flows
- **guess** (optional): A number that you guess is close to the result of IRR. If omitted, defaults to 0.1 (10%).

### Return Value
The internal rate of return for the investment.

### Examples
```
=IRR(A2:A7)
// Calculates the internal rate of return for cash flows in cells A2 through A7

=IRR(A2:A7, 0.1)
// Calculates the internal rate of return with a guess of 10% for cash flows in cells A2 through A7
```

### Notes
- IRR is calculated through an iterative process that can have zero or more solutions.
- If IRR cannot find a result after 20 iterations, it returns the #NUM! error value.
- In most cases, you don't need to provide a guess for the IRR calculation.
- IRR is closely related to NPV (net present value) function. The rate of return calculated by IRR is the interest rate corresponding to NPV = 0.

## Common Troubleshooting

### Error Messages
- **#NUM!**: The calculation cannot converge to a result. Try providing a different guess value.
- **#VALUE!**: One of the arguments is not a number. Check that all arguments are numeric.
- **#DIV/0!**: Division by zero occurred in the calculation. Check your rate and period values.

### Best Practices
1. **Consistent Time Periods**: Ensure that all rate and period values use the same time units (monthly, annual, etc.).
2. **Cash Flow Sign Convention**: Use negative numbers for cash paid out and positive numbers for cash received.
3. **Cell References**: Use cell references instead of hard-coded values for easier updates and better readability.
4. **Data Validation**: Validate your input data before performing financial calculations.

### Performance Tips
1. **Avoid Circular References**: Financial calculations can create circular references if not structured properly.
2. **Use Named Ranges**: For complex financial models, use named ranges to make formulas more readable.
3. **Limit Iteration**: Be cautious with functions that use iterative calculations (like IRR) as they can slow down recalculation.

## Related Functions
- **PV**: Calculates the present value of an investment
- **RATE**: Calculates the interest rate per period of an annuity
- **NPER**: Calculates the number of periods for an investment
- **IPMT**: Calculates the interest payment for an investment
- **PPMT**: Calculates the payment on the principal for an investment