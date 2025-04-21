
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/ticks_history/receive.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::msg_type::MsgType;
use crate::subscription::Subscription;
use crate::history::History;

/// Historic tick data for a single symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TicksHistoryResponse {
    /// Array of OHLC (open/high/low/close) price values for the given time (only for style=`candles`)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub candles: Option<String>,
    /// Echo of the request made.\n
    // Correct serde attribute construction - Use helper
    
    pub echo_req: Value,
    /// Historic tick data for a given symbol. Note: this will always return the latest possible set of ticks with accordance to the parameters specified.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub history: Option<History>,
    /// Type of the response according to the `style` sent in request. Would be `history` or `candles` for the first response, and `tick` or `ohlc` for the rest when subscribed.\n
    // Correct serde attribute construction - Use helper
    
    pub msg_type: MsgType,
    /// Field 'ohlc' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub ohlc: Option<Value>,
    /// Indicates the number of decimal points that the returned amounts must be displayed with\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub pip_size: Option<String>,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// For subscription requests only.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub subscription: Option<Subscription>,
}

