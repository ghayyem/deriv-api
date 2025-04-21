
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/kyc_auth_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::last_rejected::LastRejected; 
use crate::status::Status; 
use crate::supported_document_item::SupportedDocumentItem; 
use crate::service::Service; 

// It's a struct
/// POI authentication status details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Identity {
    /// Available services for the next POI attempt.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub available_services: Option<Vec<String>>,
    /// Details on the rejected POI attempt.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_rejected: Option<LastRejected>,
    /// Service used for the current POI status.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub service: Option<Service>,
    /// Current POI status.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<Status>,
    /// Supported documents.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub supported_documents: Option<Vec<SupportedDocumentItem>>,
}

