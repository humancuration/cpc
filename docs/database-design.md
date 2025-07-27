# Impact Weights Database Design

## Schema Definition

The `impact_weights` table (migration: `apps/backend/migrations/20250726_impact_weights_table.sql`) implements our core business constraint:

```sql
CREATE TABLE impact_weights (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    category VARCHAR(50) NOT NULL CHECK (category IN ('Community', 'Environment', 'Workers')),
    weight DECIMAL(3,2) NOT NULL CHECK (weight BETWEEN 0 AND 1),
    PRIMARY KEY (user_id, category)
);

-- Initialize default weights for existing users
INSERT INTO impact_weights (user_id, category, weight)
SELECT 
    id, 
    category, 
    weight
FROM 
    users
CROSS JOIN (VALUES 
    ('Community', 0.45),
    ('Environment', 0.30),
    ('Workers', 0.25)
) AS defaults(category, weight);
```

## Design Rationale

### 1. Constraint Strategy

#### Category Enforcement
```sql
CHECK (category IN ('Community', 'Environment', 'Workers'))
```
- **Why**: Business requirement specifies exactly three impact categories
- **Alternative Considered**: Separate categories table
- **Decision**: Fixed set of categories justifies ENUM-like constraint
- **Tradeoff**: Adding new categories requires migration (intentional)

#### Weight Validation
```sql
CHECK (weight BETWEEN 0 AND 1)
```
- **Why**: Mathematical requirement for distribution weights
- **Alternative Considered**: Application-level validation only
- **Decision**: Defense-in-depth approach with multiple validation layers
- **Tradeoff**: Prevents invalid data at earliest possible stage

#### Composite Primary Key
```sql
PRIMARY KEY (user_id, category)
```
- **Why**: Each user can have exactly one weight per category
- **Alternative Considered**: Surrogate ID with unique constraint
- **Decision**: Natural key better expresses business constraint
- **Tradeoff**: More efficient for our access patterns

### 2. Referential Integrity
```sql
user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
```
- **Why**: Orphaned weights provide no business value
- **Alternative Considered**: ON DELETE SET NULL (not applicable - user_id is NOT NULL)
- **Decision**: Automatic cleanup maintains data hygiene
- **Tradeoff**: Prevents historical analysis of deleted users (by design)

## Valid Data States

### ✅ Valid Examples
| user_id | category    | weight |
|---------|-------------|--------|
| a1b2c3  | Community   | 0.50   |
| a1b2c3  | Environment | 0.30   |
| a1b2c3  | Workers     | 0.20   |

**Explanation**: Sum = 1.0, all categories present, weights within bounds

| user_id | category    | weight |
|---------|-------------|--------|
| d4e5f6  | Community   | 0.00   |
| d4e5f6  | Environment | 1.00   |
| d4e5f6  | Workers     | 0.00   |

**Explanation**: Extreme but valid distribution (all weight on Environment)

### ❌ Invalid Examples
| user_id | category    | weight | Reason |
|---------|-------------|--------|--------|
| g7h8i9  | Community   | 1.50   | Weight > 1 (violates CHECK) |
| g7h8i9  | Environment | -0.20  | Weight < 0 (violates CHECK) |
| g7h8i9  | Sustainability | 0.50 | Invalid category (violates CHECK) |

| user_id | category    | weight |
|---------|-------------|--------|
| j0k1l2  | Community   | 0.40   |
| j0k1l2  | Environment | 0.40   |

**Reason**: Missing "Workers" category (application-level validation will fail due to sum < 1.0)

## Validation Layer Coordination

Our three validation layers work together:

1. **Database Constraints** (First line of defense)
   - Reject invalid categories/weights immediately
   - Prevents invalid data insertion at lowest level

2. **Domain Model Validation** (Business rules)
   ```rust
   fn validate_distribution(weights: &[ImpactDistribution]) -> Result<(), CalculationError> {
       let total: f64 = weights.iter().map(|w| w.weight).sum();
       if (total - 1.0).abs() > 1e-10 {
           return Err(CalculationError::InvalidDistribution);
       }
       // Also verifies all three categories exist
   }
   ```

3. **GraphQL Error Handling** (User experience)
   ```rust
   impl From<CalculationError> for Error {
       fn from(err: CalculationError) -> Self {
           match err {
               CalculationError::InvalidDistribution => 
                   Error::new("INVALID_DISTRIBUTION: Weights must sum to exactly 1.0"),
               // ...
           }
       }
   }
   ```

## Migration Rationale

The initial data insertion uses a CROSS JOIN to ensure:
- Every existing user gets all three categories
- Default weights follow our business rules (0.45, 0.30, 0.25)
- Atomic operation during migration

```sql
INSERT INTO impact_weights (user_id, category, weight)
SELECT 
    id, 
    category, 
    weight
FROM 
    users
CROSS JOIN (VALUES 
    ('Community', 0.45),
    ('Environment', 0.30),
    ('Workers', 0.25)
) AS defaults(category, weight);
```

**Why not application-level seeding?**
- Ensures data consistency during initial deployment
- Atomic operation (all users get weights or none do)
- Prevents race conditions during first application startup

## Performance Considerations

### Indexing Strategy
- Implicit index from PRIMARY KEY (user_id, category)
- Optimal for our primary access pattern: `WHERE user_id = $1`

### Query Efficiency
The domain layer query:
```rust
sqlx::query_as!(ImpactDistribution, r#"
    SELECT 
        category AS "category: String",
        weight AS "weight: f64"
    FROM impact_weights
    WHERE user_id = $1
"#, user_uuid)
```

- Uses clustered index (by primary key)
- Returns exactly 3 rows per user (fixed cardinality)
- Completes in O(1) time complexity

## Future-Proofing Considerations

### Potential Evolution Paths

1. **Historical Tracking**
   ```sql
   ALTER TABLE impact_weights ADD COLUMN effective_date TIMESTAMPTZ NOT NULL DEFAULT NOW();
   ALTER TABLE impact_weights DROP CONSTRAINT impact_weights_pkey;
   ALTER TABLE impact_weights ADD PRIMARY KEY (user_id, category, effective_date);
   ```
   *Use Case*: Track how impact distributions change over time

2. **Dynamic Categories**
   ```sql
   CREATE TABLE impact_categories (
       id SERIAL PRIMARY KEY,
       name VARCHAR(50) NOT NULL UNIQUE
   );
   
   ALTER TABLE impact_weights 
   DROP CONSTRAINT impact_weights_category_check,
   ADD COLUMN category_id INTEGER NOT NULL REFERENCES impact_categories(id);
   ```
   *Use Case*: Allow cooperative-defined impact categories

> **Pitfall Alert**: Avoid premature optimization for future requirements. Current design intentionally limits flexibility to enforce business constraints. Only evolve the schema when proven business needs emerge.