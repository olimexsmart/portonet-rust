use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

#[derive(Serialize)]
struct ExampleData {
    ukey: String,
}

// Handler function to fetch data from Postgres
async fn fetch_data(pool: sqlx::PgPool) -> Result<Vec<ExampleData>, sqlx::Error> {
    let rows = sqlx::query!("SELECT uKey FROM keys")
        .fetch_all(&pool)
        .await?;

    let data = rows.into_iter()
        .map(|row| ExampleData {
            ukey: row.ukey
        })
        .collect();

    Ok(data)
}

// Handler to respond to API request
async fn example_handler(pool: sqlx::PgPool) -> Json<Vec<ExampleData>> {
    // Fetch data from the database
    let data = fetch_data(pool).await.expect("Failed to fetch data");

    // Return the data as JSON
    Json(data)
}

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
        .route("/data", get({
            let pool = pool.clone();
            move || example_handler(pool)
        }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
