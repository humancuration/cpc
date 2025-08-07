# Shtairir Types and Signatures

This document defines the Shtairir type system used by the shared IR and enforced by hosts. It covers scalar, composite, interop types, basic coercions, generics, and type hashing/versioning to inform compatibility.

Design Goals
- Strong, explicit typing for predictable composition and validation.
- Practical set of scalars and composites to cover most app needs.
- Deterministic serialization and hashing of types for caching and compatibility checks.
- Simple generics with constraints to enable reusable blocks (e.g., math operations, collections).
- Interop-friendly primitives for Rust and WASM execution.

Type Kinds
1) Scalar
- Bool
- Int (64-bit signed)
- UInt (64-bit unsigned)
- Decimal (arbitrary precision; host uses rust_decimal)
- Float (64-bit IEEE; discouraged in deterministic contexts)
- String (UTF-8)
- Bytes (opaque byte array)
- Char (Unicode scalar)
- Timestamp (UTC instant; RFC 3339)
- Duration (nanoseconds precision; normalized)
- Url
- Uuid

2) Composite
- Option[T]
- Result[Ok, Err]
- List[T]
- Map[K, V] (with K constrained to Hashable)
- Tuple[T1, T2, ... Tn] (n ≤ 8 recommended)
- Struct { field: Type, ... } (named fields, stable order)
- Enum { Variant(Type?), ... } (sum types with optional payload per variant)

3) Interop
- Json (lossy, flexible; use sparingly—prefer Struct/Enum)
- Any (escape hatch for dynamic wiring; strongly discouraged for stable modules)
- Foreign[T] (marker for host-provided handles; must not cross WASM boundary unless wrapped)

Notes
- Prefer Decimal for financial or precise math; Float is allowed for graphics/signal processing where determinism is not required.
- Timestamp is always UTC; time zone-sensitive representations are layered on top via Params or Struct fields.

Generics and Constraints
- Type variables: T, K, V
- Constraints:
  - Hashable: valid Map key; includes Bool, Int, UInt, Decimal (canonical), String, Uuid
  - Ordered: supports ordering; includes Int, UInt, Decimal, String, Timestamp, Duration
  - Serializable: all built-in types; Foreign[T] requires adapter
- Example:
  - List.map: (List[T], fn: (T) -> U) -> List[U]
  - Map.keys: Map[K: Hashable, V] -> List[K]

Signatures
A Block signature references types for inputs, outputs, and params:
- Input: name: Type
- Output: name: Type
- Param: name: Type = default?
- Variadic inputs: name: List[Type] or multiplicity: variadic
- Optionals: Option[T] or optional: true on the port (lowered to Option in IR)
- Errors: Result[OkType, ErrorType] for textual; visual maps error output pin to Err branch

Literals and Defaults
- Defaults must be valid literals of the declared type:
  - Bool: true/false
  - Int/UInt: decimal representation
  - Decimal: quoted decimal string or canonical decimal
  - String: quoted UTF-8
  - Bytes: base64
  - Timestamp: RFC3339 string
  - Duration: ISO 8601 duration or numeric nanoseconds (canonical form defined below)
  - Url/Uuid: canonical string format
  - Option: null for None or value for Some
  - List/Map/Tuple/Struct/Enum: JSON-like notation with explicit field names for Struct/Enum

Coercions (Basic)
Implicit (safe, no precision loss):
- UInt -> Int (when representable)
- Int/UInt -> Decimal
- String -> Url/Uuid/Timestamp (when parsing succeeds; otherwise error)
- Bytes -> String (UTF-8) only when valid; otherwise error
- T -> Option[T] (wrap as Some)
- OkType -> Result[OkType, E] (wrap as Ok)

Explicit (require cast block or parameter):
- Float <-> Decimal
- Bytes <-> String (non-UTF-8)
- Json <-> Struct/Enum (via schema-aware adapter)
- Any <-> T (unsafe; discouraged)

