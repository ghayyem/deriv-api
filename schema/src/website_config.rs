
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_config/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::payment_agents::PaymentAgents; 
use crate::currencies_config_value::CurrenciesConfigValue; 

// It's a struct
/// Server status and other information regarding general settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebsiteConfig {
    /// Available currencies and their information\n
    // Correct serde attribute construction - Use helper
    
    pub currencies_config: HashMap<String, CurrenciesConfigValue>,
    /// Feature flags related to the website/server for various features and options:\n///  - 'signup_with_optional_email_verification': Allow signup with optional email verification.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub feature_flags: Option<Vec<String>>,
    /// Payments Agents system settings.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_agents: Option<PaymentAgents>,
    /// Provides codes for languages supported.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub supported_languages: Option<Vec<String>>,
    /// Latest terms and conditions version.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub terms_conditions_version: Option<String>,
}

