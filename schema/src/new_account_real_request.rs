
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_real/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::tin_skipped_enum::TinSkippedEnum;
use crate::secret_question_enum::SecretQuestionEnum;
use crate::new_account_real_enum::NewAccountRealEnum;
use crate::account_turnover_enum::AccountTurnoverEnum;
use crate::tnc_acceptance_enum::TncAcceptanceEnum;
use crate::salutation_enum::SalutationEnum;
use crate::fatca_declaration_enum::FatcaDeclarationEnum;
use crate::client_type_enum::ClientTypeEnum;

/// This call opens a new real-money account. This call can be made from a virtual-money or a real-money account. If it is the latter, client information fields in this call will be ignored and data from your existing real-money account will be used.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewAccountRealRequest {
    /// Field 'account_opening_reason' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_opening_reason: Option<Value>,
    /// [Optional] The anticipated account turnover.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_turnover: Option<AccountTurnoverEnum>,
    /// [Optional] Within 100 characters.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_city: Option<String>,
    /// Within 70 characters, with no leading whitespaces and may contain letters/numbers and/or any of following characters '.,:;()@#/-\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_line_1: Option<String>,
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
    /// [Optional] Country of legal citizenship, 2-letter country code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub citizen: Option<String>,
    /// [Optional] Indicates whether this is for a client requesting an account with professional status.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub client_type: Option<ClientTypeEnum>,
    /// [Optional] To set currency of the account. List of supported currencies can be acquired with `payout_currencies` call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency: Option<String>,
    /// Date of birth format: `yyyy-mm-dd`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_of_birth: Option<String>,
    /// Employment Status.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub employment_status: Option<String>,
    /// [Optional] Indicates client's self-declaration of FATCA.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub fatca_declaration: Option<FatcaDeclarationEnum>,
    /// [Optional] The version of the financial information form.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub financial_information_version: Option<String>,
    /// Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub first_name: Option<String>,
    /// Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_name: Option<String>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub new_account_real: NewAccountRealEnum,
    /// [Optional] Indicates client's self-declaration of not being a PEP/RCA (Politically Exposed Person/Relatives and Close Associates).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub non_pep_declaration: Option<i64>,
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
    /// 2-letter country code, possible value receive from `residence_list` call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub residence: Option<f64>,
    /// [Optional] Accept any value in enum list.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub salutation: Option<SalutationEnum>,
    /// [Optional] Answer to secret question, within 4-50 characters. Required for new account and existing client details will be used if client open another account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub secret_answer: Option<String>,
    /// [Optional] Accept any value in enum list. Required for new account and existing client details will be used if client open another account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub secret_question: Option<SecretQuestionEnum>,
    /// [Optional] Tax identification number. Only applicable for real money account. Required for `maltainvest` landing company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tax_identification_number: Option<String>,
    /// [Optional] Residence for tax purpose. Comma separated iso country code if multiple jurisdictions. Only applicable for real money account. Required for `maltainvest` landing company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tax_residence: Option<String>,
    /// [Optional] Whether the client has skipped the TIN form. Only applicable for real money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tin_skipped: Option<TinSkippedEnum>,
    /// The tnc acceptance status of the user.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tnc_acceptance: Option<TncAcceptanceEnum>,
}

