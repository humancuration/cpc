# CRM Module Migration Guide

This document provides instructions for migrating the CRM module database schema.

## Migration Principles

1. All migrations must be additive and backward-compatible when possible
2. Destructive migrations require explicit user consent
3. Migrations are managed through the shared cpc-core migration system
4. Each migration file should contain both up and down scripts

## Migration Process

1. Create a new migration file in `packages/cpc-core/migrations/` with the naming convention:
   `YYYYMMDDHHMMSS_crm_description.sql`

2. Write the forward migration in the `-- +goose Up` section
3. Write the rollback migration in the `-- +goose Down` section

## Example Migration

```sql
-- +goose Up
-- SQL in section 'Up' is executed when this migration is applied

CREATE TABLE crm_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(50),
    company VARCHAR(255),
    address TEXT,
    notes TEXT,
    tags JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_crm_contacts_user_id ON crm_contacts(user_id);
CREATE INDEX idx_crm_contacts_email ON crm_contacts(email);

-- +goose Down
-- SQL section 'Down' is executed when this migration is rolled back

DROP TABLE crm_contacts;
```

## Data Migration Considerations

When migrating data:
1. Always backup data before running migrations
2. Test migrations on a copy of production data
3. Ensure migrations can be rolled back safely
4. Monitor migration performance on large datasets

## P2P Data Migration

For p2p data schema changes:
1. Update the schema definitions in `src/infrastructure/p2p/`
2. Implement schema versioning in data sharing protocols
3. Provide backward compatibility for older schema versions
4. Document breaking changes in release notes