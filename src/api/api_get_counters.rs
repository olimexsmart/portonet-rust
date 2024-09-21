use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    custom_error_mapper::APIError,
    db_access::{table_logs::insert_log, table_system::get_system_counters},
};

use ::function_name::named;

#[named]
pub async fn get_counters(State(pool): State<sqlx::PgPool>) -> Result<impl IntoResponse, APIError> {
    insert_log(&pool, function_name!(), None).await?;

    let data = get_system_counters(&pool).await?;

    Ok(Json(data))
}
