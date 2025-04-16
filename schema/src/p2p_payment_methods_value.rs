
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_payment_methods/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::fields_value::FieldsValue; 
use crate::type_enum::TypeEnum; 

// It's a struct
/// Payment method identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pPaymentMethodsValue {
    /// Display name of payment method.\n
    // Correct serde attribute construction - Use helper
    
    pub display_name: String,
    /// Payment method field definitions.\n
    // Correct serde attribute construction - Use helper
    
    pub fields: HashMap<String, FieldsValue>,
    /// Payment method type.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type")] 
    pub r#type: TypeEnum,
}

