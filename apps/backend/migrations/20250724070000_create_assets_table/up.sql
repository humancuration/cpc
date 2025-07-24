CREATE TABLE assets (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    path VARCHAR NOT NULL UNIQUE,
    size BIGINT NOT NULL,
    asset_type VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    thumbnail_path VARCHAR
);

CREATE INDEX idx_assets_path ON assets(path);
CREATE INDEX idx_assets_type ON assets(asset_type);
CREATE INDEX idx_assets_created ON assets(created_at);