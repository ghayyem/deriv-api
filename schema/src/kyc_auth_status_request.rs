
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/kyc_auth_status/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::landing_companies_item_enum::LandingCompaniesItemEnum;
use crate::kyc_auth_status_enum::KycAuthStatusEnum;

/// Get KYC Authentication Status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KycAuthStatusRequest {
    /// The country for which service availability is being verified, 2-letter country code\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub country: Option<String>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub kyc_auth_status: KycAuthStatusEnum,
    /// Indicates which landing companies to get the KYC authentication status for.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub landing_companies: Option<Vec<LandingCompaniesItemEnum>>,
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
}

