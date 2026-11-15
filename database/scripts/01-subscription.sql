CREATE TABLE IF NOT EXISTS subscriptions (
    sub_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    url TEXT NOT NULL,
    auth_id TEXT NOT NULL
);