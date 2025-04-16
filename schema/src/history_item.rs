
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_account_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 


use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::status_enum::StatusEnum; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HistoryItem {
    /// 2-letter country code used to request the attempt.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub country_code: Option<String>,
    /// The document type of the attempt.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub document_type: Option<String>,
    /// The id of the attempt.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub id: Option<String>,
    /// The service used to make the verification.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub service: Option<String>,
    /// Status of the attempt.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<StatusEnum>,
    /// The epoch of the attempt.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub timestamp: Option<DateTime<Utc>>,
}

