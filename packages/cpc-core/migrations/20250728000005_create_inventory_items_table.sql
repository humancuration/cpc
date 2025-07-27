-- +goose Up
-- SQL in section 'Up' is executed when this migration is applied

-- Create inventory items table
CREATE TABLE inventory_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sku VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(50) NOT NULL CHECK (category IN ('raw_materials', 'work_in_process', 'finished_goods', 'packaging', 'supplies', 'equipment')),
    unit_of_measure VARCHAR(50) NOT NULL,
    safety_stock_level INTEGER NOT NULL DEFAULT 0,
    reorder_point INTEGER NOT NULL DEFAULT 0,
    current_quantity INTEGER NOT NULL DEFAULT 0,
    warehouse_id UUID NOT NULL REFERENCES warehouses(id) ON DELETE CASCADE,
    unit_cost_amount DECIMAL(15,2),
    unit_cost_currency VARCHAR(3),
    consent_settings JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for inventory items
CREATE INDEX idx_inventory_items_warehouse_id ON inventory_items(warehouse_id);
CREATE INDEX idx_inventory_items_sku ON inventory_items(sku);
CREATE INDEX idx_inventory_items_category ON inventory_items(category);
CREATE INDEX idx_inventory_items_reorder_point ON inventory_items(reorder_point);

-- +goose Down
-- SQL section 'Down' is executed when this migration is rolled back

DROP TABLE IF EXISTS inventory_items;