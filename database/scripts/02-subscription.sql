CREATE TABLE IF NOT EXISTS subscriptions (
    sub_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    url TEXT NOT NULL,
    api_key TEXT REFERENCES api_keys(key_hash) ON DELETE CASCADE
);