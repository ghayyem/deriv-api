
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_payment_methods/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::fields_value::FieldsValue; 
use crate::is_enabled_enum::IsEnabledEnum; 
use crate::type_enum::TypeEnum; 

// It's a struct
/// Advertiser payment method ID, to be used for updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pAdvertiserPaymentMethodsValue {
    /// Display name of payment method.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub display_name: Option<String>,
    /// Payment method fields.\n
    // Correct serde attribute construction - Use helper
    
    pub fields: HashMap<String, FieldsValue>,
    /// Indicates if this method is available on adverts and orders.\n
    // Correct serde attribute construction - Use helper
    
    pub is_enabled: IsEnabledEnum,
    /// Payment method identifier.\n
    // Correct serde attribute construction - Use helper
    
    pub method: String,
    /// Payment method type.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type")] 
    pub r#type: TypeEnum,
    /// IDs of adverts that use this payment method.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub used_by_adverts: Option<Vec<String>>,
    /// IDs of orders that use this payment method.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub used_by_orders: Option<Vec<String>>,
}

