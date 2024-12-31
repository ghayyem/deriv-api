
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/identity_verification_document_add/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Adds document information such as issuing country, id and type for identity verification processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct IdentityVerificationDocumentAddRequest {
    /// [Optional] Additional info required by some document types.
    #[serde(rename = "document_additional", skip_serializing_if = "Option::is_none")]
    pub document_additional: String,
    /// The identification number of the document.
    #[serde(rename = "document_number")]
    pub document_number: String,
    /// The type of the document based on provided `issuing_country` (can obtained from `residence_list` call).
    #[serde(rename = "document_type")]
    pub document_type: String,
    /// Must be `1`
    #[serde(rename = "identity_verification_document_add")]
    pub identity_verification_document_add: IdentityVerificationDocumentAddEnum,
    /// 2-letter country code (can obtained from `residence_list` call).
    #[serde(rename = "issuing_country")]
    pub issuing_country: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityVerificationDocumentAddEnum {
    Value1 = 1,
}

impl IdentityVerificationDocumentAddEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
