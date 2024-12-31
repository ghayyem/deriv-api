
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_financial_assessment/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This call gets the financial assessment details of client's account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct GetFinancialAssessmentResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Client's financial assessment details
    #[serde(rename = "get_financial_assessment", skip_serializing_if = "Option::is_none")]
    pub get_financial_assessment: GetFinancialAssessment,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Client's financial assessment details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct GetFinancialAssessment {
    /// The anticipated account turnover
    #[serde(rename = "account_turnover", skip_serializing_if = "Option::is_none")]
    pub account_turnover: String,
    /// Binary options trading experience
    #[serde(rename = "binary_options_trading_experience", skip_serializing_if = "Option::is_none")]
    pub binary_options_trading_experience: String,
    /// Binary options trading frequency
    #[serde(rename = "binary_options_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub binary_options_trading_frequency: String,
    /// How much experience do you have in CFD trading?
    #[serde(rename = "cfd_experience", skip_serializing_if = "Option::is_none")]
    pub cfd_experience: String,
    /// How many CFD trades have you placed in the past 12 months?
    #[serde(rename = "cfd_frequency", skip_serializing_if = "Option::is_none")]
    pub cfd_frequency: String,
    /// CFD Score
    #[serde(rename = "cfd_score", skip_serializing_if = "Option::is_none")]
    pub cfd_score: i64,
    /// In your understanding, CFD trading allows you to:
    #[serde(rename = "cfd_trading_definition", skip_serializing_if = "Option::is_none")]
    pub cfd_trading_definition: String,
    /// CFDs trading experience
    #[serde(rename = "cfd_trading_experience", skip_serializing_if = "Option::is_none")]
    pub cfd_trading_experience: String,
    /// CFDs trading frequency
    #[serde(rename = "cfd_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub cfd_trading_frequency: String,
    /// Commodities trading experience
    #[serde(rename = "commodities_trading_experience", skip_serializing_if = "Option::is_none")]
    pub commodities_trading_experience: String,
    /// Commodities trading frequency
    #[serde(rename = "commodities_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub commodities_trading_frequency: String,
    /// Level of Education
    #[serde(rename = "education_level", skip_serializing_if = "Option::is_none")]
    pub education_level: String,
    /// Industry of Employment
    #[serde(rename = "employment_industry", skip_serializing_if = "Option::is_none")]
    pub employment_industry: String,
    /// Employment Status
    #[serde(rename = "employment_status", skip_serializing_if = "Option::is_none")]
    pub employment_status: String,
    /// Estimated Net Worth
    #[serde(rename = "estimated_worth", skip_serializing_if = "Option::is_none")]
    pub estimated_worth: String,
    /// Financial Information Score
    #[serde(rename = "financial_information_score", skip_serializing_if = "Option::is_none")]
    pub financial_information_score: i64,
    /// Forex trading experience
    #[serde(rename = "forex_trading_experience", skip_serializing_if = "Option::is_none")]
    pub forex_trading_experience: String,
    /// Forex trading frequency
    #[serde(rename = "forex_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub forex_trading_frequency: String,
    /// Income Source
    #[serde(rename = "income_source", skip_serializing_if = "Option::is_none")]
    pub income_source: String,
    /// Indices trading experience
    #[serde(rename = "indices_trading_experience", skip_serializing_if = "Option::is_none")]
    pub indices_trading_experience: String,
    /// Indices trading frequency
    #[serde(rename = "indices_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub indices_trading_frequency: String,
    /// How does leverage affect CFD trading?
    #[serde(rename = "leverage_impact_trading", skip_serializing_if = "Option::is_none")]
    pub leverage_impact_trading: String,
    /// Leverage trading is high-risk, so it's a good idea to use risk management features such as stop loss. Stop loss allows you to
    #[serde(rename = "leverage_trading_high_risk_stop_loss", skip_serializing_if = "Option::is_none")]
    pub leverage_trading_high_risk_stop_loss: String,
    /// Net Annual Income
    #[serde(rename = "net_income", skip_serializing_if = "Option::is_none")]
    pub net_income: String,
    /// Occupation
    #[serde(rename = "occupation", skip_serializing_if = "Option::is_none")]
    pub occupation: String,
    /// Trading experience in other financial derivatives
    #[serde(rename = "other_derivatives_trading_experience", skip_serializing_if = "Option::is_none")]
    pub other_derivatives_trading_experience: String,
    /// Trading frequency in other financial derivatives
    #[serde(rename = "other_derivatives_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub other_derivatives_trading_frequency: String,
    /// Trading experience in other financial instruments
    #[serde(rename = "other_instruments_trading_experience", skip_serializing_if = "Option::is_none")]
    pub other_instruments_trading_experience: String,
    /// Trading frequency in other financial instruments
    #[serde(rename = "other_instruments_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub other_instruments_trading_frequency: String,
    /// When would you be required to pay an initial margin?
    #[serde(rename = "required_initial_margin", skip_serializing_if = "Option::is_none")]
    pub required_initial_margin: String,
    /// Do you understand that you could potentially lose 100% of the money you use to trade?
    #[serde(rename = "risk_tolerance", skip_serializing_if = "Option::is_none")]
    pub risk_tolerance: String,
    /// How much knowledge and experience do you have in relation to online trading?
    #[serde(rename = "source_of_experience", skip_serializing_if = "Option::is_none")]
    pub source_of_experience: String,
    /// Source of wealth
    #[serde(rename = "source_of_wealth", skip_serializing_if = "Option::is_none")]
    pub source_of_wealth: String,
    /// Stocks trading experience
    #[serde(rename = "stocks_trading_experience", skip_serializing_if = "Option::is_none")]
    pub stocks_trading_experience: String,
    /// Stocks trading frequency
    #[serde(rename = "stocks_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub stocks_trading_frequency: String,
    /// Total Score
    #[serde(rename = "total_score", skip_serializing_if = "Option::is_none")]
    pub total_score: i64,
    /// How much experience do you have with other financial instruments?
    #[serde(rename = "trading_experience_financial_instruments", skip_serializing_if = "Option::is_none")]
    pub trading_experience_financial_instruments: String,
    /// How many trades have you placed with other financial instruments in the past 12 months?
    #[serde(rename = "trading_frequency_financial_instruments", skip_serializing_if = "Option::is_none")]
    pub trading_frequency_financial_instruments: String,
    /// Trading Experience Score
    #[serde(rename = "trading_score", skip_serializing_if = "Option::is_none")]
    pub trading_score: i64,
}






