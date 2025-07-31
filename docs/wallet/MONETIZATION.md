# Wallet Monetization Features

## Real-time Tip Notification Flow

```mermaid
sequenceDiagram
    participant Sender
    participant WalletService
    participant Recipient
    Sender->>WalletService: sendTip(recipientId, amount)
    WalletService->>WalletService: Process transaction
    WalletService->>Recipient: Broadcast TipReceived event
    Recipient-->>WalletService: Subscribe to tipReceived
```

## GraphQL Subscription
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