# DateTime Serialization Standard

## Format
We use the ISO 8601 format with milliseconds and 'Z' suffix for UTC:  
`yyyy-MM-dd'T'HH:mm:ss.SSSX`

Examples:
- `2025-07-22T01:42:45.082Z`

## Compatibility

### Rust (using chrono crate)
- Use `DateTime<Utc>` for all timestamps
- Serialization: `dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()`
- Deserialization: `DateTime::parse_from_rfc3339`

### Kotlin (using kotlinx-datetime)
- Use `Instant` for all timestamps
- Serialization: `instant.toString()`
- Deserialization: `Instant.parse()`

### Swift
- Use `ISO8601DateFormatter` with `.withFractionalSeconds` option
- Format: `yyyy-MM-dd'T'HH:mm:ss.SSSSSSZZZZZ`

### JavaScript
- Use `Date.toISOString()` which outputs the required format

## Usage Guidelines
1. All timestamps in API requests/responses must use this format
2. Database storage should use UTC timestamps in this format
3. Client UIs should convert to local timezone for display only