-- +goose Up
-- SQL in section 'Up' is executed when this migration is applied

-- Create supply chain networks table
CREATE TABLE supply_chain_networks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    nodes JSONB,
    connections JSONB,
    consent_settings JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for supply chain networks
CREATE INDEX idx_supply_chain_networks_owner_id ON supply_chain_networks(owner_id);
CREATE INDEX idx_supply_chain_networks_name ON supply_chain_networks(name);

-- +goose Down
-- SQL section 'Down' is executed when this migration is rolled back

DROP TABLE IF EXISTS supply_chain_networks;