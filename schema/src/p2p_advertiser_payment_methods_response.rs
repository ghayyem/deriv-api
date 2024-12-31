
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_payment_methods/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// List P2P advertiser payment methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertiserPaymentMethodsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// List of current methods.
    #[serde(rename = "p2p_advertiser_payment_methods", skip_serializing_if = "Option::is_none", flatten)]
    pub p_2p_advertiser_payment_methods: HashMap<String, P2pAdvertiserPaymentMethodsvalue>,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Advertiser payment method ID, to be used for updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertiserPaymentMethodsvalue {
    /// Display name of payment method.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// Payment method fields.
    #[serde(rename = "fields", flatten)]
    pub fields: HashMap<String, Fieldsvalue>,
    /// Indicates if this method is available on adverts and orders.
    #[serde(rename = "is_enabled")]
    pub is_enabled: IsEnabledEnum,
    /// Payment method identifier.
    #[serde(rename = "method")]
    pub method: String,
    /// Payment method type.
    #[serde(rename = "type")]
    pub type: TypeEnum,
    /// IDs of adverts that use this payment method.
    #[serde(rename = "used_by_adverts", skip_serializing_if = "Option::is_none")]
    pub used_by_adverts: Option<Value>,
    /// IDs of orders that use this payment method.
    #[serde(rename = "used_by_orders", skip_serializing_if = "Option::is_none")]
    pub used_by_orders: Option<Value>,
}






