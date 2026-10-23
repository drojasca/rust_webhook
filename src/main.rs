use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let url = "postgresql://user:password@db:5432/my_database";
    let pool = PgPoolOptions::new().connect(&url).await?;
    println!("Connected to the database!");
    let app: Router = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}

async fn root() -> &'static str {
    "Hello, world!"
}