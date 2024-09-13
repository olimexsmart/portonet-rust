use axum::{extract::State, response::IntoResponse, Json};

// Importing
use crate::db_access::table_keys::get_keys;

// Handler to respond to API request
pub async fn list_keys(State(pool): State<sqlx::PgPool>) -> impl IntoResponse {
    // Fetch data from the database
    let data = get_keys(pool).await.expect("Failed to fetch data");
    // Return the data as JSON
    Json(data)
}
