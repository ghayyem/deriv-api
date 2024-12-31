
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/partner_accounts/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get All Partner Accounts (Partner account details like website, provider, company details)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PartnerAccountsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Partner Accounts against a user
    #[serde(rename = "partner_accounts", skip_serializing_if = "Option::is_none")]
    pub partner_accounts: PartnerAccounts,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Partner Accounts against a user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PartnerAccounts {
    /// List of partner_settigns for all accounts associated as partners.
    #[serde(rename = "partner_settings", skip_serializing_if = "Option::is_none")]
    pub partner_settings: Vec<PartnerSettingsitem>,
    /// Platform URL for Dynamic works dashboard to be redirected from Partners Hub which will be set in BackOffice.
    #[serde(rename = "platform_url", skip_serializing_if = "Option::is_none", flatten)]
    pub platform_url: HashMap<String, String>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PartnerSettingsitem {
    /// The user id.
    #[serde(rename = "binary_user_id", skip_serializing_if = "Option::is_none")]
    pub binary_user_id: i64,
    /// [Optional] Company name. Only applicable for partners of type company.
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: String,
    /// [Optional] Company registration number. Only applicable for partners of type company.
    #[serde(rename = "company_registration_no", skip_serializing_if = "Option::is_none")]
    pub company_registration_no: String,
    /// Partner's login ID
    #[serde(rename = "partner_loginid", skip_serializing_if = "Option::is_none")]
    pub partner_loginid: String,
    /// Defines whether this partner is an individual or a company.
    #[serde(rename = "partner_type", skip_serializing_if = "Option::is_none")]
    pub partner_type: PartnerTypeEnum,
    /// Defines the provider platform.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: ProviderEnum,
    /// Partner's Website URI/Promotional Platform
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: String,
}








