mod api_key;
mod auth;

use axum::middleware::from_fn_with_state;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use jsonschema::Validator;
use lapin::{Channel, Connection, ConnectionProperties};
use serde::Deserialize;
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::collections::HashMap;
use std::error::Error;

use crate::auth::AuthUser;

const UNIQUE_VIOLATION_SQLSTATE: &str = "23505";

#[derive(Debug, sqlx::FromRow)]
struct EventType {
    event_type: String,
    event_schema: serde_json::Value,
}

#[derive(Serialize)]
struct ApiKey {
    api_key: String,
}

#[derive(Serialize)]
struct SubId {
    sub_id: String,
}

#[derive(Clone)]
struct AppState {
    validators: HashMap<String, Validator>,
    pool: Pool<Postgres>,
    queue: Channel,
}

#[derive(Deserialize)]
struct EventPayload {
    event_type: String,
    payload: serde_json::Value,
}

#[derive(Deserialize)]
struct SubscribePayload {
    post_urls: Vec<String>,
    event: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = initialize_db().await?;
    let validators = initialize_event_schemas(&pool).await?;
    let channel = initialize_rabbitmq().await?;

    let app_state = AppState {
        validators,
        pool,
        queue: channel,
    };

    let app: Router = Router::new()
        .route("/event", post(event_handler))
        .route("/create/subscriber", post(create_sub_handler))
        .route(
            "/create/publisher",
            post(create_pub_handler).layer(from_fn_with_state(
                app_state.clone(),
                auth::require_admin,
            )),
        )
        .route(
            "/subscribe",
            post(sub_handler).layer(from_fn_with_state(
                app_state.clone(),
                auth::require_sub,
            )),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn initialize_event_schemas(
    pool: &Pool<Postgres>,
) -> Result<HashMap<String, Validator>, Box<dyn Error>> {
    let mut validator_map: HashMap<String, Validator> = HashMap::new();
    let event_types = sqlx::query_as::<Postgres, EventType>(
        "SELECT event_type, event_schema FROM event_types",
    )
    .fetch_all(pool)
    .await?;

    for event_type in event_types {
        let validator = jsonschema::validator_for(&event_type.event_schema)?;
        validator_map.insert(event_type.event_type, validator);
    }

    Ok(validator_map)
}

async fn initialize_db() -> Result<Pool<Postgres>, sqlx::Error> {
    let url = "postgresql://user:password@db:5432/my_database";
    let pool = PgPoolOptions::new().connect(&url).await?;
    println!("Connected to the database!");
    Ok(pool)
}

async fn initialize_rabbitmq() -> Result<Channel, Box<dyn Error>> {
    // connect to RabbitMQ server
    let connection = Connection::connect(
        "amqp://guest:guest@rabbitmq:5672", // rabbitMQ connection string
        ConnectionProperties::default(),
    )
    .await?;

    // create a communication channel
    let channel = connection.create_channel().await?;

    println!("Initialized rabbitmq!");
    Ok(channel)
}

async fn event_handler(
    State(state): State<AppState>,
    Json(body): Json<EventPayload>,
) -> &'static str {
    let Some(validator) = state.validators.get(&body.event_type) else {
        return "Unknown\n";
    };

    if validator.is_valid(&body.payload) {
        "Valid!\n"
    } else {
        "Invalid\n"
    }
}

async fn create_sub_handler(
    State(state): State<AppState>,
) -> Result<Json<ApiKey>, (StatusCode, &'static str)> {
    let api_key = api_key::generate_api_key(api_key::Permission::Subscriber);
    let key_hash = api_key::hash_key(&api_key);

    let _ = sqlx::query!(
        "INSERT INTO api_keys (key_hash, permission)
        VALUES ($1, $2)",
        key_hash,
        "subscribe"
    )
    .execute(&state.pool)
    .await
    .map(|err| {
        tracing::error!("Failed to execute query: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    });
    let res = ApiKey { api_key };
    Ok(Json(res))
}

async fn create_pub_handler(
    State(state): State<AppState>,
) -> Result<Json<ApiKey>, (StatusCode, &'static str)> {
    let api_key = api_key::generate_api_key(api_key::Permission::Publisher);
    let key_hash = api_key::hash_key(&api_key);

    let _ = sqlx::query!(
        "INSERT INTO api_keys (key_hash, permission)
        VALUES ($1, $2)",
        key_hash,
        "publish"
    )
    .execute(&state.pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute query: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    });
    let res = ApiKey { api_key };
    Ok(Json(res))
}

async fn sub_handler(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(body): Json<SubscribePayload>,
) -> Result<Json<SubId>, (StatusCode, &'static str)> {
    if !state.validators.contains_key(&body.event) {
        return Err((StatusCode::BAD_REQUEST, "Unknown event"));
    }
    let mut tx = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to begin transaction: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    })?;

    let res = sqlx::query!(
        "INSERT INTO subscriptions (key_id, event_type)
            VALUES ($1, $2)
            RETURNING sub_id",
        &auth_user.key_id,
        &body.event
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|err| {
        let is_dup = err.as_database_error().is_some_and(|db_err| {
            db_err.code().as_deref() == Some(UNIQUE_VIOLATION_SQLSTATE)
                && db_err.constraint() == Some("subscriptions_pkey")
        });
        if is_dup {
            return (StatusCode::CONFLICT, "Subscription already exists");
        }
        tracing::error!("Failed to create subscription: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    })?;
    let sub_id = res.sub_id.ok_or_else(|| {
        tracing::error!("Created subscription without a sub_id");
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    })?;

    for url in body.post_urls {
        let _ = sqlx::query!(
            "INSERT INTO subscription_urls (sub_id, post_url)
            VALUES ($1, $2)",
            sub_id,
            url
        )
        .execute(&mut *tx)
        .await
        .map_err(|err| {
            tracing::error!(
                "Failed to associate subscription with url: {:?}",
                err
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        })?;
    }

    tx.commit().await.map_err(|err| {
        tracing::error!("Transaction Failed: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    })?;

    Ok(Json(SubId {
        sub_id: sub_id.to_string(),
    }))
}
