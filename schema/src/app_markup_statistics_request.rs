
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_markup_statistics/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::app_markup_statistics_enum::AppMarkupStatisticsEnum;

/// Retrieve statistics of `app_markup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppMarkupStatisticsRequest {
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub app_markup_statistics: AppMarkupStatisticsEnum,
    /// Start date (epoch or YYYY-MM-DD HH:MM:SS). Results are inclusive of this time.\n
    // Correct serde attribute construction - Use helper
    
    pub date_from: String,
    /// End date (epoch or YYYY-MM-DD HH::MM::SS). Results are inclusive of this time.\n
    // Correct serde attribute construction - Use helper
    
    pub date_to: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

