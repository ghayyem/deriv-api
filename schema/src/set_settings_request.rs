
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/set_settings/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Set User Settings (this call should be used in conjunction with `get_settings`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct SetSettingsRequest {
    /// [Optional] Purpose and reason for requesting the account opening. Only applicable for real money account. Required for clients that have not set it yet. Can only be set once.
    #[serde(rename = "account_opening_reason", skip_serializing_if = "Option::is_none")]
    pub account_opening_reason: AccountOpeningReasonEnum,
    /// [Optional] Note: not applicable for virtual account. Required field for real money account.
    #[serde(rename = "address_city", skip_serializing_if = "Option::is_none")]
    pub address_city: String,
    /// [Optional] Note: not applicable for virtual account. Required field for real money account.
    #[serde(rename = "address_line_1", skip_serializing_if = "Option::is_none")]
    pub address_line_1: String,
    /// [Optional] Note: not applicable for virtual account. Optional field for real money account.
    #[serde(rename = "address_line_2", skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<Value>,
    /// [Optional] Note: not applicable for virtual account. Optional field for real money account.
    #[serde(rename = "address_postcode", skip_serializing_if = "Option::is_none")]
    pub address_postcode: String,
    /// [Optional] Note: not applicable for virtual account. Optional field for real money account.
    #[serde(rename = "address_state", skip_serializing_if = "Option::is_none")]
    pub address_state: String,
    /// [Optional] Boolean value 1 or 0, indicating permission to allow others to follow your trades. Note: not applicable for Virtual account. Only allow for real money account.
    #[serde(rename = "allow_copiers", skip_serializing_if = "Option::is_none")]
    pub allow_copiers: AllowCopiersEnum,
    /// [Optional] Country of legal citizenship, 2-letter country code.
    #[serde(rename = "citizen", skip_serializing_if = "Option::is_none")]
    pub citizen: Option<Value>,
    /// [Optional] Date of birth format: yyyy-mm-dd (can only be changed on unauthenticated svg accounts).
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: String,
    /// Boolean value 1 or 0, indicating if user email belong to dxtrade exception list.
    #[serde(rename = "dxtrade_user_exception", skip_serializing_if = "Option::is_none")]
    pub dxtrade_user_exception: DxtradeUserExceptionEnum,
    /// [Optional] Boolean value 1 or 0, indicating permission to use email address for any contact which may include marketing
    #[serde(rename = "email_consent", skip_serializing_if = "Option::is_none")]
    pub email_consent: EmailConsentEnum,
    /// [Optional] Employment Status.
    #[serde(rename = "employment_status", skip_serializing_if = "Option::is_none")]
    pub employment_status: EmploymentStatusEnum,
    /// [Optional] Enable or disable one or multiple features.
    #[serde(rename = "feature_flag", skip_serializing_if = "Option::is_none")]
    pub feature_flag: FeatureFlag,
    /// [Optional] Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes (can only be changed on unauthenticated svg accounts).
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: String,
    /// [Optional] Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes (can only be changed on unauthenticated svg accounts).
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Indicates client's self-declaration of not being a PEP/RCA (Politically Exposed Person/Relatives and Close Associates). Effective for real accounts only.
    #[serde(rename = "non_pep_declaration", skip_serializing_if = "Option::is_none")]
    pub non_pep_declaration: NonPepDeclarationEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Note: not applicable for virtual account. Starting with `+` followed by 9-35 digits, hyphens or space.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<Value>,
    /// [Optional] Place of birth, 2-letter country code.
    #[serde(rename = "place_of_birth", skip_serializing_if = "Option::is_none")]
    pub place_of_birth: String,
    /// [Optional] User's preferred language, ISO standard language code
    #[serde(rename = "preferred_language", skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<Value>,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] Required when client wants to be treated as professional. Applicable for financial accounts only.
    #[serde(rename = "request_professional_status", skip_serializing_if = "Option::is_none")]
    pub request_professional_status: RequestProfessionalStatusEnum,
    /// [Optional] 2-letter country code. Note: not applicable for real money account. Only allow for Virtual account without residence set.
    #[serde(rename = "residence", skip_serializing_if = "Option::is_none")]
    pub residence: Option<Value>,
    /// [Optional] Accept any value in enum list (can only be changed on unauthenticated svg accounts).
    #[serde(rename = "salutation", skip_serializing_if = "Option::is_none")]
    pub salutation: SalutationEnum,
    /// [Optional] Answer to secret question, within 4-50 characters. Required for new account and existing client details will be used if client opens another account.
    #[serde(rename = "secret_answer", skip_serializing_if = "Option::is_none")]
    pub secret_answer: String,
    /// [Optional] Accept any value in enum list. Required for new account and existing client details will be used if client opens another account.
    #[serde(rename = "secret_question", skip_serializing_if = "Option::is_none")]
    pub secret_question: SecretQuestionEnum,
    /// Must be `1`
    #[serde(rename = "set_settings")]
    pub set_settings: SetSettingsEnum,
    /// [Optional] Tax identification number. Only applicable for real money account. Required for maltainvest landing company.
    #[serde(rename = "tax_identification_number", skip_serializing_if = "Option::is_none")]
    pub tax_identification_number: String,
    /// [Optional] Residence for tax purpose. Comma separated iso country code if multiple jurisdictions. Only applicable for real money account. Required for maltainvest landing company.
    #[serde(rename = "tax_residence", skip_serializing_if = "Option::is_none")]
    pub tax_residence: String,
    /// [Optional] Whether the client has skipped the TIN form. Only applicable for real money account.
    #[serde(rename = "tin_skipped", skip_serializing_if = "Option::is_none")]
    pub tin_skipped: TinSkippedEnum,
    /// [Optional] Enable/Disable Trading Hub dashboard
    #[serde(rename = "trading_hub", skip_serializing_if = "Option::is_none")]
    pub trading_hub: TradingHubEnum,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetSettingsEnum {
    Value1 = 1,
}

impl SetSettingsEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// [Optional] Enable/Disable Trading Hub dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradingHubEnum {
    Value0,
    Value1 = 1,
}

impl TradingHubEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
