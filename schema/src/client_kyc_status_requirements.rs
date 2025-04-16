
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/landing_company/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::poi_status_enum::PoiStatusEnum; 
use crate::valid_tin_enum::ValidTinEnum; 
use crate::poa_status_enum::PoaStatusEnum; 

// It's a struct
/// [Optional] Pertains to client KYC. Returned only if the client fails to meet the requirements, including proof of identity (POI), validity of the tax identification number (TIN), and proof of address (POA).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientKycStatusRequirements {
    /// Status of proof of address (POA).\n
    // Correct serde attribute construction - Use helper
    
    pub poa_status: PoaStatusEnum,
    /// Status of proof of identity (POI).\n
    // Correct serde attribute construction - Use helper
    
    pub poi_status: PoiStatusEnum,
    /// Indicates whether the tax identification number (TIN) is valid (1) or not (0).\n
    // Correct serde attribute construction - Use helper
    
    pub valid_tin: ValidTinEnum,
}

