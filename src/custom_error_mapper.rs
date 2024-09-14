use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::Error as SqlxError;

pub enum AppError {
    SqlxError(SqlxError),
    Unauthorized,
    // Forbidden,
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        AppError::SqlxError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::SqlxError(err) => {
                // For database errors, return a 500 Internal Server Error
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("DB error: {}", err.to_string()),
                )
                    .into_response()
            }
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Wrong Master Password").into_response()
            }
            // AppError::Forbidden => (StatusCode::FORBIDDEN, "Wrong access key").into_response(),
        }
    }
}
