-- +goose Up
-- SQL in section 'Up' is executed when this migration is applied

-- Create supplier contracts table
CREATE TABLE supplier_contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    supplier_id UUID NOT NULL REFERENCES suppliers(id) ON DELETE CASCADE,
    start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date TIMESTAMP WITH TIME ZONE NOT NULL,
    terms TEXT NOT NULL,
    renewal_required BOOLEAN NOT NULL DEFAULT false,
    special_certifications JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for supplier contracts
CREATE INDEX idx_supplier_contracts_supplier_id ON supplier_contracts(supplier_id);
CREATE INDEX idx_supplier_contracts_end_date ON supplier_contracts(end_date);
CREATE INDEX idx_supplier_contracts_renewal_required ON supplier_contracts(renewal_required);

-- +goose Down
-- SQL section 'Down' is executed when this migration is rolled back

DROP TABLE IF EXISTS supplier_contracts;