
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/landing_company/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The company has a number of licensed subsidiaries in various jurisdictions, which are called Landing Companies. This call will return the appropriate Landing Company for clients of a given country. The landing company may differ for derived contracts (Synthetic Indices) and Financial contracts (Forex, Stock Indices, Commodities).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LandingCompanyRequest {
    /// Client's 2-letter country code (obtained from `residence_list` call).
    #[serde(rename = "landing_company")]
    pub landing_company: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




