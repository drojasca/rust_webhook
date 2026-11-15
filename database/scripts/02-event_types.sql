CREATE TABLE IF NOT EXISTS event_types (
    event_type TEXT PRIMARY KEY,
    event_schema JSONB NOT NULL
);

INSERT INTO event_types (event_type, event_schema)
VALUES (
    'post.created',
    '{
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "type": "object",
      "properties": {
        "occurred_at": { "type": "string", "format": "date-time" },
        "post_id": { "type": "string" },
        "author_id": { "type": "string" },
        "text": { "type": "string" }
      },
      "required": ["occurred_at", "post_id", "author_id", "text"]
    }'::jsonb
);

INSERT INTO event_types (event_type, event_schema)
VALUES (
    'comment.added',
    '{
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "type": "object",
      "properties": {
        "occurred_at": { "type": "string", "format": "date-time" },
        "post_id": { "type": "string" },
        "post_author_id": { "type": "string" },
        "author_id": { "type": "string" },
        "text": { "type": "string" },
        "comment_id": { "type": "string" }
      },
      "required": ["occurred_at", "post_id", "post_author_id", "author_id", "text", "comment_id"]
    }'::jsonb
);

INSERT INTO event_types (event_type, event_schema)
VALUES (
    'reaction.added',
    '{
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "type": "object",
      "properties": {
        "occurred_at": { "type": "string", "format": "date-time" },
        "reaction_id": { "type": "string" },
        "post_id": { "type": "string" },
        "post_author_id": { "type": "string" },
        "user_id": { "type": "string" },
        "kind": { "type": "string" }
      },
      "required": ["occurred_at", "reaction_id", "post_id", "post_author_id", "user_id", "kind"]
    }'::jsonb
);

INSERT INTO event_types (event_type, event_schema)
VALUES (
    'user.followed',
    '{
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "type": "object",
      "properties": {
        "occurred_at": { "type": "string", "format": "date-time" },
        "follower_id": { "type": "string" },
        "followee_id": { "type": "string" },
        "followed_at": { "type": "string", "format": "date-time" }
      },
      "required": ["occurred_at", "follower_id", "followee_id", "followed_at"]
    }'::jsonb
);
