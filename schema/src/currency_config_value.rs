
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_account_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::is_deposit_suspended_enum::IsDepositSuspendedEnum; 
use crate::is_withdrawal_suspended_enum::IsWithdrawalSuspendedEnum; 

// It's a struct
/// Client currency
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CurrencyConfigValue {
    /// Deposit is allowed for currency or not\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_deposit_suspended: Option<IsDepositSuspendedEnum>,
    /// Withdrawal is allowed for currency or not\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_withdrawal_suspended: Option<IsWithdrawalSuspendedEnum>,
}

