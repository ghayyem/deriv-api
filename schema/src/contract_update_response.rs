
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contract_update/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Contract update status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractUpdateResponse {
    /// Contains the update status of the request
    #[serde(rename = "contract_update", skip_serializing_if = "Option::is_none")]
    pub contract_update: ContractUpdate,
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
use std::collections::HashMap;


/// Contains the update status of the request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractUpdate {
    /// The target spot price where the contract will be closed automatically at the loss specified by the user.
    #[serde(rename = "stop_loss", skip_serializing_if = "Option::is_none")]
    pub stop_loss: StopLoss,
    /// The target spot price where the contract will be closed automatically at the profit specified by the user.
    #[serde(rename = "take_profit", skip_serializing_if = "Option::is_none")]
    pub take_profit: TakeProfit,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

/// The target spot price where the contract will be closed automatically at the loss specified by the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct StopLoss {
    /// Localized display name
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// Stop loss amount
    #[serde(rename = "order_amount", skip_serializing_if = "Option::is_none")]
    pub order_amount: Option<Value>,
    /// Stop loss order epoch
    #[serde(rename = "order_date", skip_serializing_if = "Option::is_none")]
    pub order_date: i64,
    /// Stop loss pip-sized barrier value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

/// The target spot price where the contract will be closed automatically at the profit specified by the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TakeProfit {
    /// Localized display name
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// Take profit amount
    #[serde(rename = "order_amount", skip_serializing_if = "Option::is_none")]
    pub order_amount: Option<Value>,
    /// Take profit order epoch
    #[serde(rename = "order_date", skip_serializing_if = "Option::is_none")]
    pub order_date: i64,
    /// Take profit pip-sized barrier value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}








