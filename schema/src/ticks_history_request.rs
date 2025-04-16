
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/ticks_history/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::style_enum::StyleEnum;
use crate::adjust_start_time_enum::AdjustStartTimeEnum;
use crate::granularity_enum::GranularityEnum;
use crate::subscribe_enum::SubscribeEnum;

/// Get historic tick data for a given symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TicksHistoryRequest {
    /// [Optional] 1 - if the market is closed at the end time, or license limit is before end time, adjust interval backwards to compensate.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub adjust_start_time: Option<AdjustStartTimeEnum>,
    /// [Optional] An upper limit on ticks to receive.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub count: Option<i64>,
    /// Epoch value representing the latest boundary of the returned ticks. If `latest` is specified, this will be the latest available timestamp.\n
    // Correct serde attribute construction - Use helper
    
    pub end: f64,
    /// [Optional] Only applicable for style: `candles`. Candle time-dimension width setting. (default: `60`).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub granularity: Option<GranularityEnum>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] Epoch value representing the earliest boundary of the returned ticks.\n/// - For `"style": "ticks"`: this will default to 1 day ago.\n/// - For `"style": "candles"`: it will default to 1 day ago if count or granularity is undefined.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub start: Option<i64>,
    /// [Optional] The tick-output style.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub style: Option<StyleEnum>,
    /// [Optional] 1 - to send updates whenever a new tick is received.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub subscribe: Option<SubscribeEnum>,
    /// Short symbol name (obtained from the `active_symbols` call).\n
    // Correct serde attribute construction - Use helper
    
    pub ticks_history: String,
}

