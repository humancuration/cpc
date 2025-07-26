-- migrations/20250726_impact_weights_table.sql
CREATE TABLE impact_weights (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    category VARCHAR(50) NOT NULL CHECK (category IN ('Community', 'Environment', 'Workers')),
    weight DECIMAL(3,2) NOT NULL CHECK (weight BETWEEN 0 AND 1),
    PRIMARY KEY (user_id, category)
);

-- Initialize default weights for existing users
INSERT INTO impact_weights (user_id, category, weight)
SELECT 
    id, 
    category, 
    weight
FROM 
    users
CROSS JOIN (VALUES 
    ('Community', 0.45),
    ('Environment', 0.30),
    ('Workers', 0.25)
) AS defaults(category, weight);