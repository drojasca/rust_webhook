CREATE TABLE event_subscriptions (
    event_type TEXT REFERENCES event_types(event_type) ON DELETE CASCADE,
    sub_id UUID REFERENCES subscriptions(sub_id) ON DELETE CASCADE,
    PRIMARY KEY (event_type, sub_id)
);