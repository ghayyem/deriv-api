
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/partner_settings_update/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::provider::Provider;
use crate::partner_type::PartnerType;

/// A message with Partner Settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PartnerSettingsUpdateRequest {
    /// [Optional] Company name. Only applicable for partners of type company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub company_name: Option<String>,
    /// [Optional] Company registration number. Only applicable for partners of type company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub company_registration_no: Option<String>,
    /// [Optional] The login id of the partner account. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub partner_settings_update: i64,
    /// Defines whether this partner is an individual or a company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub partner_type: Option<PartnerType>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// Name of the provider platform.\n
    // Correct serde attribute construction - Use helper
    
    pub provider: Provider,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// Partner's Website URI/Promotional Platform\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub website: Option<String>,
}

