# rust_webhook
## Testing
```sh
docker compose up -d db --build
DATABASE_URL='postgres://user:password@localhost:5433/my_database' cargo sqlx prepare
docker compose up --build
```

In another terminal, send requests to exercise the `/event` handler:

To open a shell in the running app container:

```sh
docker compose exec server sh
```

From inside the container, send a request with `curl`:

```sh
curl -i -X POST http://localhost:3000/event \
  -H 'Content-Type: application/json' \
  -d '{"event_type":"does.not.exist","payload":{}}'
```

The same `curl` command works from your host by using `http://localhost:3000/event`.
The image installs curl, so rebuild with `docker compose up --build` after changing the Dockerfile.

Create a subscriber API key (returns the generated key as JSON):

```sh
curl -i -X POST http://localhost:3000/create/subscriber
```

Create a publisher API key (replace the placeholder with an authorized API key):

```sh
curl -i -X POST http://localhost:3000/create/publisher \
  -H 'bearer: YOUR_ADMIN_API_KEY'
```

Subscribe a subscriber API key to an event and one or more webhook URLs:

```sh
curl -i -X POST http://localhost:3000/subscribe \
  -H 'bearer: YOUR_SUBSCRIBER_API_KEY' \
  -H 'Content-Type: application/json' \
  -d '{"post_urls":["https://example.com/webhooks/post-created"],"event":"post.created"}'
```

Other example requests, which can also be run from your host:

Valid `post.created` payload (returns `Valid!`):

```sh
curl -i -X POST http://localhost:3000/event \
  -H 'Content-Type: application/json' \
  -d '{"event_type":"post.created","payload":{"occurred_at":"2026-11-15T01:00:00Z","post_id":"pst_123","author_id":"usr_123","text":"Hello world"}}'
```

Known event type with an invalid payload (returns `Invalid`):

```sh
curl -i -X POST http://localhost:3000/event \
  -H 'Content-Type: application/json' \
  -d '{"event_type":"post.created","payload":{"post_id":"pst_123"}}'
```

Unknown event type (returns `Unknown`):

```sh
curl -i -X POST http://localhost:3000/event \
  -H 'Content-Type: application/json' \
  -d '{"event_type":"does.not.exist","payload":{}}'
```
