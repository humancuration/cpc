-- Create table for storing Double Ratchet sessions for end-to-end encryption

CREATE TABLE ratchet_sessions (
    document_id UUID NOT NULL REFERENCES documents(id),
    node_id UUID NOT NULL,
    session_data BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (document_id, node_id)
);

-- Indexes for performance
CREATE INDEX idx_ratchet_sessions_document_id ON ratchet_sessions(document_id);
CREATE INDEX idx_ratchet_sessions_node_id ON ratchet_sessions(node_id);
CREATE INDEX idx_ratchet_sessions_created_at ON ratchet_sessions(created_at);