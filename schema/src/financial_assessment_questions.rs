
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/financial_assessment_questions/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate

// It's a struct
/// The financial assessment questionnaire structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FinancialAssessmentQuestions {
    /// Field 'questions' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub questions: Option<Value>,
    /// The version of the financial assessment questionnaire.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub version: Option<String>,
}

