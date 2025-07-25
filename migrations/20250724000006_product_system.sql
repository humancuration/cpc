-- Create enums for product system
CREATE TYPE verification_status AS ENUM ('UNVERIFIED', 'PENDING', 'VERIFIED', 'DISPUTED', 'REJECTED');
CREATE TYPE supply_chain_stage AS ENUM ('RAW_MATERIAL', 'MANUFACTURING', 'PROCESSING', 'PACKAGING', 'DISTRIBUTION', 'WHOLESALE', 'RETAIL', 'CONSUMER');

-- Create products table
CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(255) NOT NULL,
    brand VARCHAR(255),
    sku VARCHAR(255),
    barcode VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create product_origins table
CREATE TABLE IF NOT EXISTS product_origins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    country_of_origin VARCHAR(255) NOT NULL,
    region VARCHAR(255),
    city VARCHAR(255),
    manufacturer VARCHAR(255) NOT NULL,
    manufacturer_address TEXT,
    certification_info TEXT,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    verification_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(product_id)
);

-- Create supply_chains table
CREATE TABLE IF NOT EXISTS supply_chains (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    stage supply_chain_stage NOT NULL,
    location VARCHAR(255) NOT NULL,
    organization VARCHAR(255) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    description TEXT,
    verification_status verification_status NOT NULL DEFAULT 'UNVERIFIED',
    previous_stage_id UUID REFERENCES supply_chains(id) ON DELETE SET NULL,
    next_stage_id UUID REFERENCES supply_chains(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_products_name ON products(name);
CREATE INDEX IF NOT EXISTS idx_products_category ON products(category);
CREATE INDEX IF NOT EXISTS idx_products_brand ON products(brand);
CREATE INDEX IF NOT EXISTS idx_products_sku ON products(sku);
CREATE INDEX IF NOT EXISTS idx_products_barcode ON products(barcode);

CREATE INDEX IF NOT EXISTS idx_product_origins_product_id ON product_origins(product_id);
CREATE INDEX IF NOT EXISTS idx_product_origins_country ON product_origins(country_of_origin);
CREATE INDEX IF NOT EXISTS idx_product_origins_verified ON product_origins(verified);

CREATE INDEX IF NOT EXISTS idx_supply_chains_product_id ON supply_chains(product_id);
CREATE INDEX IF NOT EXISTS idx_supply_chains_stage ON supply_chains(stage);
CREATE INDEX IF NOT EXISTS idx_supply_chains_timestamp ON supply_chains(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_supply_chains_verification_status ON supply_chains(verification_status);
CREATE INDEX IF NOT EXISTS idx_supply_chains_previous_stage ON supply_chains(previous_stage_id);
CREATE INDEX IF NOT EXISTS idx_supply_chains_next_stage ON supply_chains(next_stage_id);