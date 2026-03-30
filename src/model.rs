use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// =========================
// Request DTOs
// =========================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MintNftRequest {
    pub owner_address: String,
    pub token_name: String,
    pub token_uri: String,
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenFileForm {
    pub file: Vec<u8>,
}

// =========================
// Response DTOs
// =========================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NftMetadata {
    pub token_id: String,
    pub owner_address: String,
    pub token_name: String,
    pub token_uri: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UploadResponse {
    pub token_uri: String,
}
