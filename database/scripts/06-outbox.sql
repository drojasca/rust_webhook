CREATE TABLE IF NOT EXISTS outbox (
    event_id UUID PRIMARY KEY REFERENCES events(event_id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'Unpublished'
        CHECK (status IN ('Unpublished', 'Published', 'Complete', 'Failed')),
    remaining INTEGER NOT NULL DEFAULT -1
        CHECK (remaining >= -1)
);