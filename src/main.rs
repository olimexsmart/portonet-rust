// Importing crates
use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
// Importing DB Access functions
mod db_access;
// Importing API handlers
mod api_list_keys;
use api_list_keys::list_keys;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env file
                   // Create a connection pool for Postgres
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create Postgres connection pool");

    // Build the application with a route
    let app = Router::new()
        .route("/listKeys", get(list_keys))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
