use crate::{
    custom_error_mapper::APIError,
    db_access::{
        table_keys::update_revoke_all_keys, table_logs::insert_log, table_system::{check_master_password, handle_attempt_failed}
    },
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyQueryParams {
    master_password: String,
}

use ::function_name::named;

#[named]
pub async fn revoke_all_keys(
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
            update_revoke_all_keys(&pool).await?;
            Ok(())
        }
        false => {
            handle_attempt_failed(&pool).await?;
            Err(APIError::Unauthorized)
        }
    }
}
