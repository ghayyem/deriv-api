
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/landing_company/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::has_reality_check_enum::HasRealityCheckEnum; 
use crate::tin_not_mandatory_enum::TinNotMandatoryEnum; 
use crate::requirements::Requirements; 
use crate::support_professional_client_enum::SupportProfessionalClientEnum; 

// It's a struct
/// Landing Company details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Standard {
    /// Landing Company address\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address: Option<Vec<String>>,
    /// Special conditions for changing sensitive fields\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub changeable_fields: Option<Value>,
    /// Landing Company country of incorporation\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub country: Option<String>,
    /// The configuration of each currency.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency_config: Option<Value>,
    /// Flag to indicate whether reality check is applicable for this Landing Company. `1`: applicable, `0`: not applicable. The Reality Check is a feature that gives a summary of the client's trades and account balances on a regular basis throughout his session, and is a regulatory requirement for certain Landing Companies.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub has_reality_check: Option<HasRealityCheckEnum>,
    /// Allowed contract types\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub legal_allowed_contract_categories: Option<Vec<String>>,
    /// Allowable currencies\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub legal_allowed_currencies: Option<Vec<String>>,
    /// Allowable markets\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub legal_allowed_markets: Option<Vec<String>>,
    /// Default account currency\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub legal_default_currency: Option<String>,
    /// Landing Company legal name\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub name: Option<String>,
    /// Legal requirements for the Landing Company\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub requirements: Option<Requirements>,
    /// Landing Company short code\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub shortcode: Option<String>,
    /// Flag that indicates whether the landing company supports professional accounts or not\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub support_professional_client: Option<SupportProfessionalClientEnum>,
    /// Flag that indicates whether tax identifier number is not mandatory for the current country and landing company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tin_not_mandatory: Option<TinNotMandatoryEnum>,
}

