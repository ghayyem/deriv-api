
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_dispute/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Dispute a P2P order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderDisputeRequest {
    /// The predefined dispute reason
    #[serde(rename = "dispute_reason")]
    pub dispute_reason: DisputeReasonEnum,
    /// The unique identifier for this order.
    #[serde(rename = "id")]
    pub id: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be 1
    #[serde(rename = "p2p_order_dispute")]
    pub p_2p_order_dispute: P2pOrderDisputeEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




/// The predefined dispute reason
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisputeReasonEnum {
    Seller_Not_Released,
    Buyer_Underpaid,
    Buyer_Overpaid,
    Buyer_Not_Paid,
    Buyer_Third_Party_Payment_Method,
}

impl DisputeReasonEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Seller_Not_Released => "seller_not_released",
            Self::Buyer_Underpaid => "buyer_underpaid",
            Self::Buyer_Overpaid => "buyer_overpaid",
            Self::Buyer_Not_Paid => "buyer_not_paid",
            Self::Buyer_Third_Party_Payment_Method => "buyer_third_party_payment_method",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "seller_not_released" => Some(Self::Seller_Not_Released),
            "buyer_underpaid" => Some(Self::Buyer_Underpaid),
            "buyer_overpaid" => Some(Self::Buyer_Overpaid),
            "buyer_not_paid" => Some(Self::Buyer_Not_Paid),
            "buyer_third_party_payment_method" => Some(Self::Buyer_Third_Party_Payment_Method),
            _ => None,
        }
    }
}
/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pOrderDisputeEnum {
    Value1 = 1,
}

impl P2pOrderDisputeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
