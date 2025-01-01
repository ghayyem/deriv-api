
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_markup_details/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Retrieve details of `app_markup` according to criteria specified.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AppMarkupDetailsRequest {
    /// [Optional] Specific application `app_id` to report on.
    #[serde(rename = "app_id", skip_serializing_if = "Option::is_none")]
    pub app_id: i64,
    /// Must be `1`
    #[serde(rename = "app_markup_details")]
    pub app_markup_details: AppMarkupDetailsEnum,
    /// [Optional] Specific client loginid to report on, like CR12345
    #[serde(rename = "client_loginid", skip_serializing_if = "Option::is_none")]
    pub client_loginid: String,
    /// Start date (epoch or YYYY-MM-DD HH:MM:SS). Results are inclusive of this time.
    #[serde(rename = "date_from")]
    pub date_from: String,
    /// End date (epoch or YYYY-MM-DD HH::MM::SS). Results are inclusive of this time.
    #[serde(rename = "date_to")]
    pub date_to: String,
    /// [Optional] If set to 1, will return `app_markup` transaction details.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: DescriptionEnum,
    /// [Optional] Apply upper limit to count of transactions received.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: f64,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Number of transactions to skip.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: i64,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] Sort direction on `transaction_time`. Other fields sort order is ASC.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: SortEnum,
    /// [Optional] One or more of the specified fields to sort on. Default sort field is by `transaction_time`.
    #[serde(rename = "sort_fields", skip_serializing_if = "Option::is_none")]
    pub sort_fields: Vec<SortFieldsitemEnum>,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppMarkupDetailsEnum {
    Value1 = 1,
}

impl AppMarkupDetailsEnum {
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
/// [Optional] If set to 1, will return `app_markup` transaction details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DescriptionEnum {
    Value0,
    Value1 = 1,
}

impl DescriptionEnum {
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
/// [Optional] Sort direction on `transaction_time`. Other fields sort order is ASC.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortEnum {
    ASC,
    DESC,
}

impl SortEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ASC => "ASC",
            Self::DESC => "DESC",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ASC" => Some(Self::ASC),
            "DESC" => Some(Self::DESC),
            _ => None,
        }
    }
}
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortFieldsitemEnum {
    App_Id,
    Client_Loginid,
    Transaction_Time,
}

impl SortFieldsitemEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::App_Id => "app_id",
            Self::Client_Loginid => "client_loginid",
            Self::Transaction_Time => "transaction_time",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "app_id" => Some(Self::App_Id),
            "client_loginid" => Some(Self::Client_Loginid),
            "transaction_time" => Some(Self::Transaction_Time),
            _ => None,
        }
    }
}
