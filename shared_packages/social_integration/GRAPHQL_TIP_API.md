# GraphQL Tip API Specification (DEPRECATED)

**NOTE**: This API specification is deprecated. The tip functionality has been moved to the wallet package.
Please refer to the wallet package's GraphQL API documentation for the current implementation.

## Schema Definition

```graphql
# Represents monetary value with currency
type Money {
  amount: Float!
  currency: Currency!
}

enum Currency {
  DAB # Dabloons (our primary currency)
  USD
  EUR
  GBP
  JPY
  # Add other supported currencies as needed
}

# Tip transaction type
type TipTransaction {
  id: ID!
  senderId: ID!
  recipientId: ID!
  amount: Money!
  note: String!
  createdAt: DateTime! # ISO 8601 format
}

# Paginated list of tip transactions
type TipTransactionConnection {
  edges: [TipTransactionEdge!]!
  pageInfo: PageInfo!
}

type TipTransactionEdge {
  node: TipTransaction!
  cursor: String!
}

type PageInfo {
  hasNextPage: Boolean!
  endCursor: String
}

# Root query type
type Query {
  tipTransactionsByUser(
    userId: ID!
    first: Int = 10
    after: String
  ): TipTransactionConnection!
}

# Root mutation type
type Mutation {
  sendTip(
    recipientId: ID!
    amount: MoneyInput!
    note: String
  ): TipTransaction!
}

# Input type for money
input MoneyInput {
  amount: Float!
  currency: Currency!
}

scalar DateTime
```

## Example Queries

### Fetch tip history
```graphql
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

### Send a tip
```graphql
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
```

## Authorization Requirements
- All tip operations require authentication
- Users can only query their own tip history (user ID from auth token must match query parameter)
- Users can only send tips from their own account (sender ID from auth token)

## Error Handling
- `401 Unauthorized` for unauthenticated requests
- `403 Forbidden` for attempts to access another user's data
- `400 Bad Request` for invalid inputs (negative amounts, etc.)
- `500 Internal Server Error` for unexpected failures

## Validation Rules
- Tip amount must be positive
- Sender and recipient must be valid user IDs
- Currency must be supported
- Note must be ≤ 200 characters