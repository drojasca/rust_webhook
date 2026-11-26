CREATE TABLE IF NOT EXISTS subscriptions (
    sub_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    url TEXT NOT NULL,
    key_id UUID REFERENCES api_keys(key_id) ON DELETE CASCADE
);