
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contracts_for/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;



// Import shared types from the *same* crate
use crate::forward_starting_option_item::ForwardStartingOptionItem; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AvailableItem {
    /// Array of available barriers for a predefined trading period\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub available_barriers: Option<Vec<Value>>,
    /// Barrier Details.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier: Option<Value>,
    /// Category of barrier.\n
    // Correct serde attribute construction - Use helper
    
    pub barrier_category: String,
    /// [Only for Vanilla] Barrier Choices\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier_choices: Option<Vec<Value>>,
    /// Number of barriers.\n
    // Correct serde attribute construction - Use helper
    
    pub barriers: f64,
    /// Cancellation range\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub cancellation_range: Option<Vec<Value>>,
    /// Category of contract.\n
    // Correct serde attribute construction - Use helper
    
    pub contract_category: String,
    /// Category of the contract.\n
    // Correct serde attribute construction - Use helper
    
    pub contract_category_display: String,
    /// Display name for the type of contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub contract_display: Option<String>,
    /// Type of contract.\n
    // Correct serde attribute construction - Use helper
    
    pub contract_type: String,
    /// Default stake for the contract\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub default_stake: Option<f64>,
    /// [Only for Turbos] Its selected payout per point\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub display_number_of_contracts: Option<f64>,
    /// [Only for Snowball] Available contract durations in seconds.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub duration_choices: Option<Vec<i64>>,
    /// Name of exchange\n
    // Correct serde attribute construction - Use helper
    
    pub exchange_name: String,
    /// Array of barriers already expired\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub expired_barriers: Option<Vec<Value>>,
    /// Expiry Type.\n
    // Correct serde attribute construction - Use helper
    
    pub expiry_type: String,
    /// Array of returned forward starting options\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub forward_starting_options: Option<Vec<ForwardStartingOptionItem>>,
    /// Growth rate range.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub growth_rate_range: Option<Vec<Value>>,
    /// High barrier Details.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub high_barrier: Option<Value>,
    /// Last digit range\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_digit_range: Option<Vec<Value>>,
    /// Low barrier Details.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub low_barrier: Option<Value>,
    /// Type of market.\n
    // Correct serde attribute construction - Use helper
    
    pub market: String,
    /// Maximum contract duration\n
    // Correct serde attribute construction - Use helper
    
    pub max_contract_duration: String,
    /// [Only for turbos options] Maximum contract stake\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub max_stake: Option<Value>,
    /// Minimum contract duration.\n
    // Correct serde attribute construction - Use helper
    
    pub min_contract_duration: String,
    /// [Only for turbos options] Minimum contract stake\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_stake: Option<Value>,
    /// Multiplier range.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub multiplier_range: Option<Vec<Value>>,
    /// [Only for Turbos] Payout Choices\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payout_choices: Option<Vec<Value>>,
    /// Maximum payout.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payout_limit: Option<f64>,
    /// Type of sentiment.\n
    // Correct serde attribute construction - Use helper
    
    pub sentiment: String,
    /// Start Type.\n
    // Correct serde attribute construction - Use helper
    
    pub start_type: String,
    /// Type of submarket.\n
    // Correct serde attribute construction - Use helper
    
    pub submarket: String,
    /// [Only for Snowball] Available risk profile options.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trade_risk_profile_choices: Option<Vec<String>>,
    /// A hash of predefined trading period\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trading_period: Option<Value>,
    /// Symbol code\n
    // Correct serde attribute construction - Use helper
    
    pub underlying_symbol: String,
}

