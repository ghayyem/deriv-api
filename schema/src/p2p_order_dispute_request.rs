
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_dispute/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::dispute_reason_enum::DisputeReasonEnum;

/// Dispute a P2P order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pOrderDisputeRequest {
    /// The predefined dispute reason\n
    // Correct serde attribute construction - Use helper
    
    pub dispute_reason: DisputeReasonEnum,
    /// The unique identifier for this order.\n
    // Correct serde attribute construction - Use helper
    
    pub id: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// Field 'p2p_order_dispute' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub p2p_order_dispute: Value,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

