use axum::{extract::State, response::IntoResponse, Json};

use crate::{custom_error_mapper::AppError, db_access::table_system::get_system_counters};

pub async fn get_counters(State(pool): State<sqlx::PgPool>) -> Result<impl IntoResponse, AppError> {
    let data = get_system_counters(pool).await?;

    Ok(Json(data))
}
