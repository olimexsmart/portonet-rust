use crate::{
    custom_error_mapper::AppError,
    db_access::{table_keys::update_revoke_all_keys, table_system::check_master_password},
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct KeyQueryParams {
    master_password: String,
}

pub async fn revoke_all_keys(
    State(pool): State<sqlx::PgPool>,
    Query(params): Query<KeyQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    match check_master_password(&pool, params.master_password).await? {
        true => {
            update_revoke_all_keys(&pool).await?;
            Ok(())
        }
        false => {
            // Master password is incorrect
            Err(AppError::Unauthorized)
        }
    }
}
