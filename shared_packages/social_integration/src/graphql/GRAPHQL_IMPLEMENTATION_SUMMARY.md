# GraphQL API Implementation Summary (DEPRECATED)

**NOTE**: This document describes the deprecated GraphQL API implementation for tipping functionality. The tip functionality has been moved to the wallet package.
Please refer to the wallet package documentation for the current implementation.

## Overview

A GraphQL API was implemented to provide access to tipping functionality, allowing users to send tips and query their tip transaction history. (DEPRECATED - moved to wallet package)

## Key Components Implemented

### 1. GraphQL Schema (DEPRECATED - moved to wallet package)

- Defined GraphQL types for Money, Currency, TipTransaction, and related types
- Implemented QueryRoot with `tipTransactionsByUser` query
- Implemented MutationRoot with `sendTip` mutation
- Created a schema factory function `create_schema`

### 2. GraphQL Types (DEPRECATED - moved to wallet package)

- `Money`: Represents monetary value with amount and currency
- `Currency`: Enum of supported currencies (DAB, USD, EUR, GBP, JPY)
- `TipTransaction`: Represents a tip transaction with sender, recipient, amount, note, and timestamp
- `TipTransactionConnection`: Paginated list of tip transactions
- `TipTransactionEdge`: Edge containing a tip transaction and cursor
- `PageInfo`: Information about pagination
- `MoneyInput`: Input type for money values

### 3. GraphQL Queries (DEPRECATED - moved to wallet package)

- `tipTransactionsByUser`: Query tip transactions for a user with pagination
  - Requires authentication
  - Users can only query their own tip history
  - Supports pagination with `first` and `after` parameters

### 4. GraphQL Mutations (DEPRECATED - moved to wallet package)

- `sendTip`: Send a tip from the authenticated user to another user
  - Requires authentication
  - Users can only send tips from their own account
  - Validates tip amount is positive
  - Validates note length is ≤ 200 characters
  - Validates currency is supported

### 5. Error Handling (DEPRECATED - moved to wallet package)

- Custom GraphQL error types for different error scenarios:
  - Unauthorized
  - Forbidden
  - InvalidInput
  - InternalServerError
  - NotImplemented

### 6. Authorization (DEPRECATED - moved to wallet package)

- Middleware to ensure users can only access their own data
- Users can only query their own tip history
- Users can only send tips from their own account

### 7. Input Validation (DEPRECATED - moved to wallet package)

- Tip amount must be positive
- Currency must be supported
- Note length must be ≤ 200 characters

## Integration with Existing Services (DEPRECATED - moved to wallet package)

- Uses `TipService` for business logic
- Uses `WalletService` for fund transfers
- Works with existing `TipTransactionRepository` for data access

## Testing (DEPRECATED - moved to wallet package)

- Created integration tests for GraphQL queries and mutations
- Tests cover successful operations and error cases
- Tests use mock services for isolation

## Usage (DEPRECATED - moved to wallet package)

The GraphQL API can be used to send tips and query tip transaction history:

```graphql
# Send a tip
mutation SendTip($recipientId: ID!, $amount: MoneyInput!, $note: String) {
  sendTip(recipientId: $recipientId, amount: $amount, note: $note) {
    id
    senderId
    recipientId
    amount {
      amount
      currency
    }
    note
    createdAt
  }
}

# Query tip history
query GetTipHistory($userId: ID!, $first: Int, $after: String) {
  tipTransactionsByUser(userId: $userId, first: $first, after: $after) {
    edges {
      node {
        id
        senderId
        recipientId
        amount {
          amount
          currency
        }
        note
        createdAt
      }
      cursor
    }
    pageInfo {
      hasNextPage
      endCursor
    }
  }
}
```

## Benefits (DEPRECATED - moved to wallet package)

1. **Standardized API**: GraphQL provides a standardized way to access tipping functionality
2. **Flexible Queries**: Clients can request exactly the data they need
3. **Real-time Updates**: GraphQL subscriptions can be added for real-time updates
4. **Type Safety**: Strong typing ensures data consistency
5. **Authorization**: Built-in authorization ensures data security
6. **Validation**: Input validation prevents invalid operations