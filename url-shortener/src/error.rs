use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database errpr: {0}")]
    Database(#[from] sqlx::Error),

    #[error("URL not found")]
    UrLNotFound,

    #[error("Invalid URL format")]
    InvalidUrl,

    #[error("Short code already exists")]
    CodeAlreadyExists,

    #[error("URL has expired")]
    UrlExpired,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server errror")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database erorr"),
            AppError::UrLNotFound => (StatusCode::NOT_FOUND, "URL not found"),
            AppError::InvalidUrl => (StatusCode::BAD_REQUEST, "Invalid URL format"),
            AppError::CodeAlreadyExists => (StatusCode::CONFLICT, "Short code already exists"),
            AppError::UrlExpired => (StatusCode::GONE, "Url has expired"),
            AppError::Validation(ref message) => (StatusCode::BAD_REQUEST, message.as_str()),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": message,
            "details": self.to_string(),
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
