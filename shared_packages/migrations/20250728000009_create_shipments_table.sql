-- +goose Up
-- SQL in section 'Up' is executed when this migration is applied

-- Create shipments table
CREATE TABLE shipments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tracking_number VARCHAR(100) NOT NULL UNIQUE,
    carrier VARCHAR(255) NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('created', 'in_transit', 'delayed', 'delivered', 'cancelled')),
    origin_id UUID NOT NULL,
    destination_id UUID NOT NULL,
    expected_transit_days INTEGER NOT NULL,
    actual_transit_days INTEGER,
    estimated_delivery TIMESTAMP WITH TIME ZONE NOT NULL,
    actual_delivery TIMESTAMP WITH TIME ZONE,
    consent_settings JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create shipment line items table
CREATE TABLE shipment_line_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    shipment_id UUID NOT NULL REFERENCES shipments(id) ON DELETE CASCADE,
    inventory_item_id UUID NOT NULL REFERENCES inventory_items(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL,
    description TEXT
);

-- Create indexes for shipments
CREATE INDEX idx_shipments_tracking_number ON shipments(tracking_number);
CREATE INDEX idx_shipments_carrier ON shipments(carrier);
CREATE INDEX idx_shipments_status ON shipments(status);
CREATE INDEX idx_shipments_origin_id ON shipments(origin_id);
CREATE INDEX idx_shipments_destination_id ON shipments(destination_id);

-- Create indexes for shipment line items
CREATE INDEX idx_shipment_line_items_shipment_id ON shipment_line_items(shipment_id);
CREATE INDEX idx_shipment_line_items_inventory_item_id ON shipment_line_items(inventory_item_id);

-- +goose Down
-- SQL section 'Down' is executed when this migration is rolled back

DROP TABLE IF EXISTS shipment_line