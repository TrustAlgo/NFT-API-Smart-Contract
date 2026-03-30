use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

// =========================
// Application Error
// =========================

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    InternalServerError,

    #[error(transparent)]
    Web3Error(#[from] web3::Error),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unexpected error: {0}")]
    GenericError(String),
}

// Convert boxed errors into GenericError
impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Self::GenericError(err.to_string())
    }
}

// Centralized status code mapping
impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Web3Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SerdeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::GenericError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn message(&self) -> String {
        match self {
            AppError::InternalServerError => "Internal server error".to_string(),
            _ => self.to_string(),
        }
    }
}

// Clean IntoResponse implementation
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        let body = Json(json!({
            "error": self.message()
        }));

        (status, body).into_response()
    }
}

// =========================
// Upload Error
// =========================

#[derive(Error, Debug)]
pub enum UploadError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": "Internal server error"
        }));

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

// =========================
// Signature Error
// =========================

#[derive(Error, Debug)]
pub enum SignatureError {
    #[error("Invalid hex format")]
    HexDecodeError(#[from] hex::FromHexError),
}

// Convert SignatureError → AppError
impl From<SignatureError> for AppError {
    fn from(_: SignatureError) -> Self {
        AppError::BadRequest("Invalid hex format".to_string())
    }
}
