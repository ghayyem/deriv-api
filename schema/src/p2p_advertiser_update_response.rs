
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_update/receive.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::p2p_advertiser_update::P2pAdvertiserUpdate;

/// Returns latest information of the advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pAdvertiserUpdateResponse {
    /// Echo of the request made.\n
    // Correct serde attribute construction - Use helper
    
    pub echo_req: Value,
    /// Action name of the request made.\n
    // Correct serde attribute construction - Use helper
    
    pub msg_type: String,
    /// P2P advertiser information.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub p2p_advertiser_update: Option<P2pAdvertiserUpdate>,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

