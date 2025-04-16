
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_virtual/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::email_consent_enum::EmailConsentEnum;
use crate::signup_device_enum::SignupDeviceEnum;
use crate::type_enum::TypeEnum;

/// Create a new virtual-money account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewAccountVirtualRequest {
    /// [Optional] Affiliate token, within 100 characters.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub affiliate_token: Option<String>,
    /// Password (Accepts any printable ASCII character. Must be within 8-25 characters, and include numbers, lowercase and uppercase letters. Must not be the same as the user's email address).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub client_password: Option<String>,
    /// [Optional] Date of first contact, format: `yyyy-mm-dd` in GMT timezone.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_first_contact: Option<String>,
    /// [Optional] Email address for signup.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub email: Option<String>,
    /// [Optional] Boolean value: 1 or 0, indicating whether the client has given consent for marketing emails.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub email_consent: Option<EmailConsentEnum>,
    /// [Optional] Google Click Identifier to track source.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub gclid_url: Option<String>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// Field 'new_account_virtual' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub new_account_virtual: Value,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// 2-letter country code (obtained from `residence_list` call).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub residence: Option<String>,
    /// [Optional] Show whether user has used mobile or desktop.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub signup_device: Option<SignupDeviceEnum>,
    /// Account type\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")] 
    pub r#type: Option<TypeEnum>,
    /// [Optional] Identifier of particular ad. Value must match Regex pattern to be recorded\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub utm_ad_id: Option<Value>,
    /// [Optional] Identifier of ad group in the campaign. Value must match Regex pattern to be recorded\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub utm_adgroup_id: Option<Value>,
    /// [Optional] Unique identifier of click on AdRoll ads platform. Value must match Regex pattern to be recorded\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub utm_adrollclk_id: Option<Value>,
    /// [Optional] Identifies a specific product promotion or strategic campaign such as a spring sale or other promotions. Value must match Regex pattern to be recorded\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub utm_campaign: Option<Value>,
    /// [Optional] Identifier of paid ad campaign. Value must match Regex pattern to be recorded\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub utm_campaign_id: Option<Value>,
    /// [Optional] Used to differentiate similar content, or links within the same ad. Value must match Regex pattern to be recorded\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub utm_content: Option<Value>,
    /// [Optional] Unique identifier of click on Facebook ads platform. Value must match Regex pattern to be recorded\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub utm_fbcl_id: Option<Value>,
    /// [Optional] Unique visitor identifier on Google Ads platform. Value must match Regex pattern to be recorded\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub utm_gl_client_id: Option<Value>,
    /// [Optional] Identifies the medium the link was used upon such as: email, CPC, or other methods of sharing. Value must match Regex pattern to be recorded\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub utm_medium: Option<Value>,
    /// [Optional] Unique click identifier on Microsoft Bing ads platform. Value must match Regex pattern to be recorded\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub utm_msclk_id: Option<Value>,
    /// [Optional] Identifies the source of traffic such as: search engine, newsletter, or other referral. Value must match Regex pattern to be recorded\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub utm_source: Option<Value>,
    /// [Optional] Used to send information related to the campaign term like paid search keywords. Value must match Regex pattern to be recorded\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub utm_term: Option<Value>,
    /// Email verification code (received from a `verify_email` call, which must be done first).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub verification_code: Option<String>,
}

