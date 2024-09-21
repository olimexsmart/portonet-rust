use crate::{
    custom_error_mapper::APIError,
    db_access::{
        table_keys::select_keys,
        table_logs::insert_log,
        table_system::{check_master_password, handle_attempt_failed},
    },
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyQueryParams {
    master_password: String,
}

use ::function_name::named;

#[named]
pub async fn list_keys(
    State(pool): State<sqlx::PgPool>,
    Query(params): Query<KeyQueryParams>,
) -> Result<impl IntoResponse, APIError> {
    insert_log(
        &pool,
        function_name!(),
        Some(serde_json::to_string(&params).unwrap()),
    )
    .await?;

    match check_master_password(&pool, params.master_password).await? {
        true => {
            // If the master password is correct, fetch the keys
            let data = select_keys(&pool).await?;
            // Return the data as JSON
            Ok(Json(data))
        }
        false => {
            handle_attempt_failed(&pool).await?;
            Err(APIError::Unauthorized)
        }
    }
}
