
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;



// Import shared types from the *same* crate
use crate::phone_number_item::PhoneNumberItem; 
use crate::supported_payment_method_item::SupportedPaymentMethodItem; 
use crate::url_item::UrlItem; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListItem {
    /// Currencies that are accepted by this payment agent.\n
    // Correct serde attribute construction - Use helper
    
    pub currencies: String,
    /// Commission amount applied on deposits made through this payment agent.\n
    // Correct serde attribute construction - Use helper
    
    pub deposit_commission: String,
    /// Payment agent's email address.\n
    // Correct serde attribute construction - Use helper
    
    pub email: String,
    /// More descriptions about this payment agent.\n
    // Correct serde attribute construction - Use helper
    
    pub further_information: String,
    /// Maximum withdrawal allowed for transactions through this payment agent.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub max_withdrawal: Option<Value>,
    /// Minimum withdrawal allowed for transactions through this payment agent.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_withdrawal: Option<Value>,
    /// Payment agent's name.\n
    // Correct serde attribute construction - Use helper
    
    pub name: String,
    /// Payment agent's loginid.\n
    // Correct serde attribute construction - Use helper
    
    pub paymentagent_loginid: String,
    /// Payment agent's phone number(s) with country code.\n
    // Correct serde attribute construction - Use helper
    
    pub phone_numbers: Vec<PhoneNumberItem>,
    /// A summary about payment agent.\n
    // Correct serde attribute construction - Use helper
    
    pub summary: String,
    /// A list of supported payment methods.\n
    // Correct serde attribute construction - Use helper
    
    pub supported_payment_methods: Vec<SupportedPaymentMethodItem>,
    /// The URL(s) of payment agent's website(s).\n
    // Correct serde attribute construction - Use helper
    
    pub urls: Vec<UrlItem>,
    /// Commission amount applied on withdrawals made through this payment agent.\n
    // Correct serde attribute construction - Use helper
    
    pub withdrawal_commission: String,
}

