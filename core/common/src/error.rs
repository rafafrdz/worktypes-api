use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Recurso no encontrado: {0}")]
    NotFound(String),

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Error de validación: {0}")]
    Validation(String),

    #[error("Error interno del servidor: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Database(e) => {
                tracing::error!("Error de base de datos: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error interno de base de datos".to_string(),
                )
            }
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => {
                tracing::error!("Error interno: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error interno del servidor".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
