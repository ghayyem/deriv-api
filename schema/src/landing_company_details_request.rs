
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/landing_company_details/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::landing_company_details::LandingCompanyDetails;

/// The company has a number of licensed subsidiaries in various jurisdictions, which are called Landing Companies (and which are wholly owned subsidiaries of the Deriv Group). This call provides information about each Landing Company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LandingCompanyDetailsRequest {
    /// [Optional] Will return an extra field `tin_not_mandatory` indicating if the landing company does not require tax identification number for the provided country.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub country: Option<String>,
    /// Landing company shortcode.\n
    // Correct serde attribute construction - Use helper
    
    pub landing_company_details: LandingCompanyDetails,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

