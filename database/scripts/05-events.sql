CREATE TABLE events (
    event_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type TEXT NOT NULL REFERENCES event_types(event_type),
    event_date TIMESTAMPTZ NOT NULL,
    payload JSONB
);