
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/verify_email/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Verify an email address for various purposes. The system will send an email to the address containing a security code for verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct VerifyEmailRequest {
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Purpose of email verification, request_email and reset_password are the only two types restricted from all unoffical apps
    #[serde(rename = "type")]
    pub type: TypeEnum,
    /// [Optional] Extra parameters that can be attached to the verify email link URL.
    #[serde(rename = "url_parameters", skip_serializing_if = "Option::is_none")]
    pub url_parameters: UrlParameters,
    /// Email address to be verified.
    #[serde(rename = "verify_email")]
    pub verify_email: String,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// [Optional] Extra parameters that can be attached to the verify email link URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct UrlParameters {
    /// [Optional] Affiliate token, within 32 characters.
    #[serde(rename = "affiliate_token", skip_serializing_if = "Option::is_none")]
    pub affiliate_token: String,
    /// [Optional] Date of first contact, format: yyyy-mm-dd in GMT timezone.
    #[serde(rename = "date_first_contact", skip_serializing_if = "Option::is_none")]
    pub date_first_contact: String,
    /// [Optional] Google Click Identifier to track source.
    #[serde(rename = "gclid_url", skip_serializing_if = "Option::is_none")]
    pub gclid_url: String,
    /// [Optional] The amount to withdraw to the payment agent. Only allowed for payment agent withdraw.
    #[serde(rename = "pa_amount", skip_serializing_if = "Option::is_none")]
    pub pa_amount: f64,
    /// [Optional] The currency code. Only allowed for payment agent withdraw.
    #[serde(rename = "pa_currency", skip_serializing_if = "Option::is_none")]
    pub pa_currency: String,
    /// [Optional] The payment agent loginid received from the `paymentagent_list` call. Only allowed for payment agent withdraw.
    #[serde(rename = "pa_loginid", skip_serializing_if = "Option::is_none")]
    pub pa_loginid: String,
    /// [Optional] Remarks about the withdraw. Only letters, numbers, space, period, comma, - ' are allowed. Only allowed for payment agent withdraw.
    #[serde(rename = "pa_remarks", skip_serializing_if = "Option::is_none")]
    pub pa_remarks: String,
    /// [Optional] The page ID to redirect to
    #[serde(rename = "redirect_to", skip_serializing_if = "Option::is_none")]
    pub redirect_to: i64,
    /// [Optional] Show whether user has used mobile or desktop.
    #[serde(rename = "signup_device", skip_serializing_if = "Option::is_none")]
    pub signup_device: SignupDeviceEnum,
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
}






