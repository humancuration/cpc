-- +goose Up
-- SQL in section 'Up' is executed when this migration is applied

-- Create suppliers table
CREATE TABLE suppliers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(50) NOT NULL,
    address TEXT NOT NULL,
    website VARCHAR(255),
    delivery_time_score DECIMAL(3,2) NOT NULL DEFAULT 0.00,
    quality_score DECIMAL(3,2) NOT NULL DEFAULT 0.00,
    responsiveness_score DECIMAL(3,2) NOT NULL DEFAULT 0.00,
    last_evaluation_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_critical BOOLEAN NOT NULL DEFAULT false,
    consent_settings JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for suppliers
CREATE INDEX idx_suppliers_name ON suppliers(name);
CREATE INDEX idx_suppliers_email ON suppliers(email);
CREATE INDEX idx_suppliers_critical ON suppliers(is_critical);

-- +goose Down
-- SQL section 'Down' is executed when this migration is rolled back

DROP TABLE IF EXISTS suppliers;