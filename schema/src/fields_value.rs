
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_create/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::type_enum::TypeEnum; 

// It's a struct
/// Field identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FieldsValue {
    /// Display name of payment method field.\n
    // Correct serde attribute construction - Use helper
    
    pub display_name: String,
    /// Is field required or optional.\n
    // Correct serde attribute construction - Use helper
    
    pub required: i64,
    /// Field type.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type")] 
    pub r#type: TypeEnum,
    /// Current value of payment method field.\n
    // Correct serde attribute construction - Use helper
    
    pub value: f64,
}

