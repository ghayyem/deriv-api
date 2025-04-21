
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_list/receive.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::app_list_item::AppListItem;

/// A message with created applications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppListResponse {
    /// List of created applications for the authorized account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub app_list: Option<Vec<AppListItem>>,
    /// Echo of the request made.\n
    // Correct serde attribute construction - Use helper
    
    pub echo_req: Value,
    /// Action name of the request made.\n
    // Correct serde attribute construction - Use helper
    
    pub msg_type: String,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

