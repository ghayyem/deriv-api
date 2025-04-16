
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/copytrading_statistics/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 

use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Import shared types from the *same* crate

// It's a struct
/// Statistics of the trader
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopytradingStatistics {
    /// This is the epoch the investor started trading.\n
    // Correct serde attribute construction - Use helper
    
    pub active_since: DateTime<Utc>,
    /// Average seconds of keeping positions open.\n
    // Correct serde attribute construction - Use helper
    
    pub avg_duration: i64,
    /// Average loss of trades in percentage.\n
    // Correct serde attribute construction - Use helper
    
    pub avg_loss: f64,
    /// Average profitable trades in percentage.\n
    // Correct serde attribute construction - Use helper
    
    pub avg_profit: f64,
    /// Number of copiers for this trader.\n
    // Correct serde attribute construction - Use helper
    
    pub copiers: f64,
    /// Represents the net change in equity for a 12-month period.\n
    // Correct serde attribute construction - Use helper
    
    pub last_12months_profitable_trades: f64,
    /// Represents the net change in equity per month.\n
    // Correct serde attribute construction - Use helper
    
    pub monthly_profitable_trades: HashMap<String, f64>,
    /// Trader performance probability.\n
    // Correct serde attribute construction - Use helper
    
    pub performance_probability: f64,
    /// Total number of trades for all time.\n
    // Correct serde attribute construction - Use helper
    
    pub total_trades: i64,
    /// Represents the portfolio distribution by markets.\n
    // Correct serde attribute construction - Use helper
    
    pub trades_breakdown: HashMap<String, f64>,
    /// Number of profit trades in percentage.\n
    // Correct serde attribute construction - Use helper
    
    pub trades_profitable: f64,
    /// Represents the net change in equity per year.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub yearly_profitable_trades: Option<HashMap<String, f64>>,
}

