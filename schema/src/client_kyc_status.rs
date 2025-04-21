
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_login_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::poa_status::PoaStatus; 
use crate::valid_tin::ValidTin; 
use crate::poi_status::PoiStatus; 

// It's a struct
/// [Optional] Pertains to client KYC. Returned only if the client fails to meet the requirements, including proof of identity (POI), validity of the tax identification number (TIN), and proof of address (POA).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientKycStatus {
    /// Status of proof of address (POA).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub poa_status: Option<PoaStatus>,
    /// Status of proof of identity (POI).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub poi_status: Option<PoiStatus>,
    /// Indicates whether the tax identification number (TIN) is valid (1) or not (0).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub valid_tin: Option<ValidTin>,
}

