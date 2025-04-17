
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/document_upload/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::lifetime_valid_enum::LifetimeValidEnum;
use crate::document_type_enum::DocumentTypeEnum;
use crate::page_type_enum::PageTypeEnum;
use crate::document_upload_enum::DocumentUploadEnum;
use crate::proof_of_ownership::ProofOfOwnership;
use crate::document_format_enum::DocumentFormatEnum;

/// Request KYC information from client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DocumentUploadRequest {
    /// Document file format\n
    // Correct serde attribute construction - Use helper
    
    pub document_format: DocumentFormatEnum,
    /// [Optional] Document ID (required for Passport, Proof of ID and Driver's License)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub document_id: Option<String>,
    /// 2-letter country code, mandatory for POI only\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub document_issuing_country: Option<String>,
    /// Document type\n
    // Correct serde attribute construction - Use helper
    
    pub document_type: DocumentTypeEnum,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub document_upload: DocumentUploadEnum,
    /// The checksum of the file to be uploaded\n
    // Correct serde attribute construction - Use helper
    
    pub expected_checksum: String,
    /// [Optional] Document expiration date (required for Passport, Proof of ID and Driver's License)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub expiration_date: Option<String>,
    /// Document size (should be less than 10MB)\n
    // Correct serde attribute construction - Use helper
    
    pub file_size: i64,
    /// [Optional] Boolean value that indicates whether this document is lifetime valid (only applies to POI document types, cancels out the expiration_date given if any)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub lifetime_valid: Option<LifetimeValidEnum>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] To determine document side\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub page_type: Option<PageTypeEnum>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] It contains info about the proof of ownership being uploaded (mandatory for proof_of_ownership document type)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub proof_of_ownership: Option<ProofOfOwnership>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

