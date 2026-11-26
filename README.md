# rust_webhook

## Starting PostgreSQL locally

```sh
docker compose up -d db --build
```

To compile code that uses SQLx's `query!` macro, make the database URL available
to Cargo so SQLx can validate queries against the schema:

```sh
DATABASE_URL='postgres://user:password@localhost:5433/my_database' cargo check
```

Keep PostgreSQL running while compiling. Stop it when you are done with:

```sh
docker compose stop db
```

The Docker image build uses SQLx's offline query metadata in `.sqlx/`. Refresh
that metadata with `cargo sqlx prepare` when a `query!` query or its schema
changes, and include `.sqlx/` in the build context.

## Testing
Always build the app image before starting the app and its dependencies (use this instead of plain `docker compose up`):

```sh
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
curl -i -X POST http://localhost:3000/create/subscribe
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
