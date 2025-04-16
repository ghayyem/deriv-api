
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_config/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::is_deposit_suspended_enum::IsDepositSuspendedEnum; 
use crate::transfer_between_accounts::TransferBetweenAccounts; 
use crate::is_withdrawal_suspended_enum::IsWithdrawalSuspendedEnum; 
use crate::platform::Platform; 
use crate::is_suspended_enum::IsSuspendedEnum; 
use crate::type_enum::TypeEnum; 

// It's a struct
/// Currency code
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CurrenciesConfigValue {
    /// Number of fractional digits.\n
    // Correct serde attribute construction - Use helper
    
    pub fractional_digits: f64,
    /// Current status for payment deposit for the currency\n
    // Correct serde attribute construction - Use helper
    
    pub is_deposit_suspended: IsDepositSuspendedEnum,
    /// Current status for the currency\n
    // Correct serde attribute construction - Use helper
    
    pub is_suspended: IsSuspendedEnum,
    /// Current status for payment withdrawal for the currency\n
    // Correct serde attribute construction - Use helper
    
    pub is_withdrawal_suspended: IsWithdrawalSuspendedEnum,
    /// Name of the currency.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub name: Option<String>,
    /// List of cashier platforms supported for this currency. It is categorized by cashier and ramp (on-ramp, off-ramp) platforms.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub platform: Option<Platform>,
    /// Default stake value for the currency.\n
    // Correct serde attribute construction - Use helper
    
    pub stake_default: f64,
    /// Fees and range of allowed amount for transfer between accounts with different types of currencies.\n
    // Correct serde attribute construction - Use helper
    
    pub transfer_between_accounts: TransferBetweenAccounts,
    /// Type of the currency.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type")] 
    pub r#type: TypeEnum,
}

