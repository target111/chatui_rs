use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("API request failed: {0}")]
    RequestFailed(#[from] ureq::Error),
    #[error("Failed to parse API response: {0}")]
    ResponseParseError(#[from] serde_json::Error),
    #[error("Configuration error: {0}")]
    ConfigError(#[from] derive_builder::UninitializedFieldError),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("API error: {0}")]
    APIError(#[from] APIError),
    #[error("Unexpected response: {0}")]
    UnexpectedResponse(String),
}

#[derive(Error, Debug, Deserialize)]
#[error("API error: {message} (type: {error_type})")]
pub struct APIError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub code: Option<String>,
    pub param: Option<String>,
}
