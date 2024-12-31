
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/partner_settings/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get Partner Settings (Partner Type, Company Details etc)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PartnerSettingsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Partner-specific information and settings.
    #[serde(rename = "partner_settings", skip_serializing_if = "Option::is_none")]
    pub partner_settings: PartnerSettings,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Partner-specific information and settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PartnerSettings {
    /// [Optional] Company name. Only applicable for partners of type company.
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: String,
    /// [Optional] Company registration number. Only applicable for partners of type company.
    #[serde(rename = "company_registration_number", skip_serializing_if = "Option::is_none")]
    pub company_registration_number: String,
    /// Defines whether this partner is an individual or a company.
    #[serde(rename = "partner_type", skip_serializing_if = "Option::is_none")]
    pub partner_type: PartnerTypeEnum,
    /// Platform URL for Dynamic works dashboard to be redirected from Partners Hub which will be set in BackOffice.
    #[serde(rename = "platform_URL", skip_serializing_if = "Option::is_none")]
    pub platform_url: String,
    /// Defines the provider platform.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: ProviderEnum,
    /// Partner's Website URI/Promotional Platform
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: String,
}






