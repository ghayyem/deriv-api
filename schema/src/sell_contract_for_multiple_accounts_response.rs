
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/sell_contract_for_multiple_accounts/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Confirmation of the sale status for the selected contracts and accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct SellContractForMultipleAccountsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Status information for each affected account.
    #[serde(rename = "sell_contract_for_multiple_accounts", skip_serializing_if = "Option::is_none")]
    pub sell_contract_for_multiple_accounts: SellContractForMultipleAccounts,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Status information for each affected account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct SellContractForMultipleAccounts {
    /// The result of sell for multiple accounts request.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Vec<Value>,
}






