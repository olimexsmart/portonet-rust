use std::collections::HashMap;

use crate::custom_error_mapper::APIError;
use crate::db_access::table_keys::{check_key, KeyCheckResult};
use crate::db_access::table_logs::insert_log;
use crate::db_access::table_system::{handle_attempt_failed, handle_attempt_ok, is_system_locked};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyQueryParams {
    u_key: String,
}

use ::function_name::named;

#[named]
pub async fn open_door(
    State(pool): State<sqlx::PgPool>,
    Query(params): Query<KeyQueryParams>,
) -> Result<impl IntoResponse, APIError> {
    insert_log(
        &pool,
        function_name!(),
        Some(serde_json::to_string(&params).unwrap()),
    )
    .await?;

    match is_system_locked(&pool).await? {
        false => {
            match check_key(&pool, params.u_key).await? {
                KeyCheckResult::Valid => {
                    // Send the POST request to Home Assistant
                    let bearer_token_home_assistant = std::env::var("BEARER_HOME_ASSISTANT")
                        .map_err(|_| {
                            APIError::EnvError("BEARER_HOME_ASSISTANT not set".to_string())
                        })?;
                    let url_home_assistant = std::env::var("URL_HOME_ASSISTANT").map_err(|_| {
                        APIError::EnvError("URL_HOME_ASSISTANT not set".to_string())
                    })?;
                    let entity_home_assistant =
                        std::env::var("ENTITY_HOME_ASSISTANT").map_err(|_| {
                            APIError::EnvError("ENTITY_HOME_ASSISTANT not set".to_string())
                        })?;
                    let mut map = HashMap::new();
                    map.insert("entity_id", entity_home_assistant);
                    let response = Client::new()
                        .post(url_home_assistant)
                        .header(
                            "Authorization",
                            format!("Bearer {}", bearer_token_home_assistant),
                        ) // Authorization header
                        .header("Content-Type", "application/json") // Content-Type header
                        .json(&map) // The JSON body
                        .send() // Send the request
                        .await
                        .map_err(|_| APIError::HomeAssistantError)?;
                    // Send error to client informing if request failed
                    if response.status().is_success() {
                        println!("DOOR OPENED");
                    } else {
                        return Err(APIError::HomeAssistantError);
                    }
                    // Succesful return point
                    handle_attempt_ok(&pool).await?;
                    Ok("OK")
                }
                KeyCheckResult::Expired => {
                    handle_attempt_failed(&pool).await?;
                    Err(APIError::Expired)
                }
                KeyCheckResult::Revoked => {
                    handle_attempt_failed(&pool).await?;
                    Err(APIError::Revoked)
                }
                KeyCheckResult::Invalid => {
                    handle_attempt_failed(&pool).await?;
                    Err(APIError::NotFound)
                }
            }
        }
        true => Err(APIError::Locked),
    }
}
