
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::Error as SqlxError;

pub enum APIError {
    SqlxError(SqlxError),
    Unauthorized,
    NotFound,
    Expired,
    Revoked,
    Locked,
    HomeAssistantError,
    EnvError(String),
}

impl From<SqlxError> for APIError {
    fn from(err: SqlxError) -> Self {
        APIError::SqlxError(err)
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        match self {
            APIError::SqlxError(err) => {
                // For database errors, return a 500 Internal Server Error
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("DB error: {}", err.to_string()),
                )
                    .into_response()
            }
            APIError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Wrong Master Password").into_response()
            }
            APIError::NotFound => (StatusCode::NOT_FOUND, "Wrong access key").into_response(),
            APIError::Expired => (StatusCode::REQUEST_TIMEOUT, "Key Expired").into_response(),
            APIError::Revoked => (StatusCode::GONE, "Key Revoked").into_response(),
            APIError::Locked => (StatusCode::LOCKED, "Too many failed attempts").into_response(),
            APIError::HomeAssistantError => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Failed request to Home Assistant",
            )
                .into_response(),
            APIError::EnvError(var_name) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Missing ENV variable {}", var_name),
            )
                .into_response(),
        }
    }
}
