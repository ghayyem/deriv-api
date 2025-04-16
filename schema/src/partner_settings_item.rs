
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/partner_accounts/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::provider_enum::ProviderEnum; 
use crate::partner_type_enum::PartnerTypeEnum; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PartnerSettingsItem {
    /// The user id.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub binary_user_id: Option<i64>,
    /// [Optional] Company name. Only applicable for partners of type company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub company_name: Option<String>,
    /// [Optional] Company registration number. Only applicable for partners of type company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub company_registration_no: Option<String>,
    /// Partner's login ID\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub partner_loginid: Option<String>,
    /// Defines whether this partner is an individual or a company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub partner_type: Option<PartnerTypeEnum>,
    /// Defines the provider platform.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub provider: Option<ProviderEnum>,
    /// Partner's Website URI/Promotional Platform\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub website: Option<String>,
}

