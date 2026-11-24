mod api_key;

use axum::{extract::State, routing::post, Json, Router};
use jsonschema::Validator;
use lapin::{Channel, Connection, ConnectionProperties};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, sqlx::FromRow)]
struct EventType {
    event_type: String,
    event_schema: serde_json::Value,
}

#[derive(Clone)]
struct EventState {
    validators: HashMap<String, Validator>,
}

#[derive(Deserialize)]
struct EventPayload {
    event_type: String,
    payload: serde_json::Value,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = initialize_db().await?;
    let validators = initialize_event_schemas(&pool).await?;
    let channel = initialize_rabbitmq().await?;

    let event_state = EventState { validators };

    let app: Router = Router::new()
        .route("/event", post(event_handler).with_state(event_state));
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
    State(state): State<EventState>,
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
