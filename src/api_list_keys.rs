use crate::{
    custom_error_mapper::AppError,
    db_access::{table_keys::get_keys, table_system::check_master_password},
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct KeyQueryParams {
    master_password: String,
}

// Handler to respond to API request
pub async fn list_keys(
    State(pool): State<sqlx::PgPool>,
    Query(params): Query<KeyQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    match check_master_password(pool.clone(), params.master_password).await? {
        true => {
            // If the master password is correct, fetch the keys
            let data = get_keys(pool).await?;
            // Return the data as JSON
            Ok(Json(data))
        }
        false => {
            // Master password is incorrect
            Err(AppError::Unauthorized)
        }
    }
}
