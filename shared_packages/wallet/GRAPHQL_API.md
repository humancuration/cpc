## Tip Notification Subscription

```graphql
subscription TipReceived($recipientId: ID!) {
  tipReceived(recipientId: $recipientId) {
    transactionId
    senderId
    amount {
      amount
      currency
    }
    timestamp
    note
  }
}
```

**Authorization**: 
- Requires authenticated user
- Recipient ID must match current user ID