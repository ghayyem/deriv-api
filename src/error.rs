use thiserror::Error;

#[derive(Error, Debug)]
pub enum DerivError {
    #[error("WebSocket connection error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid schema: {0}")]
    InvalidSchema(String),

    #[error("Invalid app ID: {0}")]
    InvalidAppId(i32),

    #[error("Invalid language code: {0}")]
    InvalidLanguage(String),

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Empty subscription ID")]
    EmptySubscriptionId,

    #[error("API error: {code} - {message}")]
    ApiError {
        code: String,
        message: String,
        details: Option<serde_json::Value>,
    },

    #[error("Request timeout")]
    Timeout,
}

pub type Result<T> = std::result::Result<T, DerivError>;

// API Error response structure
#[derive(Debug, serde::Deserialize)]
pub(crate) struct ApiErrorResponse {
    pub error: ApiErrorDetails,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ApiErrorDetails {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub details: Option<serde_json::Value>,
}

impl From<ApiErrorDetails> for DerivError {
    fn from(error: ApiErrorDetails) -> Self {
        DerivError::ApiError {
            code: error.code,
            message: error.message,
            details: error.details,
        }
    }
}

pub(crate) fn parse_error(raw_response: &[u8]) -> Result<()> {
    let response: serde_json::Value = serde_json::from_slice(raw_response)?;

    if let Some(error) = response.get("error") {
        let error_details: ApiErrorDetails = serde_json::from_value(error.clone())?;
        return Err(error_details.into());
    }

    Ok(())
}
