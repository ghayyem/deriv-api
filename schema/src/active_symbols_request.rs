
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/active_symbols/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::contract_type_item::ContractTypeItem;
use crate::landing_company_short::LandingCompanyShort;
use crate::barrier_category_item::BarrierCategoryItem;
use crate::landing_company::LandingCompany;

/// Retrieve a list of all currently active symbols (underlying markets upon which contracts are available for trading).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActiveSymbolsRequest {
    /// Field 'active_symbols' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub active_symbols: Value,
    /// [Optional] Category of barrier.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier_category: Option<Vec<BarrierCategoryItem>>,
    /// [Optional] The proposed contract type\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub contract_type: Option<Vec<ContractTypeItem>>,
    /// Deprecated - replaced by landing_company_short.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub landing_company: Option<LandingCompany>,
    /// [Optional] If you specify this field, only symbols available for trading by that landing company will be returned. If you are logged in, only symbols available for trading by your landing company will be returned regardless of what you specify in this field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub landing_company_short: Option<LandingCompanyShort>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] If you specify this field, only symbols that can be traded through that product type will be returned.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub product_type: Option<String>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

