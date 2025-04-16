
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/crypto_estimations/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::withdrawal_fee::WithdrawalFee; 

// It's a struct
/// Cryptocurrency code
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CryptoEstimationsValue {
    /// Estimated fee for crypto withdrawal calculated based on the current network conditions.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub withdrawal_fee: Option<WithdrawalFee>,
}

