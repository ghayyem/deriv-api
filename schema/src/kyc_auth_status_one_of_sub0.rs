
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/kyc_auth_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::identity::Identity; 
use crate::address::Address; 
use crate::risk_classification::RiskClassification; 

// It's a struct
/// Proof of Identity (POI) and Proof of Address (POA) authentication status details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KycAuthStatusOneOfSub0 {
    /// POA authentication status details.\n
    // Correct serde attribute construction - Use helper
    
    pub address: Address,
    /// POI authentication status details.\n
    // Correct serde attribute construction - Use helper
    
    pub identity: Identity,
    /// Risk classification of the client.\n
    // Correct serde attribute construction - Use helper
    
    pub risk_classification: RiskClassification,
}

