
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_account_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::status_enum::StatusEnum; 

// It's a struct
/// The authentication status for document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Document {
    /// This is the epoch of the document expiry date.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub expiry_date: Option<DateTime<Utc>>,
    /// Show the last reported reasons for the rejected poa cases\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_rejected: Option<String>,
    /// This represents the current status of the proof of address document submitted for authentication.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<StatusEnum>,
    /// Field 'verified_jurisdiction' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub verified_jurisdiction: Option<Value>,
}

