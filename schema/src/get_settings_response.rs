
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_settings/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with User Settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct GetSettingsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// User information and settings.
    #[serde(rename = "get_settings", skip_serializing_if = "Option::is_none")]
    pub get_settings: GetSettings,
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
use std::collections::HashMap;


/// User information and settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct GetSettings {
    /// Purpose and reason for requesting the account opening. Only applicable for real money account.
    #[serde(rename = "account_opening_reason", skip_serializing_if = "Option::is_none")]
    pub account_opening_reason: Option<Value>,
    /// City (note: Only available for users who have at least one real account)
    #[serde(rename = "address_city", skip_serializing_if = "Option::is_none")]
    pub address_city: String,
    /// Address line 1 (note: Only available for users who have at least one real account)
    #[serde(rename = "address_line_1", skip_serializing_if = "Option::is_none")]
    pub address_line_1: String,
    /// Address line 2 (note: Only available for users who have at least one real account)
    #[serde(rename = "address_line_2", skip_serializing_if = "Option::is_none")]
    pub address_line_2: String,
    /// Post Code (note: Only available for users who have at least one real account)
    #[serde(rename = "address_postcode", skip_serializing_if = "Option::is_none")]
    pub address_postcode: String,
    /// State (note: Only available for users who have at least one real account)
    #[serde(rename = "address_state", skip_serializing_if = "Option::is_none")]
    pub address_state: String,
    /// Boolean value 1 or 0, indicating permission to allow others to follow your trades. Note: not applicable for Virtual account. Only allow for real money account.
    #[serde(rename = "allow_copiers", skip_serializing_if = "Option::is_none")]
    pub allow_copiers: AllowCopiersEnum,
    /// Country of legal citizenship, 2-letter country code.
    #[serde(rename = "citizen", skip_serializing_if = "Option::is_none")]
    pub citizen: String,
    /// Latest terms and conditions version accepted by client
    #[serde(rename = "client_tnc_status", skip_serializing_if = "Option::is_none")]
    pub client_tnc_status: Option<Value>,
    /// Cooldown expiration epoch date when a client fails appropriateness tests
    #[serde(rename = "cooling_off_expiration_date", skip_serializing_if = "Option::is_none")]
    pub cooling_off_expiration_date: Option<Value>,
    /// User Country (same as residence field) - deprecated
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<Value>,
    /// 2-letter country code ISO standard
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<Value>,
    /// Epoch of user's birthday (note: Only available for users who have at least one real account)
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<Value>,
    /// Boolean value 1 or 0, indicating if user email belong to dxtrade exception list.
    #[serde(rename = "dxtrade_user_exception", skip_serializing_if = "Option::is_none")]
    pub dxtrade_user_exception: DxtradeUserExceptionEnum,
    /// User Email
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: String,
    /// Boolean value 1 or 0, indicating permission to use email address for any contact which may include marketing
    #[serde(rename = "email_consent", skip_serializing_if = "Option::is_none")]
    pub email_consent: EmailConsentEnum,
    /// Employment Status.
    #[serde(rename = "employment_status", skip_serializing_if = "Option::is_none")]
    pub employment_status: EmploymentStatusEnum,
    /// Indicates client's self-declaration for FATCA.
    #[serde(rename = "fatca_declaration", skip_serializing_if = "Option::is_none")]
    pub fatca_declaration: Option<Value>,
    /// Contains features that are enabled or disabled for this user
    #[serde(rename = "feature_flag", skip_serializing_if = "Option::is_none")]
    pub feature_flag: FeatureFlag,
    /// First name (note: Only available for users who have at least one real account)
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: String,
    /// Returns 1 if the client has a secret answer, 0 otherwise.
    #[serde(rename = "has_secret_answer", skip_serializing_if = "Option::is_none")]
    pub has_secret_answer: HasSecretAnswerEnum,
    /// A list of profile fields which are immutable (read-only unless they are not set yet) due to landing company regulations and the current status of the account.
    #[serde(rename = "immutable_fields", skip_serializing_if = "Option::is_none")]
    pub immutable_fields: Vec<String>,
    /// Boolean value 1 or 0, indicating whether is payment agent (note: not applicable for virtual money accounts)
    #[serde(rename = "is_authenticated_payment_agent", skip_serializing_if = "Option::is_none")]
    pub is_authenticated_payment_agent: IsAuthenticatedPaymentAgentEnum,
    /// Last name (note: Only available for users who have at least one real account)
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: String,
    /// Indicates client's self-declaration of not being a PEP/RCA (Politically Exposed Person/Relatives and Close Associates). Note: returned for real accounts only.
    #[serde(rename = "non_pep_declaration", skip_serializing_if = "Option::is_none")]
    pub non_pep_declaration: NonPepDeclarationEnum,
    /// Telephone (note: Only available for users who have at least one real account)
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<Value>,
    /// The status of the Phone Number Verification.
    #[serde(rename = "phone_number_verification", skip_serializing_if = "Option::is_none")]
    pub phone_number_verification: PhoneNumberVerification,
    /// Place of birth, 2-letter country code.
    #[serde(rename = "place_of_birth", skip_serializing_if = "Option::is_none")]
    pub place_of_birth: Option<Value>,
    /// User's preferred language, ISO standard code of language
    #[serde(rename = "preferred_language", skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<Value>,
    /// Boolean value 1 or 0, indicating if client has requested professional status.
    #[serde(rename = "request_professional_status", skip_serializing_if = "Option::is_none")]
    pub request_professional_status: RequestProfessionalStatusEnum,
    /// User Country
    #[serde(rename = "residence", skip_serializing_if = "Option::is_none")]
    pub residence: Option<Value>,
    /// Salutation (note: Only available for users who have at least one real account)
    #[serde(rename = "salutation", skip_serializing_if = "Option::is_none")]
    pub salutation: String,
    /// Tax identification number. Only applicable for real money account.
    #[serde(rename = "tax_identification_number", skip_serializing_if = "Option::is_none")]
    pub tax_identification_number: Option<Value>,
    /// Residence for tax purpose. Comma separated iso country code if multiple jurisdictions. Only applicable for real money account.
    #[serde(rename = "tax_residence", skip_serializing_if = "Option::is_none")]
    pub tax_residence: Option<Value>,
    /// [Optional] Whether the client has skipped the TIN form. Only applicable for real money account.
    #[serde(rename = "tin_skipped", skip_serializing_if = "Option::is_none")]
    pub tin_skipped: TinSkippedEnum,
    /// Terms and condition status tells us whether all the accounts of this user has accepted the latest T&C version.
    #[serde(rename = "tnc_status", skip_serializing_if = "Option::is_none")]
    pub tnc_status: TncStatus,
    /// The start date to inform users within a grace period, with the notification automatically deactivating after the specified duration.
    #[serde(rename = "tnc_update_notification_start_date", skip_serializing_if = "Option::is_none")]
    pub tnc_update_notification_start_date: Option<Value>,
    /// Boolean value 1 or 0, indicating if client has enabled the Trading Hub dashboard
    #[serde(rename = "trading_hub", skip_serializing_if = "Option::is_none")]
    pub trading_hub: i64,
    /// Hash generated using user details to verify whether the user is legitimate for our customer support system.
    #[serde(rename = "user_hash", skip_serializing_if = "Option::is_none")]
    pub user_hash: Option<Value>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Contains features that are enabled or disabled for this user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct FeatureFlag {
    /// Boolean value 1 or 0 indicating whether his feature this enabled or not
    #[serde(rename = "wallet", skip_serializing_if = "Option::is_none")]
    pub wallet: WalletEnum,
}




