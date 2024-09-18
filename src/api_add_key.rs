use crate::db_access::table_system::{check_master_password, handle_attempt_failed};
use crate::{custom_error_mapper::AppError, db_access::table_keys::insert_or_update_key};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
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
) -> Result<impl IntoResponse, AppError> {
    // First, handle the result of checking the master password
    match check_master_password(&pool, params.master_password).await? {
        true => {
            // Master password is correct, proceed with inserting or updating the key
            let result = insert_or_update_key(
                &pool,
                params.new_key.clone(),
                (Utc::now() + params.duration).naive_utc(),
            )
            .await?;
            Ok(result.to_string())
        }
        false => {
            handle_attempt_failed(&pool).await?;
            Err(AppError::Unauthorized)
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
