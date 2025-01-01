
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_config/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// All config related settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct WebsiteConfigResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// For subscription requests only.
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Subscription,
    /// Server status and other information regarding general settings
    #[serde(rename = "website_config", skip_serializing_if = "Option::is_none")]
    pub website_config: WebsiteConfig,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Server status and other information regarding general settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct WebsiteConfig {
    /// Available currencies and their information
    #[serde(rename = "currencies_config", flatten)]
    pub currencies_config: HashMap<String, CurrenciesConfigvalue>,
    /// Feature flags related to the website/server for various features and options: 
///  - 'signup_with_optional_email_verification': Allow signup with optional email verification.
    #[serde(rename = "feature_flags", skip_serializing_if = "Option::is_none")]
    pub feature_flags: Vec<String>,
    /// Payments Agents system settings.
    #[serde(rename = "payment_agents", skip_serializing_if = "Option::is_none")]
    pub payment_agents: PaymentAgents,
    /// Provides codes for languages supported.
    #[serde(rename = "supported_languages", skip_serializing_if = "Option::is_none")]
    pub supported_languages: Vec<String>,
    /// Latest terms and conditions version.
    #[serde(rename = "terms_conditions_version", skip_serializing_if = "Option::is_none")]
    pub terms_conditions_version: String,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Payments Agents system settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentAgents {
    /// Initial deposit requirement per country.
    #[serde(rename = "initial_deposit_per_country", flatten)]
    pub initial_deposit_per_country: HashMap<String, f64>,
}








