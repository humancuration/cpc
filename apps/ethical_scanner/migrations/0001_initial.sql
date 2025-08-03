-- Initial migration for EthicalScanner database
-- Create tables for products, suppliers, and related data

-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Products table
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    barcode VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    brand VARCHAR(255),
    category VARCHAR(255),
    health_score DECIMAL(3,2) CHECK (health_score >= 0 AND health_score <= 1),
    ethical_score DECIMAL(3,2) CHECK (ethical_score >= 0 AND ethical_score <= 1),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Ingredients table
CREATE TABLE ingredients (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID REFERENCES products(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    origin VARCHAR(255),
    is_allergen BOOLEAN DEFAULT FALSE,
    health_impact VARCHAR(20) CHECK (health_impact IN ('Positive', 'Neutral', 'Negative')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Nutritional facts table
CREATE TABLE nutritional_facts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID REFERENCES products(id) ON DELETE CASCADE,
    calories DECIMAL(10,2) CHECK (calories >= 0),
    protein DECIMAL(10,2) CHECK (protein >= 0),
    carbs DECIMAL(10,2) CHECK (carbs >= 0),
    fats DECIMAL(10,2) CHECK (fats >= 0),
    sugars DECIMAL(10,2) CHECK (sugars >= 0),
    fiber DECIMAL(10,2) CHECK (fiber >= 0),
    sodium DECIMAL(10,2) CHECK (sodium >= 0),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Suppliers table
CREATE TABLE suppliers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    location VARCHAR(255),
    ethical_rating DECIMAL(3,2) CHECK (ethical_rating >= 0 AND ethical_rating <= 1),
    environmental_impact DECIMAL(3,2) CHECK (environmental_impact >= 0 AND environmental_impact <= 1),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Supply chain nodes table
CREATE TABLE supply_chain_nodes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID REFERENCES products(id) ON DELETE CASCADE,
    supplier_id UUID REFERENCES suppliers(id),
    step VARCHAR(255) NOT NULL,
    location VARCHAR(255),
    company VARCHAR(255),
    ethical_rating DECIMAL(3,2) CHECK (ethical_rating >= 0 AND ethical_rating <= 1),
    environmental_impact DECIMAL(3,2) CHECK (environmental_impact >= 0 AND environmental_impact <= 1),
    sequence_number INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Alternative suggestions table
CREATE TABLE alternative_suggestions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID REFERENCES products(id) ON DELETE CASCADE,
    alternative_product_id UUID REFERENCES products(id) ON DELETE CASCADE,
    reason VARCHAR(50) CHECK (reason IN ('HealthierOption', 'MoreEthical', 'LocalProducer', 'SustainablePackaging')),
    health_improvement DECIMAL(3,2),
    ethical_improvement DECIMAL(3,2),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for better query performance
CREATE INDEX idx_products_barcode ON products(barcode);
CREATE INDEX idx_products_category ON products(category);
CREATE INDEX idx_ingredients_product_id ON ingredients(product_id);
CREATE INDEX idx_nutritional_facts_product_id ON nutritional_facts(product_id);
CREATE INDEX idx_supply_chain_nodes_product_id ON supply_chain_nodes(product_id);
CREATE INDEX idx_supply_chain_nodes_supplier_id ON supply_chain_nodes(supplier_id);
CREATE INDEX idx_alternative_suggestions_product_id ON alternative_suggestions(product_id);
CREATE INDEX idx_alternative_suggestions_alternative_product_id ON alternative_suggestions(alternative_product_id);

-- Trigger to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_products_updated_at BEFORE UPDATE ON products
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_suppliers_updated_at BEFORE UPDATE ON suppliers
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();