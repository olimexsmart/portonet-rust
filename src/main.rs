use api::{
    api_add_key::add_key, api_get_counters::get_counters, api_list_keys::list_keys,
    api_list_logs::list_logs, api_open_door::open_door, api_revoke_all_keys::revoke_all_keys,
    api_revoke_key::revoke_key,
};
// Importing crates
use axum::{
    routing::{get, post, put},
    Router,
};
use tower_http::services::ServeDir;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
// Importing DB Access functions
mod db_access;
// Importing API handlers
mod api;
mod custom_error_mapper;

#[tokio::main]
async fn main() {
    println!("PortoNet backend START");
    // Load environment variables from .env file
    dotenv().ok();
    // Create a connection pool for Postgres
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create Postgres connection pool");
    // Build the application with a route
    let app = Router::new()
        .route("/list_keys", get(list_keys))
        .route("/add_key", post(add_key))
        .route("/get_counters", get(get_counters))
        .route("/revoke_key", put(revoke_key))
        .route("/revoke_all_keys", put(revoke_all_keys))
        .route("/open_door", put(open_door))
        .route("/list_logs", get(list_logs))
        .nest_service("/", ServeDir::new("frontend"))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
