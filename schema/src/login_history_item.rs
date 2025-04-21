
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/login_history/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::status::Status; 

// It's a struct
/// User login history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LoginHistoryItem {
    /// Type of action.\n
    // Correct serde attribute construction - Use helper
    
    pub action: String,
    /// Browser used\n
    // Correct serde attribute construction - Use helper
    
    pub browser: String,
    /// Country the login originated (IP Based))\n
    // Correct serde attribute construction - Use helper
    
    pub country: String,
    /// ISO6801 timestame of the activity\n
    // Correct serde attribute construction - Use helper
    
    pub datetime: String,
    /// Client device\n
    // Correct serde attribute construction - Use helper
    
    pub device: String,
    /// Provides details about browser, device used during login or logout\n
    // Correct serde attribute construction - Use helper
    
    pub environment: String,
    /// IP Address the login was from\n
    // Correct serde attribute construction - Use helper
    
    pub ip: String,
    /// Browser language\n
    // Correct serde attribute construction - Use helper
    
    pub language: String,
    /// Operating system\n
    // Correct serde attribute construction - Use helper
    
    pub os: String,
    /// Status of activity: 1 - success, 0 - failure\n
    // Correct serde attribute construction - Use helper
    
    pub status: Status,
    /// Epoch time of the activity\n
    // Correct serde attribute construction - Use helper
    
    pub time: i64,
    /// Version of the browser\n
    // Correct serde attribute construction - Use helper
    
    pub version: String,
}

