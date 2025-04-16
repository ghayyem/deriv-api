
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/financial_assessment_questions/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnswersItem {
    /// Array of conditions that determine if this answer should be hidden\n
    // Correct serde attribute construction - Use helper
    
    pub hide_if: Vec<String>,
    /// The key for the answer option\n
    // Correct serde attribute construction - Use helper
    
    pub key: String,
    /// The next question to show after this answer is selected\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub next_node: Option<String>,
    /// Display text for this answer option\n
    // Correct serde attribute construction - Use helper
    
    pub value: String,
}

