DROP TABLE IF EXISTS subscription_urls CASCADE;
DROP TABLE IF EXISTS subscriptions CASCADE;

CREATE TABLE subscriptions (
    sub_id UUID UNIQUE DEFAULT gen_random_uuid(),
    key_id UUID REFERENCES api_keys(key_id) ON DELETE CASCADE,
    event_type TEXT REFERENCES event_types(event_type) ON DELETE CASCADE,
    PRIMARY KEY (key_id, event_type)

);

CREATE TABLE subscription_urls (
    sub_id UUID NOT NULL REFERENCES subscriptions(sub_id) ON DELETE CASCADE,
    post_url TEXT NOT NULL,
    PRIMARY KEY (sub_id, post_url)
);