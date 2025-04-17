
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_maltainvest/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::tin_skipped_enum::TinSkippedEnum;
use crate::secret_question_enum::SecretQuestionEnum;
use crate::leverage_trading_high_risk_stop_loss_enum::LeverageTradingHighRiskStopLossEnum;
use crate::required_initial_margin_enum::RequiredInitialMarginEnum;
use crate::resident_self_declaration_enum::ResidentSelfDeclarationEnum;
use crate::cfd_experience_enum::CfdExperienceEnum;
use crate::risk_tolerance_enum::RiskToleranceEnum;
use crate::source_of_experience_enum::SourceOfExperienceEnum;
use crate::new_account_maltainvest_enum::NewAccountMaltainvestEnum;
use crate::cfd_trading_definition_enum::CfdTradingDefinitionEnum;
use crate::leverage_impact_trading_enum::LeverageImpactTradingEnum;
use crate::trading_frequency_financial_instruments_enum::TradingFrequencyFinancialInstrumentsEnum;
use crate::tnc_acceptance_enum::TncAcceptanceEnum;
use crate::salutation_enum::SalutationEnum;
use crate::fatca_declaration_enum::FatcaDeclarationEnum;
use crate::client_type_enum::ClientTypeEnum;
use crate::accept_risk_enum::AcceptRiskEnum;
use crate::cfd_frequency_enum::CfdFrequencyEnum;
use crate::trading_experience_financial_instruments_enum::TradingExperienceFinancialInstrumentsEnum;

/// This call opens a new real-money account with the `maltainvest` Landing Company. This call can be made from a virtual-money account or real-money account at Deriv (Europe) Limited. If it is the latter, client information fields in this call will be ignored and data from your existing real-money account will be used.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewAccountMaltainvestRequest {
    /// Show whether client has accepted risk disclaimer.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub accept_risk: Option<AcceptRiskEnum>,
    /// Field 'account_opening_reason' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_opening_reason: Option<Value>,
    /// [Optional] The anticipated account turnover.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_turnover: Option<String>,
    /// Within 100 characters\n
    // Correct serde attribute construction - Use helper
    
    pub address_city: String,
    /// Within 70 characters, with no leading whitespaces and may contain letters/numbers and/or any of following characters '.,:;()@#/-\n
    // Correct serde attribute construction - Use helper
    
    pub address_line_1: String,
    /// Field 'address_line_2' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_line_2: Option<Value>,
    /// [Optional] Within 20 characters and may not contain '+'.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_postcode: Option<String>,
    /// [Optional] Possible value receive from `states_list` call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_state: Option<f64>,
    /// [Optional] Affiliate token, within 32 characters.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub affiliate_token: Option<String>,
    /// [Optional] The phone's calling country code. Don't include the `+` sign. Up to 4 digits.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub calling_country_code: Option<String>,
    /// How much experience do you have in CFD trading?\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub cfd_experience: Option<CfdExperienceEnum>,
    /// How many CFD trades have you placed in the past 12 months?\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub cfd_frequency: Option<CfdFrequencyEnum>,
    /// In your understanding, CFD trading allows you to:\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub cfd_trading_definition: Option<CfdTradingDefinitionEnum>,
    /// [Optional] Country of legal citizenship, 2-letter country code. Possible value receive from `residence_list` call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub citizen: Option<f64>,
    /// [Optional] Indicates whether this is for a client requesting an account with professional status.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub client_type: Option<ClientTypeEnum>,
    /// [Optional] To set currency of the account. List of supported currencies can be acquired with `payout_currencies` call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency: Option<String>,
    /// Date of birth format: yyyy-mm-dd.\n
    // Correct serde attribute construction - Use helper
    
    pub date_of_birth: String,
    /// [Optional] Level of Education\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub education_level: Option<String>,
    /// [Optional] Industry of Employment.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub employment_industry: Option<String>,
    /// Employment Status.\n
    // Correct serde attribute construction - Use helper
    
    pub employment_status: String,
    /// [Optional] Estimated Net Worth.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub estimated_worth: Option<String>,
    /// [Optional] Indicates client's self-declaration of FATCA.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub fatca_declaration: Option<FatcaDeclarationEnum>,
    /// [Optional] Version of the financial information\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub financial_information_version: Option<String>,
    /// Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes.\n
    // Correct serde attribute construction - Use helper
    
    pub first_name: String,
    /// [Optional] Income Source.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub income_source: Option<String>,
    /// [Optional] Investment intention.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub investment_intention: Option<String>,
    /// Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes.\n
    // Correct serde attribute construction - Use helper
    
    pub last_name: String,
    /// How does leverage affect CFD trading?\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub leverage_impact_trading: Option<LeverageImpactTradingEnum>,
    /// Leverage trading is high-risk, so it's a good idea to use risk management features such as stop loss. Stop loss allows you to\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub leverage_trading_high_risk_stop_loss: Option<LeverageTradingHighRiskStopLossEnum>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Net Annual Income.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub net_income: Option<String>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub new_account_maltainvest: NewAccountMaltainvestEnum,
    /// [Optional] Indicates client's self-declaration of not being a PEP/RCA.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub non_pep_declaration: Option<i64>,
    /// [Optional] Occupation.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub occupation: Option<String>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] The phone's national format, don't include the `+` sign nor the calling country code. Up to 15 digits are allowed.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub phone: Option<String>,
    /// [Optional] Place of birth, 2-letter country code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub place_of_birth: Option<String>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// When would you be required to pay an initial margin?\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub required_initial_margin: Option<RequiredInitialMarginEnum>,
    /// 2-letter country code, possible value receive from `residence_list` call.\n
    // Correct serde attribute construction - Use helper
    
    pub residence: f64,
    /// [Optional] Indicates client's self declaration for opening account under own initiative, must be 1\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub resident_self_declaration: Option<ResidentSelfDeclarationEnum>,
    /// Do you understand that you could potentially lose 100% of the money you use to trade?\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub risk_tolerance: Option<RiskToleranceEnum>,
    /// Accept any value in enum list.\n
    // Correct serde attribute construction - Use helper
    
    pub salutation: SalutationEnum,
    /// [Optional] Answer to secret question, within 4-50 characters.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub secret_answer: Option<String>,
    /// [Optional] Accept any value in enum list.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub secret_question: Option<SecretQuestionEnum>,
    /// How much knowledge and experience do you have in relation to online trading?\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub source_of_experience: Option<SourceOfExperienceEnum>,
    /// [Optional] Source of wealth.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub source_of_wealth: Option<String>,
    /// Tax identification number. Only applicable for real money account. Required for `maltainvest` landing company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tax_identification_number: Option<String>,
    /// Residence for tax purpose. Comma separated iso country code if multiple jurisdictions. Only applicable for real money account. Required for `maltainvest` landing company.\n
    // Correct serde attribute construction - Use helper
    
    pub tax_residence: String,
    /// [Optional] Whether the client has skipped the TIN form. Only applicable for real money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tin_skipped: Option<TinSkippedEnum>,
    /// The tnc acceptance status of the user.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tnc_acceptance: Option<TncAcceptanceEnum>,
    /// How much experience do you have with other financial instruments?\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trading_experience_financial_instruments: Option<TradingExperienceFinancialInstrumentsEnum>,
    /// How many trades have you placed with other financial instruments in the past 12 months?\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trading_frequency_financial_instruments: Option<TradingFrequencyFinancialInstrumentsEnum>,
}

