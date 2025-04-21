
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_markup_details/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::description::Description;
use crate::sort::Sort;
use crate::sort_field_item::SortFieldItem;

/// Retrieve details of `app_markup` according to criteria specified.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppMarkupDetailsRequest {
    /// [Optional] Specific application `app_id` to report on.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub app_id: Option<i64>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub app_markup_details: i64,
    /// [Optional] Specific client loginid to report on, like CR12345\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub client_loginid: Option<String>,
    /// Start date (epoch or YYYY-MM-DD HH:MM:SS). Results are inclusive of this time.\n
    // Correct serde attribute construction - Use helper
    
    pub date_from: String,
    /// End date (epoch or YYYY-MM-DD HH::MM::SS). Results are inclusive of this time.\n
    // Correct serde attribute construction - Use helper
    
    pub date_to: String,
    /// [Optional] If set to 1, will return `app_markup` transaction details.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub description: Option<Description>,
    /// [Optional] Apply upper limit to count of transactions received.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub limit: Option<f64>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Number of transactions to skip.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub offset: Option<i64>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] Sort direction on `transaction_time`. Other fields sort order is ASC.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sort: Option<Sort>,
    /// [Optional] One or more of the specified fields to sort on. Default sort field is by `transaction_time`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sort_fields: Option<Vec<SortFieldItem>>,
}

