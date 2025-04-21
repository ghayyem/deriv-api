
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/active_symbols/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::allow_forward_starting::AllowForwardStarting; 
use crate::is_trading_suspended::IsTradingSuspended; 
use crate::exchange_is_open::ExchangeIsOpen; 

// It's a struct
/// The information about each symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActiveSymbolItem {
    /// `1` if the symbol is tradable in a forward starting contract, `0` if not.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub allow_forward_starting: Option<AllowForwardStarting>,
    /// Amount the data feed is delayed (in minutes) due to Exchange licensing requirements. Only returned on `full` active symbols call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub delay_amount: Option<i64>,
    /// Display name.\n
    // Correct serde attribute construction - Use helper
    
    pub display_name: String,
    /// Display order.\n
    // Correct serde attribute construction - Use helper
    
    pub display_order: i64,
    /// `1` if market is currently open, `0` if closed.\n
    // Correct serde attribute construction - Use helper
    
    pub exchange_is_open: ExchangeIsOpen,
    /// Exchange name (for underlyings listed on a Stock Exchange). Only returned on `full` active symbols call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub exchange_name: Option<String>,
    /// Intraday interval minutes. Only returned on `full` active symbols call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub intraday_interval_minutes: Option<i64>,
    /// `1` indicates that trading is currently suspended, `0` if not.\n
    // Correct serde attribute construction - Use helper
    
    pub is_trading_suspended: IsTradingSuspended,
    /// Market category (forex, indices, etc).\n
    // Correct serde attribute construction - Use helper
    
    pub market: String,
    /// Translated market name.\n
    // Correct serde attribute construction - Use helper
    
    pub market_display_name: String,
    /// Pip size (i.e. minimum fluctuation amount).\n
    // Correct serde attribute construction - Use helper
    
    pub pip: String,
    /// For stock indices, the underlying currency for that instrument. Only returned on `full` active symbols call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub quoted_currency_symbol: Option<String>,
    /// Latest spot price of the underlying. Only returned on `full` active symbols call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub spot: Option<String>,
    /// Number of seconds elapsed since the last spot price. Only returned on `full` active symbols call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub spot_age: Option<String>,
    /// Daily percentage for a symbol. Only returned on 'full' active symbols call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub spot_percentage_change: Option<String>,
    /// Latest spot epoch time. Only returned on `full` active symbols call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub spot_time: Option<String>,
    /// Subgroup name.\n
    // Correct serde attribute construction - Use helper
    
    pub subgroup: String,
    /// Translated subgroup name.\n
    // Correct serde attribute construction - Use helper
    
    pub subgroup_display_name: String,
    /// Submarket name.\n
    // Correct serde attribute construction - Use helper
    
    pub submarket: String,
    /// Translated submarket name.\n
    // Correct serde attribute construction - Use helper
    
    pub submarket_display_name: String,
    /// The symbol code for this underlying.\n
    // Correct serde attribute construction - Use helper
    
    pub symbol: String,
    /// Symbol type (forex, commodities, etc).\n
    // Correct serde attribute construction - Use helper
    
    pub symbol_type: String,
}