/// Boolean value 1 or 0 indicating whether his feature this enabled or not
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletEnum {
    Value0,
    Value1 = 1,
}

impl WalletEnum {
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


// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// The status of the Phone Number Verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PhoneNumberVerification {
    /// Indicates the attempts remaining for /phone_number_challenge
    #[serde(rename = "challenge_attempts_remaining", skip_serializing_if = "Option::is_none")]
    pub challenge_attempts_remaining: i64,
    /// (Optional) Indicates the timestamp for the next verification attempt
    #[serde(rename = "next_attempt", skip_serializing_if = "Option::is_none")]
    pub next_attempt: i64,
    /// (Optional) Indicates the timestamp for the next email verification attempt
    #[serde(rename = "next_email_attempt", skip_serializing_if = "Option::is_none")]
    pub next_email_attempt: i64,
    /// (Optional) Indicates the timestamp for the next verify attempt
    #[serde(rename = "next_verify_attempt", skip_serializing_if = "Option::is_none")]
    pub next_verify_attempt: i64,
    /// (Optional) Indicates the timestamp for the current's session email code expiry
    #[serde(rename = "session_timestamp", skip_serializing_if = "Option::is_none")]
    pub session_timestamp: i64,
    /// Indicates the verification status of the client's phone number.
    #[serde(rename = "verified")]
    pub verified: VerifiedEnum,
    /// Indicates the attempts remaining for /phone_number_verification
    #[serde(rename = "verify_attempts_remaining", skip_serializing_if = "Option::is_none")]
    pub verify_attempts_remaining: i64,
}




