
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_account_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::is_country_supported_enum::IsCountrySupportedEnum; 
use crate::status_enum::StatusEnum; 

// It's a struct
/// This shows the information related to Onfido supported services
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Onfido {
    /// 3 letter country code for Onfide SDK\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub country_code: Option<String>,
    /// This shows the list of documents types supported by Onfido\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub documents: Option<Vec<String>>,
    /// This shows the list of documents types supported.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub documents_supported: Option<Vec<String>>,
    /// This shows the information if the country is supported by Onfido\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_country_supported: Option<IsCountrySupportedEnum>,
    /// Show the last Onfido reported reasons for the rejected cases\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_rejected: Option<Vec<String>>,
    /// Shows the latest document properties detected and reported by Onfido\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub reported_properties: Option<Value>,
    /// This represents the status of the latest Onfido check.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<StatusEnum>,
    /// This shows the number of Onfido submissions left for the client\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub submissions_left: Option<i64>,
}

