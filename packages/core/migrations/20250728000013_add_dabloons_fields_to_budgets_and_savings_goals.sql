-- Add Dabloons support fields to budgets table
ALTER TABLE budgets 
ADD COLUMN dabloons_allocated DECIMAL(20, 0) DEFAULT 0,
ADD COLUMN dabloons_spent DECIMAL(20, 0) DEFAULT 0,
ADD COLUMN currency_type VARCHAR(20) DEFAULT 'TraditionalOnly';

-- Add Dabloons support fields to savings_goals table
ALTER TABLE savings_goals
ADD COLUMN target_dabloons DECIMAL(20, 0) DEFAULT 0,
ADD COLUMN current_dabloons DECIMAL(20, 0) DEFAULT 0,
ADD COLUMN currency_type VARCHAR(20) DEFAULT 'TraditionalOnly';

-- Update existing budgets to have TraditionalOnly currency type
UPDATE budgets SET currency_type = 'TraditionalOnly' WHERE currency_type IS NULL;

-- Update existing savings_goals to have TraditionalOnly currency type
UPDATE savings_goals SET currency_type = 'TraditionalOnly' WHERE currency_type IS NULL;

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_budgets_currency_type ON budgets(currency_type);
CREATE INDEX IF NOT EXISTS idx_savings_goals_currency_type ON savings_goals(currency_type);