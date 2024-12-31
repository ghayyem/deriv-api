
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/buy_contract_for_multiple_accounts/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with transaction results is received
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct BuyContractForMultipleAccountsResponse {
    /// Receipt confirmation for the purchase
    #[serde(rename = "buy_contract_for_multiple_accounts", skip_serializing_if = "Option::is_none")]
    pub buy_contract_for_multiple_accounts: BuyContractForMultipleAccounts,
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



/// Receipt confirmation for the purchase
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct BuyContractForMultipleAccounts {
    /// List of results containing transactions and/or errors for the bought contracts.
    #[serde(rename = "result")]
    pub result: Vec<Value>,
}






