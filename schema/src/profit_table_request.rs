
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/profit_table/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::sort_enum::SortEnum;
use crate::contract_type_item_enum::ContractTypeItemEnum;
use crate::description_enum::DescriptionEnum;

/// Retrieve a summary of account Profit Table, according to given search criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProfitTableRequest {
    /// Return only contracts of the specified types\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub contract_type: Option<Vec<ContractTypeItemEnum>>,
    /// [Optional] Start date (epoch or YYYY-MM-DD)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_from: Option<String>,
    /// [Optional] End date (epoch or YYYY-MM-DD)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_to: Option<String>,
    /// [Optional] If set to 1, will return full contracts description.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub description: Option<DescriptionEnum>,
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
    /// Field 'profit_table' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub profit_table: Value,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] Sort direction.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sort: Option<SortEnum>,
}

