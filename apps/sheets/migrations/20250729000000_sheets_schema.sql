-- Sheets application database schema

-- Table for sheets
CREATE TABLE sheets (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    version BIGINT NOT NULL DEFAULT 1
);

-- Index for owner lookups
CREATE INDEX idx_sheets_owner ON sheets(owner_id);

-- Table for cells
CREATE TABLE cells (
    sheet_id UUID NOT NULL REFERENCES sheets(id) ON DELETE CASCADE,
    row_index INTEGER NOT NULL,
    column_index INTEGER NOT NULL,
    value JSONB NOT NULL,
    formatted_value TEXT NOT NULL,
    style JSONB NOT NULL,
    PRIMARY KEY (sheet_id, row_index, column_index)
);

-- Index for sheet lookups
CREATE INDEX idx_cells_sheet ON cells(sheet_id);

-- Table for formulas
CREATE TABLE formulas (
    sheet_id UUID NOT NULL REFERENCES sheets(id) ON DELETE CASCADE,
    row_index INTEGER NOT NULL,
    column_index INTEGER NOT NULL,
    expression TEXT NOT NULL,
    dependencies JSONB NOT NULL,
    last_evaluated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    cache JSONB,
    PRIMARY KEY (sheet_id, row_index, column_index)
);

-- Index for formula sheet lookups
CREATE INDEX idx_formulas_sheet ON formulas(sheet_id);

-- Table for charts
CREATE TABLE charts (
    id UUID PRIMARY KEY,
    sheet_id UUID NOT NULL REFERENCES sheets(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    chart_type VARCHAR(50) NOT NULL,
    data_range JSONB NOT NULL,
    series_config JSONB NOT NULL,
    options JSONB NOT NULL
);

-- Index for chart sheet lookups
CREATE INDEX idx_charts_sheet ON charts(sheet_id);

-- Table for permissions
CREATE TABLE permissions (
    sheet_id UUID NOT NULL REFERENCES sheets(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    level VARCHAR(20) NOT NULL,
    PRIMARY KEY (sheet_id, user_id)
);

-- Index for permission user lookups
CREATE INDEX idx_permissions_user ON permissions(user_id);

-- Table for collaborative editing sessions
CREATE TABLE collaboration_sessions (
    id UUID PRIMARY KEY,
    sheet_id UUID NOT NULL REFERENCES sheets(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Index for session sheet lookups
CREATE INDEX idx_collaboration_sessions_sheet ON collaboration_sessions(sheet_id);

-- Table for collaborative edits
CREATE TABLE collaborative_edits (
    id UUID PRIMARY KEY,
    session_id UUID NOT NULL REFERENCES collaboration_sessions(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    edit_type VARCHAR(50) NOT NULL,
    data JSONB NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for edit session lookups
CREATE INDEX idx_collaborative_edits_session ON collaborative_edits(session_id);

-- Table for import/export history
CREATE TABLE import_export_history (
    id UUID PRIMARY KEY,
    sheet_id UUID NOT NULL REFERENCES sheets(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    operation VARCHAR(10) NOT NULL, -- 'import' or 'export'
    file_format VARCHAR(10) NOT NULL, -- 'xlsx' or 'csv'
    file_path TEXT,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for history sheet lookups
CREATE INDEX idx_import_export_history_sheet ON import_export_history(sheet_id);