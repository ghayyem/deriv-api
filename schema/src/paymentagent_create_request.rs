
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_create/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::phone_numbers_item::PhoneNumbersItem;
use crate::code_of_conduct_approval_enum::CodeOfConductApprovalEnum;
use crate::supported_payment_methods_item::SupportedPaymentMethodsItem;
use crate::paymentagent_create_enum::PaymentagentCreateEnum;
use crate::urls_item::UrlsItem;

/// Saves client's payment agent details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentagentCreateRequest {
    /// [Optional] Client's My Affiliate id, if exists.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub affiliate_id: Option<String>,
    /// Indicates client's agreement with the Code of Conduct.\n
    // Correct serde attribute construction - Use helper
    
    pub code_of_conduct_approval: CodeOfConductApprovalEnum,
    /// Commission  (%) the agent wants to take on deposits\n
    // Correct serde attribute construction - Use helper
    
    pub commission_deposit: f64,
    /// Commission  (%) the agent wants to take on withdrawals\n
    // Correct serde attribute construction - Use helper
    
    pub commission_withdrawal: f64,
    /// Payment agent's email address.\n
    // Correct serde attribute construction - Use helper
    
    pub email: String,
    /// [Optional] Information about payment agent and their proposed service.\n
    // Correct serde attribute construction - Use helper
    
    pub information: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// The name with which the payment agent is going to be identified.\n
    // Correct serde attribute construction - Use helper
    
    pub payment_agent_name: String,
    /// Must be 1\n
    // Correct serde attribute construction - Use helper
    
    pub paymentagent_create: PaymentagentCreateEnum,
    /// Payment agent's phone number(s) with country code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub phone_numbers: Option<Vec<PhoneNumbersItem>>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// A list of supported payment methods.\n
    // Correct serde attribute construction - Use helper
    
    pub supported_payment_methods: Vec<SupportedPaymentMethodsItem>,
    /// The URL(s) of payment agent's website(s).\n
    // Correct serde attribute construction - Use helper
    
    pub urls: Vec<UrlsItem>,
}

