
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_virtual/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Create a new virtual-money account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct NewAccountVirtualRequest {
    /// [Optional] Affiliate token, within 100 characters.
    #[serde(rename = "affiliate_token", skip_serializing_if = "Option::is_none")]
    pub affiliate_token: String,
    /// Password (Accepts any printable ASCII character. Must be within 8-25 characters, and include numbers, lowercase and uppercase letters. Must not be the same as the user's email address).
    #[serde(rename = "client_password", skip_serializing_if = "Option::is_none")]
    pub client_password: String,
    /// [Optional] Date of first contact, format: `yyyy-mm-dd` in GMT timezone.
    #[serde(rename = "date_first_contact", skip_serializing_if = "Option::is_none")]
    pub date_first_contact: String,
    /// [Optional] Email address for signup.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: String,
    /// [Optional] Boolean value: 1 or 0, indicating whether the client has given consent for marketing emails.
    #[serde(rename = "email_consent", skip_serializing_if = "Option::is_none")]
    pub email_consent: EmailConsentEnum,
    /// [Optional] Google Click Identifier to track source.
    #[serde(rename = "gclid_url", skip_serializing_if = "Option::is_none")]
    pub gclid_url: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be `1`
    #[serde(rename = "new_account_virtual")]
    pub new_account_virtual: NewAccountVirtualEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// 2-letter country code (obtained from `residence_list` call).
    #[serde(rename = "residence", skip_serializing_if = "Option::is_none")]
    pub residence: String,
    /// [Optional] Show whether user has used mobile or desktop.
    #[serde(rename = "signup_device", skip_serializing_if = "Option::is_none")]
    pub signup_device: SignupDeviceEnum,
    /// Account type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type: TypeEnum,
    /// [Optional] Identifier of particular ad. Value must match Regex pattern to be recorded
    #[serde(rename = "utm_ad_id", skip_serializing_if = "Option::is_none")]
    pub utm_ad_id: Value,
    /// [Optional] Identifier of ad group in the campaign. Value must match Regex pattern to be recorded
    #[serde(rename = "utm_adgroup_id", skip_serializing_if = "Option::is_none")]
    pub utm_adgroup_id: Value,
    /// [Optional] Unique identifier of click on AdRoll ads platform. Value must match Regex pattern to be recorded
    #[serde(rename = "utm_adrollclk_id", skip_serializing_if = "Option::is_none")]
    pub utm_adrollclk_id: Value,
    /// [Optional] Identifies a specific product promotion or strategic campaign such as a spring sale or other promotions. Value must match Regex pattern to be recorded
    #[serde(rename = "utm_campaign", skip_serializing_if = "Option::is_none")]
    pub utm_campaign: Value,
    /// [Optional] Identifier of paid ad campaign. Value must match Regex pattern to be recorded
    #[serde(rename = "utm_campaign_id", skip_serializing_if = "Option::is_none")]
    pub utm_campaign_id: Value,
    /// [Optional] Used to differentiate similar content, or links within the same ad. Value must match Regex pattern to be recorded
    #[serde(rename = "utm_content", skip_serializing_if = "Option::is_none")]
    pub utm_content: Value,
    /// [Optional] Unique identifier of click on Facebook ads platform. Value must match Regex pattern to be recorded
    #[serde(rename = "utm_fbcl_id", skip_serializing_if = "Option::is_none")]
    pub utm_fbcl_id: Value,
    /// [Optional] Unique visitor identifier on Google Ads platform. Value must match Regex pattern to be recorded
    #[serde(rename = "utm_gl_client_id", skip_serializing_if = "Option::is_none")]
    pub utm_gl_client_id: Value,
    /// [Optional] Identifies the medium the link was used upon such as: email, CPC, or other methods of sharing. Value must match Regex pattern to be recorded
    #[serde(rename = "utm_medium", skip_serializing_if = "Option::is_none")]
    pub utm_medium: Value,
    /// [Optional] Unique click identifier on Microsoft Bing ads platform. Value must match Regex pattern to be recorded
    #[serde(rename = "utm_msclk_id", skip_serializing_if = "Option::is_none")]
    pub utm_msclk_id: Value,
    /// [Optional] Identifies the source of traffic such as: search engine, newsletter, or other referral. Value must match Regex pattern to be recorded
    #[serde(rename = "utm_source", skip_serializing_if = "Option::is_none")]
    pub utm_source: Value,
    /// [Optional] Used to send information related to the campaign term like paid search keywords. Value must match Regex pattern to be recorded
    #[serde(rename = "utm_term", skip_serializing_if = "Option::is_none")]
    pub utm_term: Value,
    /// Email verification code (received from a `verify_email` call, which must be done first).
    #[serde(rename = "verification_code", skip_serializing_if = "Option::is_none")]
    pub verification_code: String,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NewAccountVirtualEnum {
    Value1 = 1,
}

impl NewAccountVirtualEnum {
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
/// [Optional] Show whether user has used mobile or desktop.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignupDeviceEnum {
    Desktop,
    Mobile,
}

impl SignupDeviceEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Mobile => "mobile",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "desktop" => Some(Self::Desktop),
            "mobile" => Some(Self::Mobile),
            _ => None,
        }
    }
}
