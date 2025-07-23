CREATE TABLE impact_metrics (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    unit TEXT NOT NULL,
    calculation_formula TEXT NOT NULL
);

CREATE TABLE impact_reports (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    period_start INTEGER NOT NULL,
    period_end INTEGER NOT NULL,
    overall_score REAL NOT NULL,
    category_distribution TEXT NOT NULL,
    signature TEXT
);

CREATE TABLE impact_data_points (
    id TEXT PRIMARY KEY,
    report_id TEXT NOT NULL,
    source TEXT NOT NULL,
    value REAL NOT NULL,
    timestamp INTEGER NOT NULL,
    category TEXT NOT NULL,
    metadata TEXT NOT NULL
);