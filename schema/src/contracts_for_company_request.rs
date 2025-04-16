
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contracts_for_company/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::landing_company_enum::LandingCompanyEnum;
use crate::contracts_for_company_enum::ContractsForCompanyEnum;

/// Get the list of currently available contracts for a given landing company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContractsForCompanyRequest {
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub contracts_for_company: ContractsForCompanyEnum,
    /// [Optional] Indicates which landing company to get a list of contracts for. If you are logged in, your account's landing company will override this field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub landing_company: Option<LandingCompanyEnum>,
    /// [Optional] The login id of the user. If left unspecified, it defaults to the initial authorized token's login id.\n
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

