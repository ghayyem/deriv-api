
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_account_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::report_available_enum::ReportAvailableEnum; 
use crate::status_enum::StatusEnum; 

// It's a struct
/// This shows the information related to IDV supported services
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Idv {
    /// This is the epoch of the document expiry date.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub expiry_date: Option<DateTime<Utc>>,
    /// Show the last IDV reported reasons for the rejected cases\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_rejected: Option<Vec<String>>,
    /// Indicate if the verification report was returned by the provider\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub report_available: Option<ReportAvailableEnum>,
    /// Shows the latest document properties detected and reported by IDVS\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub reported_properties: Option<Value>,
    /// This represents the status of the latest IDV check.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<StatusEnum>,
    /// This shows the number of IDV submissions left for the client\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub submissions_left: Option<i64>,
}

