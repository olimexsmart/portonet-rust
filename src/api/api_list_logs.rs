use crate::{
    custom_error_mapper::APIError,
    db_access::{
        table_logs::{insert_log, select_logs},
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
    limit: Option<i64>
}

use ::function_name::named;

#[named]
pub async fn list_logs(
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
            // Limit number of logs to 10k
            let data = select_logs(&pool, params.limit.unwrap_or(10000)).await?;
            // Return the data as JSON
            Ok(Json(data))
        }
        false => {
            handle_attempt_failed(&pool).await?;
            Err(APIError::Unauthorized)
        }
    }
}
