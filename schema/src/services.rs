
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_account_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::onfido::Onfido; 
use crate::manual::Manual; 
use crate::idv::Idv; 

// It's a struct
/// This shows the information about the authentication services implemented
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Services {
    /// This shows the information related to IDV supported services\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub idv: Option<Idv>,
    /// This shows the information related to the manual POI checks\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub manual: Option<Manual>,
    /// This shows the information related to Onfido supported services\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub onfido: Option<Onfido>,
}

