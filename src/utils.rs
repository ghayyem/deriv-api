use crate::error::{DerivError, Result};
use url::Url;

pub(crate) fn validate_schema(endpoint: &Url) -> Result<()> {
    match endpoint.scheme() {
        "ws" | "wss" => Ok(()),
        _ => Err(DerivError::InvalidSchema(endpoint.scheme().to_string())),
    }
}

pub(crate) fn validate_app_id(app_id: i32) -> Result<()> {
    if app_id < 1 {
        return Err(DerivError::InvalidAppId(app_id));
    }
    Ok(())
}

pub(crate) fn validate_language(lang: &str) -> Result<()> {
    if lang.len() != 2 {
        return Err(DerivError::InvalidLanguage(lang.to_string()));
    }
    Ok(())
}
