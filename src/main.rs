use axum::{
    routing::get,
    Router,
    Json,
};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
mod db_structs;
use db_structs::UKey;


// Handler function to fetch data from Postgres
async fn fetch_data(pool: sqlx::PgPool) -> Result<Vec<UKey>, sqlx::Error> {
    let rows = sqlx::query!("SELECT * FROM keys")
        .fetch_all(&pool)
        .await?;

    let data = rows.into_iter()
        .map(|row| UKey {
            id: row.id,
            ukey: row.ukey,
            exp_date: row.expdate,
            last_used: row.lastused,
            n_used: row.nused,
            revoked: row.revoked != 0
        })
        .collect();

    Ok(data)
}

// Handler to respond to API request
async fn example_handler(pool: sqlx::PgPool) -> Json<Vec<UKey>> {
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
