-- +goose Up
-- SQL in section 'Up' is executed when this migration is applied

-- Create warehouses table
CREATE TABLE warehouses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    latitude DECIMAL(10, 8) NOT NULL,
    longitude DECIMAL(11, 8) NOT NULL,
    capacity INTEGER NOT NULL,
    current_utilization INTEGER NOT NULL DEFAULT 0,
    open_time TIME NOT NULL,
    close_time TIME NOT NULL,
    contact_info TEXT,
    consent_settings JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for warehouses
CREATE INDEX idx_warehouses_name ON warehouses(name);
CREATE INDEX idx_warehouses_location ON warehouses(latitude, longitude);

-- +goose Down
-- SQL section 'Down' is executed when this migration is rolled back

DROP TABLE IF EXISTS warehouses;