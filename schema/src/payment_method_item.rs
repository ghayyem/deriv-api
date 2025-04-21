
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/payment_methods/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;



// Import shared types from the *same* crate

// It's a struct
/// A payment method suported for the given country
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentMethodItem {
    /// The min and max values for deposits.\n
    // Correct serde attribute construction - Use helper
    
    pub deposit_limits: String,
    /// How much time it takes for a deposit to be processed.\n
    // Correct serde attribute construction - Use helper
    
    pub deposit_time: String,
    /// Short description explaining the payment method.\n
    // Correct serde attribute construction - Use helper
    
    pub description: String,
    /// Common name for the payment method.\n
    // Correct serde attribute construction - Use helper
    
    pub display_name: String,
    /// Unique identifier for the payment method.\n
    // Correct serde attribute construction - Use helper
    
    pub id: String,
    /// Payment processor for this payment method.\n
    // Correct serde attribute construction - Use helper
    
    pub payment_processor: String,
    /// A list of predefined amounts for withdraw or deposit.\n
    // Correct serde attribute construction - Use helper
    
    pub predefined_amounts: String,
    /// Sign up link for this payment method.\n
    // Correct serde attribute construction - Use helper
    
    pub signup_link: String,
    /// Currencies supported for this payment method.\n
    // Correct serde attribute construction - Use helper
    
    pub supported_currencies: Vec<String>,
    /// Type of Payment Method.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type")] 
    pub type_: String,
    /// A printable description for type of payment method.\n
    // Correct serde attribute construction - Use helper
    
    pub type_display_name: String,
    /// Withdrawal limits per currency.\n
    // Correct serde attribute construction - Use helper
    
    pub withdraw_limits: Value,
    /// How much time takes a withdrawal to be processed.\n
    // Correct serde attribute construction - Use helper
    
    pub withdrawal_time: String,
}

