
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/document_upload/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Receive details of uploaded authentication documents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct DocumentUploadResponse {
    /// Details of the uploaded documents.
    #[serde(rename = "document_upload", skip_serializing_if = "Option::is_none")]
    pub document_upload: DocumentUpload,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Details of the uploaded documents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct DocumentUpload {
    /// Current call type, add this to your binary payload metadata
    #[serde(rename = "call_type")]
    pub call_type: f64,
    /// Hex encoded SHA-1 checksum of the file
    #[serde(rename = "checksum", skip_serializing_if = "Option::is_none")]
    pub checksum: String,
    /// 2-letter country code
    #[serde(rename = "document_issuing_country", skip_serializing_if = "Option::is_none")]
    pub document_issuing_country: String,
    /// File size
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: f64,
    /// Upload status (`success` or `failure`)
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: String,
    /// Current upload ID, add this to your binary payload metadata
    #[serde(rename = "upload_id")]
    pub upload_id: f64,
}






