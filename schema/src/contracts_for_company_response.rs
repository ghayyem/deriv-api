
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contracts_for_company/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get the list of currently available contracts for a given landing company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractsForCompanyResponse {
    /// List of available contracts for a given landing company.
    #[serde(rename = "contracts_for_company", skip_serializing_if = "Option::is_none")]
    pub contracts_for_company: ContractsForCompany,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// List of available contracts for a given landing company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractsForCompany {
    /// List of available contracts.
    #[serde(rename = "available")]
    pub available: Vec<Availableitem>,
    /// Count of contracts available
    #[serde(rename = "hit_count")]
    pub hit_count: f64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Availableitem {
    /// Category of contract barrier.
    #[serde(rename = "barrier_category")]
    pub barrier_category: String,
    /// Category of contract.
    #[serde(rename = "contract_category")]
    pub contract_category: String,
    /// Display name for the contract category, localized to selected language.
    #[serde(rename = "contract_category_display")]
    pub contract_category_display: String,
    /// Display name for the contract, localized to selected language.
    #[serde(rename = "contract_display")]
    pub contract_display: String,
    /// Type of contract.
    #[serde(rename = "contract_type")]
    pub contract_type: String,
    /// Default stake for the contract
    #[serde(rename = "default_stake", skip_serializing_if = "Option::is_none")]
    pub default_stake: f64,
    /// Type of sentiment.
    #[serde(rename = "sentiment")]
    pub sentiment: String,
}








