use axum::{extract::State, routing::get, Router};
use lapin::{Channel, Connection, ConnectionProperties};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::error::Error;

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
    channel: Channel,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = initialize_db().await?;
    let channel = initialize_rabbitmq().await?;

    let app_state = AppState { pool, channel };

    let app: Router = Router::new().route("/", get(root).with_state(app_state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
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

async fn root(State(state): State<AppState>) -> &'static str {
    "Hello, world!"
}
