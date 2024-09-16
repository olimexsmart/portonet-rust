use crate::db_access::table_keys::insert_or_update_key;
use crate::db_access::table_system::check_master_password;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct KeyQueryParams {
    master_password: String,
    new_key: String,
    #[serde(deserialize_with = "deserialize_duration")] // Custom deserializer
    duration: Duration,
}

// Handler to respond to API request
pub async fn add_key(
    State(pool): State<sqlx::PgPool>,
    Query(params): Query<KeyQueryParams>,
) -> impl IntoResponse {
    // First, handle the result of checking the master password
    match check_master_password(&pool, params.master_password).await {
        Ok(true) => {
            // Master password is correct, proceed with inserting or updating the key
            let result = insert_or_update_key(
                &pool,
                params.new_key.clone(),
                (Utc::now() + params.duration).naive_utc(),
            )
            .await;

            // Match on the result of the insertion
            match result {
                Ok(result) => (StatusCode::CREATED, result.to_string()).into_response(),
                Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
            }
        }
        Ok(false) => {
            // Master password is incorrect
            (StatusCode::UNAUTHORIZED, "Wrong Master Password").into_response()
        }
        Err(err) => { // TODO uniform erro handling
            // An error occurred while checking the master password
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

// Custom deserialization logic for `chrono::Duration`
fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if let Some(hours) = s.strip_suffix("h") {
        if let Ok(h) = i64::from_str(hours) {
            return Ok(Duration::hours(h));
        }
    } else if let Some(days) = s.strip_suffix("d") {
        if let Ok(d) = i64::from_str(days) {
            return Ok(Duration::days(d));
        }
    } else if let Some(months) = s.strip_suffix("m") {
        if let Ok(m) = i64::from_str(months) {
            return Ok(Duration::days(m * 30)); // Approximate month as 30 days
        }
    }

    Err(serde::de::Error::custom("Invalid duration format"))
}
