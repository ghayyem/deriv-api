
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/landing_company_details/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::after_first_deposit::AfterFirstDeposit; 
use crate::compliance::Compliance; 

// It's a struct
/// Legal requirements for the given Landing Company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Requirements {
    /// After first deposit requirements\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub after_first_deposit: Option<AfterFirstDeposit>,
    /// Compliance requirements\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub compliance: Option<Compliance>,
    /// Field 'partner' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub partner: Option<Value>,
    /// Field 'signup' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub signup: Option<Value>,
    /// Field 'withdrawal' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub withdrawal: Option<Value>,
}

