CREATE TABLE IF NOT EXISTS api_keys (
    key_hash TEXT PRIMARY KEY UNIQUE,
    permission TEXT NOT NULL
        CHECK (permission IN ('admin', 'subscribe', 'publish')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    client_id TEXT REFERENCES clients(client_id) ON DELETE CASCADE
);
