use crate::custom_error_mapper::APIError;
use crate::db_access::table_keys::{check_key, KeyCheckResult};
use crate::db_access::table_logs::insert_log;
use crate::db_access::table_system::{handle_attempt_failed, handle_attempt_ok, is_system_locked};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
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
                    // TODO implement env-based API call
                    println!("door opened TODO proper call");
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
