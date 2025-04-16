
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/document_upload/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Details of the uploaded documents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DocumentUpload {
    /// Current call type, add this to your binary payload metadata\n
    // Correct serde attribute construction - Use helper
    
    pub call_type: f64,
    /// Hex encoded SHA-1 checksum of the file\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub checksum: Option<String>,
    /// 2-letter country code\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub document_issuing_country: Option<String>,
    /// File size\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub size: Option<f64>,
    /// Upload status (`success` or `failure`)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<String>,
    /// Current upload ID, add this to your binary payload metadata\n
    // Correct serde attribute construction - Use helper
    
    pub upload_id: f64,
}

