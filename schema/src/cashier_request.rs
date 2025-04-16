
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/cashier/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::provider_enum::ProviderEnum;
use crate::cashier_enum::CashierEnum;
use crate::dry_run_enum::DryRunEnum;
use crate::type_enum::TypeEnum;

/// Request the cashier info for the specified type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CashierRequest {
    /// [Optional] Address for crypto withdrawal. Only applicable for `api` type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address: Option<String>,
    /// [Optional] Amount for crypto withdrawal. Only applicable for `api` type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub amount: Option<f64>,
    /// Operation which needs to be requested from cashier\n
    // Correct serde attribute construction - Use helper
    
    pub cashier: CashierEnum,
    /// [Optional] If set to `1`, only validation is performed. Only applicable for `withdraw` using `crypto` provider and `api` type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub dry_run: Option<DryRunEnum>,
    /// [Optional] The `unique_id` of the estimated fee received from `crypto_estimations` call in case the client is willing to pay the returned fee in order to prioritise their withdrawal request.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub estimated_fee_unique_id: Option<String>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Cashier provider.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub provider: Option<ProviderEnum>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] Data is returned from the cashier. The `crypto` provider only supports `api` (not `url`) for crypto accounts.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")] 
    pub r#type: Option<TypeEnum>,
    /// [Optional] Email verification code (received from a `verify_email` call, which must be done first)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub verification_code: Option<String>,
}

