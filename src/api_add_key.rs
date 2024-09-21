use crate::db_access::table_logs::insert_log;
use crate::db_access::table_system::{check_master_password, handle_attempt_failed};
use crate::{custom_error_mapper::APIError, db_access::table_keys::insert_or_update_key};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct KeyQueryParams {
    master_password: String,
    new_key: String,
    #[serde(
        deserialize_with = "deserialize_duration",
        serialize_with = "serialize_duration"
    )] // Custom deserializer
    duration: Duration,
}

use ::function_name::named;

#[named]
pub async fn add_key(
    State(pool): State<sqlx::PgPool>,
    Query(params): Query<KeyQueryParams>,
) -> Result<impl IntoResponse, APIError> {
    insert_log(
        &pool,
        function_name!(),
        Some(serde_json::to_string(&params).unwrap()),
    )
    .await?;

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
            Err(APIError::Unauthorized)
        }
    }
}

/*
 * PRIVATE
 */
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

fn serialize_duration<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted_duration = if duration.num_hours() % 24 != 0 {
        format!("{}h", duration.num_hours())
    } else if duration.num_days() % 30 != 0 {
        format!("{}d", duration.num_days())
    } else {
        format!("{}m", duration.num_days() / 30) // Approximate months as 30 days
    };

    serializer.serialize_str(&formatted_duration)
}
