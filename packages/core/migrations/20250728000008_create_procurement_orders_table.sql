-- +goose Up
-- SQL in section 'Up' is executed when this migration is applied

-- Create procurement orders table
CREATE TABLE procurement_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    supplier_id UUID NOT NULL REFERENCES suppliers(id) ON DELETE CASCADE,
    order_number VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('draft', 'submitted', 'approved', 'rejected', 'shipped', 'received', 'cancelled')),
    expected_delivery TIMESTAMP WITH TIME ZONE NOT NULL,
    actual_delivery TIMESTAMP WITH TIME ZONE,
    total_amount DECIMAL(15,2) NOT NULL,
    total_amount_currency VARCHAR(3) NOT NULL,
    consent_settings JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create procurement order line items table
CREATE TABLE procurement_order_line_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES procurement_orders(id) ON DELETE CASCADE,
    inventory_item_id UUID NOT NULL REFERENCES inventory_items(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL,
    unit_price_amount DECIMAL(15,2) NOT NULL,
    unit_price_currency VARCHAR(3) NOT NULL,
    extended_price_amount DECIMAL(15,2) NOT NULL,
    extended_price_currency VARCHAR(3) NOT NULL,
    description TEXT
);

-- Create indexes for procurement orders
CREATE INDEX idx_procurement_orders_supplier_id ON procurement_orders(supplier_id);
CREATE INDEX idx_procurement_orders_status ON procurement_orders(status);
CREATE INDEX idx_procurement_orders_order_number ON procurement_orders(order_number);

-- Create indexes for procurement order line items
CREATE INDEX idx_procurement_order_line_items_order_id ON procurement_order_line_items(order_id);
CREATE INDEX idx_procurement_order_line_items_inventory_item_id ON procurement_order_line_items(inventory_item_id);

-- +goose Down
-- SQL section 'Down' is executed when this migration is rolled back

DROP TABLE IF EXISTS procurement_order_line_items;
DROP TABLE IF EXISTS procurement_orders;