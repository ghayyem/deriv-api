
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_details/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::supported_payment_methods_item::SupportedPaymentMethodsItem; 
use crate::urls_item::UrlsItem; 
use crate::phone_numbers_item::PhoneNumbersItem; 
use crate::can_apply_enum::CanApplyEnum; 
use crate::newly_authorized_enum::NewlyAuthorizedEnum; 
use crate::code_of_conduct_approval_enum::CodeOfConductApprovalEnum; 

// It's a struct
/// The payment agent details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentagentDetails {
    /// Client's My Affiliate id, if exists.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub affiliate_id: Option<String>,
    /// If 1, the client may apply using paymentagent_create.\n
    // Correct serde attribute construction - Use helper
    
    pub can_apply: CanApplyEnum,
    /// Indicates client's agreement with the Code of Conduct document.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub code_of_conduct_approval: Option<CodeOfConductApprovalEnum>,
    /// Commission (%) the agent want to take on deposits\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub commission_deposit: Option<f64>,
    /// Commission (%) the agent want to take on withdrawals\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub commission_withdrawal: Option<f64>,
    /// Currency supported by the payment agent. It's usually the same as agent's Deriv account currency.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency_code: Option<String>,
    /// Contains a list of error codes that would prevent a successful payment agent application.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub eligibilty_validation: Option<Vec<String>>,
    /// Payment agent's email address.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub email: Option<String>,
    /// Information about payment agent and their proposed service.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub information: Option<String>,
    /// Maximum amount allowed for withdrawals\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub max_withdrawal: Option<f64>,
    /// Minimum amount allowed for withdrawals\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_withdrawal: Option<f64>,
    /// Indicates if the payment agent was recently approved with no transactions yet.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub newly_authorized: Option<NewlyAuthorizedEnum>,
    /// The name with which the payment agent is going to be identified.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_agent_name: Option<String>,
    /// Payment agent's phone number(s) with country code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub phone_numbers: Option<Vec<PhoneNumbersItem>>,
    /// Indicates the status of the Payment Agent.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<Value>,
    /// A list of supported payment methods.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub supported_payment_methods: Option<Vec<SupportedPaymentMethodsItem>>,
    /// Client's target country.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub target_country: Option<String>,
    /// The URL(s) of payment agent's website(s).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub urls: Option<Vec<UrlsItem>>,
}

