CREATE TABLE invoices (
    id UUID PRIMARY KEY,
    issuer_id UUID NOT NULL REFERENCES users(id),
    recipient_id UUID NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('DRAFT','ISSUED','PARTIALLY_PAID','PAID','OVERDUE','VOIDED')),
    issue_date TIMESTAMPTZ NOT NULL,
    due_date TIMESTAMPTZ NOT NULL,
    payment_date TIMESTAMPTZ,
    payment_method VARCHAR(20),
    metadata JSONB,
    template_id UUID,
    total_amount NUMERIC(15,2) GENERATED ALWAYS AS (
        (SELECT SUM(quantity * unit_price) FROM invoice_items WHERE invoice_id = id)
    ) STORED
);

CREATE TABLE invoice_items (
    id UUID PRIMARY KEY,
    invoice_id UUID NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity NUMERIC(15,4) NOT NULL,
    unit_price NUMERIC(15,4) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    tax_rate VARCHAR(10) NOT NULL
);