Canonicalization Rules (for Hashing/Equality)
- Struct/Enum field order: lexicographically sorted by field/variant name for hashing and IR serialization.
- Decimal: canonical string (no trailing zeros unless zero; explicit sign only if negative).
- Duration: stored as signed 128-bit nanoseconds; serialized as ISO 8601 duration string; canonical hash uses integer nanos.
- Timestamp: RFC3339 with Z suffix, millisecond precision unless higher is necessary.
- Map: key set serialized in sorted key order by canonical representation.
- Tuple: fixed order as declared.

Type Identifiers and Hashing
- Every type has a stable textual form (TypeSpec) and a TypeID hash.
- TypeSpec examples:
  - Int
  - List[String]
  - Map[String, Decimal]
  - Struct{amount:Decimal,currency:String}
  - Result[Struct{status:UInt,body:Bytes}, Enum{Net{code:UInt},Timeout,Denied}]
- TypeID:
  - type_id = blake3(utf8(TypeSpec)) truncated to 128 bits (16 bytes)
  - Rendered as base32 or hex for debugging
- Composite TypeID stability:
  - Depends only on component TypeSpecs in canonical form.
  - Adding a field to a Struct changes TypeID (breaking).
  - Adding a new Enum variant changes TypeID (breaking).
  - Changing field order does not affect TypeID due to canonicalization.

Type Versioning and Compatibility
- Types do not carry SemVer; they are structural. Compatibility is evaluated via:
  - Exact TypeID match = compatible (identical).
  - Backward-compatible read:
    - Struct: consumer may accept a superset if it declares optional fields with defaults; otherwise exact match required.
    - Enum: consumer must accept at least all variants produced by the producer; extra variants from producer are breaking.
  - Forward-compatible read:
    - Struct: producer may add fields only if the consumer ignores unknown fields; default host policy is strict unless annotated.
    - Map/List/Tuple: require element-wise compatibility.
- Blocks/Modules use SemVer (see 04-versioning) to communicate API changes; type hashing is a concrete tool for detection.

Error Types
- Prefer structured error types:
  - Enum{ Timeout, Denied, Net{code:UInt}, Parse{message:String}, Other{domain:String,code:String} }
- Use Result[T, E] in textual IR; visual IR maps error pins to E.

Determinism and Types
- Float and time-derived values mark flows as non-deterministic unless constrained by determinism flags at the block level.
- Randomness-dependent blocks should use explicit Seed: Bytes param to enable reproducible runs where possible.

Interoperability
- Rust mapping:
  - Int -> i64, UInt -> u64, Decimal -> rust_decimal::Decimal, Timestamp -> chrono::DateTime<Utc>
  - Bytes -> Vec<u8>, String -> String, Url -> url::Url, Uuid -> uuid::Uuid
  - Struct/Enum -> serde-friendly types
- WASM:
  - Types serialize to CBOR/JSON with canonical rules; binary paths prefer CBOR for performance.

Examples
1) Signature with generics
- block: list.map
- inputs: xs: List[T], f: (T -> U)
- outputs: ys: List[U]
- constraints: Serializable(T), Serializable(U)

2) HTTP response type
- Struct{status:UInt, headers:Map[String,String], body:Bytes}

3) Event payload with Schema
- Enum{ Text{value:String}, Json{value:Json}, Binary{value:Bytes} }

Choosing between option<T>, list<T>, and map<K,V>
- option<T> default: JSON null
- list<T> default: JSON []
- map<K,V> default: JSON {}
See example TOMLs in apps/shtairir_examples:
- string.concat (option<string>)
- math.sum_list (list<i64>)
- util.join_kv_pairs (map<string,string>)

Validation
- IR validator ensures all referenced TypeIDs resolve in the registry or local module scope.
- Coercions applied only when explicitly allowed. Implicit coercions limited to safe set above.