
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/verify_email/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;



// Import shared types from the *same* crate
use crate::signup_device::SignupDevice; 

// It's a struct
/// [Optional] Extra parameters that can be attached to the verify email link URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UrlParameters {
    /// [Optional] Affiliate token, within 32 characters.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub affiliate_token: Option<String>,
    /// [Optional] Date of first contact, format: yyyy-mm-dd in GMT timezone.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_first_contact: Option<String>,
    /// [Optional] Google Click Identifier to track source.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub gclid_url: Option<String>,
    /// [Optional] The amount to withdraw to the payment agent. Only allowed for payment agent withdraw.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub pa_amount: Option<String>,
    /// [Optional] The currency code. Only allowed for payment agent withdraw.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub pa_currency: Option<String>,
    /// [Optional] The payment agent loginid received from the `paymentagent_list` call. Only allowed for payment agent withdraw.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub pa_loginid: Option<String>,
    /// [Optional] Remarks about the withdraw. Only letters, numbers, space, period, comma, - ' are allowed. Only allowed for payment agent withdraw.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub pa_remarks: Option<String>,
    /// [Optional] The page ID to redirect to\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub redirect_to: Option<i64>,
    /// [Optional] Show whether user has used mobile or desktop.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub signup_device: Option<SignupDevice>,
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
}

