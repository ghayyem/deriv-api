
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/financial_assessment_questions/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::type_enum::TypeEnum; 
use crate::answers_item::AnswersItem; 

// It's a struct
/// Question object containing the question details and possible answers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuestionsValue {
    /// Array of possible answers for this question\n
    // Correct serde attribute construction - Use helper
    
    pub answers: Vec<AnswersItem>,
    /// Conditions that determine if this question should be hidden\n
    // Correct serde attribute construction - Use helper
    
    pub hide_if: Vec<String>,
    /// The question text to display\n
    // Correct serde attribute construction - Use helper
    
    pub question: String,
    /// The type of input required for this question\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type")] 
    pub r#type: TypeEnum,
}

