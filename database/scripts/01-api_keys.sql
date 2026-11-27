DROP TABLE IF EXISTS api_keys CASCADE;

CREATE TABLE IF NOT EXISTS api_keys (
    key_hash TEXT NOT NULL UNIQUE,
    key_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    permission TEXT NOT NULL
        CHECK (permission IN ('admin', 'subscribe', 'publish')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
