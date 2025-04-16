
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/exchange_rates/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::include_spread_enum::IncludeSpreadEnum;
use crate::exchange_rates_enum::ExchangeRatesEnum;
use crate::subscribe_enum::SubscribeEnum;

/// Retrieves the exchange rate from a base currency to a target currency supported by the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExchangeRatesRequest {
    /// Base currency (can be obtained from `payout_currencies` call)\n
    // Correct serde attribute construction - Use helper
    
    pub base_currency: String,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub exchange_rates: ExchangeRatesEnum,
    /// [Optional] 1 - Request for ask and bid rates along with the spot rate. Only available if target_currency is provided.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub include_spread: Option<IncludeSpreadEnum>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] 1 - to initiate a realtime stream of exchange rates relative to base currency.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub subscribe: Option<SubscribeEnum>,
    /// [Optional] Target currency for the exchange rate. If subscribe is set, target_currency must be specified as well.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub target_currency: Option<String>,
}

