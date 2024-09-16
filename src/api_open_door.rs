use crate::custom_error_mapper::AppError;
use crate::db_access::table_keys::{check_key, KeyCheckResult};
use crate::db_access::table_system::{handle_attempt_failed, handle_attempt_ok, is_system_locked};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct KeyQueryParams {
    u_key: String,
}

pub async fn open_door(
    State(pool): State<sqlx::PgPool>,
    Query(params): Query<KeyQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    match is_system_locked(&pool).await? {
        false => {

            match check_key(&pool, params.u_key).await? {
                KeyCheckResult::Valid => {
                    // TODO implement env-based API call
                    println!("door opened TODO proper call");
                    handle_attempt_ok(&pool).await?;
                    Ok(())
                }
                KeyCheckResult::Expired => {
                    handle_attempt_failed(&pool).await?;
                    Err(AppError::Expired)
                }
                KeyCheckResult::Revoked => {
                    handle_attempt_failed(&pool).await?;
                    Err(AppError::Revoked)
                }
                KeyCheckResult::Invalid => {
                    handle_attempt_failed(&pool).await?;
                    Err(AppError::NotFound)
                }
            }
        }
        true => Err(AppError::Locked),
    }
}
