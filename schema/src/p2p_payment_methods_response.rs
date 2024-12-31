
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_payment_methods/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// List all P2P payment methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pPaymentMethodsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Payment methods keyed by identifier.
    #[serde(rename = "p2p_payment_methods", skip_serializing_if = "Option::is_none", flatten)]
    pub p_2p_payment_methods: HashMap<String, P2pPaymentMethodsvalue>,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Payment method identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pPaymentMethodsvalue {
    /// Display name of payment method.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// Payment method field definitions.
    #[serde(rename = "fields", flatten)]
    pub fields: HashMap<String, Fieldsvalue>,
    /// Payment method type.
    #[serde(rename = "type")]
    pub type: TypeEnum,
}






