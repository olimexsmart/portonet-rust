use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::Error as SqlxError;

pub enum AppError {
    SqlxError(SqlxError),
    Unauthorized,
    NotFound,
    Expired,
    Revoked,
    Locked,
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
            AppError::NotFound => (StatusCode::NOT_FOUND, "Wrong access key").into_response(),
            AppError::Expired => (StatusCode::REQUEST_TIMEOUT, "Key Expired").into_response(),
            AppError::Revoked => (StatusCode::GONE, "Key Revoked").into_response(),
            AppError::Locked => (StatusCode::LOCKED, "Too many failed attempts").into_response(),
        }
    }
}
