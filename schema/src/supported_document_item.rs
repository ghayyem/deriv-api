
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/kyc_auth_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::additional::Additional; 
use crate::available_service_item::AvailableServiceItem; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SupportedDocumentItem {
    /// Additional information required by the document type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub additional: Option<Additional>,
    /// Services that support this document type and the specific name expected by the service.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub available_services: Option<Vec<AvailableServiceItem>>,
    /// Display name of the document type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub display_name: Option<String>,
    /// The document type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub document_type: Option<String>,
    /// Example format.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub example_format: Option<String>,
    /// Document format.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub format: Option<String>,
    /// A flag to indicate if the document has a visual sample.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub has_visual_sample: Option<bool>,
    /// Flag to indicate if the document expires or not. Documents that expire have an expiration date\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub lifetime_valid: Option<bool>,
    /// Document sides.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sides: Option<Vec<String>>,
}

