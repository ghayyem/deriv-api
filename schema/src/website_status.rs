
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::p2p_config::P2pConfig; 
use crate::site_status_enum::SiteStatusEnum; 
use crate::dxtrade_status::DxtradeStatus; 
use crate::mt5_status::Mt5Status; 
use crate::payment_agents::PaymentAgents; 
use crate::currencies_config_value::CurrenciesConfigValue; 
use crate::api_call_limits::ApiCallLimits; 

// It's a struct
/// Server status and other information regarding general settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebsiteStatus {
    /// Maximum number of API calls during specified period of time.\n
    // Correct serde attribute construction - Use helper
    
    pub api_call_limits: ApiCallLimits,
    /// List of all available broker codes.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub broker_codes: Option<Vec<String>>,
    /// Country code of connected IP\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub clients_country: Option<String>,
    /// Available currencies and their information\n
    // Correct serde attribute construction - Use helper
    
    pub currencies_config: HashMap<String, CurrenciesConfigValue>,
    /// Suspension status of Dxtrade/DerivX API calls\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub dxtrade_status: Option<DxtradeStatus>,
    /// Text for site status banner, contains problem description. shown only if set by the system.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub message: Option<String>,
    /// Suspension status of MT5 API calls\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub mt5_status: Option<Mt5Status>,
    /// Peer-to-peer payment system settings.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub p2p_config: Option<P2pConfig>,
    /// Payments Agents system settings.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_agents: Option<PaymentAgents>,
    /// The current status of the website.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub site_status: Option<SiteStatusEnum>,
    /// Provides codes for languages supported.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub supported_languages: Option<Vec<String>>,
    /// Latest terms and conditions version.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub terms_conditions_version: Option<String>,
}

