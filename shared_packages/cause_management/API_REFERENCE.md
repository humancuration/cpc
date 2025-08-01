# Cause Management Service API Reference

## Overview

This document provides a reference for all API endpoints available in the Cause Management Service, which extends the CPay gRPC service with cause management functionality.

## Service Definition

The Cause Management Service implements the `CpayService` defined in `cpay.proto`.

## RPC Methods

### CreateCause

Creates a new cause for donations.

**Request**: `CreateCauseRequest`
```protobuf
message CreateCauseRequest {
  string name = 1;
  string description = 2;
  optional string image_url = 3;
}
```

**Response**: `CreateCauseResponse`
```protobuf
message CreateCauseResponse {
  Cause cause = 1;
}
```

**Errors**:
- `INVALID_ARGUMENT`: If the request contains invalid data
- `INTERNAL`: If there's a database or server error

### GetCause

Retrieves a specific cause by ID.

**Request**: `GetCauseRequest`
```protobuf
message GetCauseRequest {
  string cause_id = 1;
}
```

**Response**: `GetCauseResponse`
```protobuf
message GetCauseResponse {
  Cause cause = 1;
}
```

**Errors**:
- `INVALID_ARGUMENT`: If the cause ID is invalid
- `NOT_FOUND`: If the cause doesn't exist
- `INTERNAL`: If there's a database or server error

### UpdateCause

Updates an existing cause.

**Request**: `UpdateCauseRequest`
```protobuf
message UpdateCauseRequest {
  string cause_id = 1;
  optional string name = 2;
  optional string description = 3;
  optional string image_url = 4;
}
```

**Response**: `UpdateCauseResponse`
```protobuf
message UpdateCauseResponse {
  Cause cause = 1;
}
```

**Errors**:
- `INVALID_ARGUMENT`: If the cause ID is invalid or request contains invalid data
- `NOT_FOUND`: If the cause doesn't exist
- `INTERNAL`: If there's a database or server error

### DeleteCause

Deletes a cause.

**Request**: `DeleteCauseRequest`
```protobuf
message DeleteCauseRequest {
  string cause_id = 1;
}
```

**Response**: `DeleteCauseResponse`
```protobuf
message DeleteCauseResponse {
  bool success = 1;
}
```

**Errors**:
- `INVALID_ARGUMENT`: If the cause ID is invalid
- `NOT_FOUND`: If the cause doesn't exist
- `INTERNAL`: If there's a database or server error

### ListCauses

Lists causes with pagination support.

**Request**: `ListCausesRequest`
```protobuf
message ListCausesRequest {
  optional int32 limit = 1;
  optional int32 offset = 2;
}
```

**Response**: `ListCausesResponse`
```protobuf
message ListCausesResponse {
  repeated Cause causes = 1;
  int32 total_count = 2;
}
```

**Errors**:
- `INVALID_ARGUMENT`: If limit or offset values are invalid
- `INTERNAL`: If there's a database or server error

### GetFeaturedCauses

Retrieves featured causes for promotion.

**Request**: `FeaturedCausesRequest`
```protobuf
message FeaturedCausesRequest {}
```

**Response**: `FeaturedCausesResponse`
```protobuf
message FeaturedCausesResponse {
  repeated Cause causes = 1;
}
```

**Errors**:
- `INTERNAL`: If there's a database or server error

## Message Types

### Cause

Represents a charitable cause.

```protobuf
message Cause {
  string id = 1;
  string name = 2;
  string description = 3;
  string image_url = 4;
  string total_donations = 5;
}
```

## Data Types

### UUID

All ID fields use UUID version 4 strings in the standard format:
```
xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
```

### Decimal

Monetary amounts and donation totals use string representation of decimal values:
```
"100.50"
"0.00"
"1000000.99"
```

## Error Codes

The service uses standard gRPC status codes:

- `OK (0)`: Success
- `CANCELLED (1)`: Request cancelled
- `UNKNOWN (2)`: Unknown error
- `INVALID_ARGUMENT (3)`: Invalid request data
- `NOT_FOUND (5)`: Resource not found
- `ALREADY_EXISTS (6)`: Resource already exists
- `PERMISSION_DENIED (7)`: Insufficient permissions
- `INTERNAL (13)`: Server error
- `UNAVAILABLE (14)`: Service unavailable

## Rate Limiting

The service may implement rate limiting to prevent abuse. Clients should handle `UNAVAILABLE` errors with appropriate backoff strategies.

## Authentication

The service expects authentication to be handled at the network level or by a gateway service. Individual RPC methods do not include authentication parameters.

## Versioning

The service uses semantic versioning. Breaking changes to the API will result in a new major version of the proto definitions.

## Examples

### Creating a Cause

```javascript
const request = {
  name: "Clean Water Initiative",
  description: "Help provide clean water to communities in need",
  image_url: "https://example.com/clean-water.jpg"
};

const response = await client.createCause(request);
console.log("Created cause:", response.cause);
```

### Listing Causes

```javascript
const request = {
  limit: 10,
  offset: 0
};

const response = await client.listCauses(request);
console.log("Total causes:", response.total_count);
console.log("Causes:", response.causes);