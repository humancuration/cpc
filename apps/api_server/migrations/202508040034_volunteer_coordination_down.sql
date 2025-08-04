-- Volunteer Coordination: schema teardown (DOWN)
-- Drops tables in reverse dependency order.

BEGIN;

-- Drop indexes explicitly only if needed; PostgreSQL drops indexes with their tables automatically.
-- We rely on cascade drop behavior for indexes.

DROP TABLE IF EXISTS volunteer_contributions;
DROP TABLE IF EXISTS volunteer_applications;
DROP TABLE IF EXISTS volunteer_opportunities;

COMMIT;