-- Create expenses table for expense tracking functionality
CREATE TABLE expenses (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    amount NUMERIC(20, 10) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    dabloons_amount NUMERIC(20, 10) DEFAULT 0.0,
    category VARCHAR(50) NOT NULL,
    custom_category VARCHAR(50),
    date TIMESTAMP WITH TIME ZONE NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Processed',
    receipt_id UUID,
    is_recurring BOOLEAN NOT NULL DEFAULT false,
    recurrence_pattern VARCHAR(100),
    linked_budget_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (receipt_id) REFERENCES receipts(id) ON DELETE SET NULL,
    FOREIGN KEY (linked_budget_id) REFERENCES budgets(id) ON DELETE SET NULL
);

-- Create receipts table for storing receipt images and OCR data
CREATE TABLE receipts (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    image_data BYTEA NOT NULL,
    image_format VARCHAR(10) NOT NULL,
    extracted_text TEXT NOT NULL DEFAULT '',
    merchant_name VARCHAR(100),
    transaction_date TIMESTAMP WITH TIME ZONE,
    total_amount NUMERIC(20, 10),
    currency VARCHAR(10),
    dabloons_amount NUMERIC(20, 10) DEFAULT 0.0,
    processing_status VARCHAR(20) NOT NULL DEFAULT 'Uploaded',
    processing_error TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create expense_sharing_preferences table for granular consent controls
CREATE TABLE expense_sharing_preferences (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE,
    sharing_enabled BOOLEAN NOT NULL DEFAULT false,
    anonymized BOOLEAN NOT NULL DEFAULT false,
    shared_categories JSONB NOT NULL DEFAULT '[]'::jsonb,
    time_limits JSONB,
    recipient_specific_rules JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Add indexes for better query performance
CREATE INDEX idx_expenses_user_id ON expenses(user_id);
CREATE INDEX idx_expenses_date ON expenses(date);
CREATE INDEX idx_expenses_category ON expenses(category);
CREATE INDEX idx_expenses_status ON expenses(status);
CREATE INDEX idx_receipts_user_id ON receipts(user_id);
CREATE INDEX idx_receipts_processing_status ON receipts(processing_status);
CREATE INDEX idx_expense_sharing_preferences_user_id ON expense_sharing_preferences(user_id);