/// Indicates the verification status of the client's phone number.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerifiedEnum {
    Value0,
    Value1 = 1,
}

impl VerifiedEnum {
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


/// Boolean value 1 or 0, indicating permission to allow others to follow your trades. Note: not applicable for Virtual account. Only allow for real money account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowCopiersEnum {
    Value0,
    Value1 = 1,
}

impl AllowCopiersEnum {
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
/// Boolean value 1 or 0, indicating if user email belong to dxtrade exception list.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DxtradeUserExceptionEnum {
    Value0,
    Value1 = 1,
}

impl DxtradeUserExceptionEnum {
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
/// Boolean value 1 or 0, indicating permission to use email address for any contact which may include marketing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmailConsentEnum {
    Value0,
    Value1 = 1,
}

impl EmailConsentEnum {
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
/// Employment Status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmploymentStatusEnum {
    Employed,
    Pensioner,
    SelfEmployed,
    Student,
    Unemployed,
}

impl EmploymentStatusEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Employed => "Employed",
            Self::Pensioner => "Pensioner",
            Self::SelfEmployed => "Self-Employed",
            Self::Student => "Student",
            Self::Unemployed => "Unemployed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Employed" => Some(Self::Employed),
            "Pensioner" => Some(Self::Pensioner),
            "Self-Employed" => Some(Self::SelfEmployed),
            "Student" => Some(Self::Student),
            "Unemployed" => Some(Self::Unemployed),
            _ => None,
        }
    }
}
/// Returns 1 if the client has a secret answer, 0 otherwise.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HasSecretAnswerEnum {
    Value0,
    Value1 = 1,
}

impl HasSecretAnswerEnum {
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
/// Boolean value 1 or 0, indicating whether is payment agent (note: not applicable for virtual money accounts)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsAuthenticatedPaymentAgentEnum {
    Value0,
    Value1 = 1,
}

impl IsAuthenticatedPaymentAgentEnum {
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
/// Indicates client's self-declaration of not being a PEP/RCA (Politically Exposed Person/Relatives and Close Associates). Note: returned for real accounts only.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NonPepDeclarationEnum {
    Value0,
    Value1 = 1,
}

impl NonPepDeclarationEnum {
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
/// Boolean value 1 or 0, indicating if client has requested professional status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestProfessionalStatusEnum {
    Value0,
    Value1 = 1,
}

impl RequestProfessionalStatusEnum {
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
/// [Optional] Whether the client has skipped the TIN form. Only applicable for real money account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TinSkippedEnum {
    Value0,
    Value1 = 1,
}

impl TinSkippedEnum {
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


