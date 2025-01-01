
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/account_list/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Returns all accounts belonging to the authorized user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AccountListResponse {
    /// List of accounts for current user. This is also available from the `authroize` call.
    #[serde(rename = "account_list", skip_serializing_if = "Option::is_none")]
    pub account_list: Vec<AccountListitem>,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
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

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AccountListitem {
    /// Account category.
    #[serde(rename = "account_category")]
    pub account_category: AccountCategoryEnum,
    /// Account type.
    #[serde(rename = "account_type")]
    pub account_type: String,
    /// 2 letter broker code.
    #[serde(rename = "broker", skip_serializing_if = "Option::is_none")]
    pub broker: String,
    /// Creation time of the account as epoch.
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// Currency of specified account.
    #[serde(rename = "currency")]
    pub currency: String,
    /// Boolean value: 1 or 0, indicating whether the account is marked as disabled or not.
    #[serde(rename = "is_disabled")]
    pub is_disabled: IsDisabledEnum,
    /// Boolean value: 1 or 0, indicating whether the account is a virtual-money account.
    #[serde(rename = "is_virtual")]
    pub is_virtual: IsVirtualEnum,
    /// Landing company shortcode the account belongs to.
    #[serde(rename = "landing_company_name")]
    pub landing_company_name: String,
    /// Details of the list of Trading accounts linked to the Wallet account.
    #[serde(rename = "linked_to")]
    pub linked_to: Vec<LinkedToitem>,
    /// The account ID of specified account.
    #[serde(rename = "loginid")]
    pub loginid: String,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LinkedToitem {
    /// Account ID.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Account platform name.
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: PlatformEnum,
}




/// Account platform name.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformEnum {
    Ctrader,
    Dtrade,
    Dwallet,
    Dxtrade,
    Mt5,
}

impl PlatformEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ctrader => "ctrader",
            Self::Dtrade => "dtrade",
            Self::Dwallet => "dwallet",
            Self::Dxtrade => "dxtrade",
            Self::Mt5 => "mt5",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ctrader" => Some(Self::Ctrader),
            "dtrade" => Some(Self::Dtrade),
            "dwallet" => Some(Self::Dwallet),
            "dxtrade" => Some(Self::Dxtrade),
            "mt5" => Some(Self::Mt5),
            _ => None,
        }
    }
}


/// Account category.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCategoryEnum {
    Trading,
    Wallet,
}

impl AccountCategoryEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Trading => "trading",
            Self::Wallet => "wallet",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "trading" => Some(Self::Trading),
            "wallet" => Some(Self::Wallet),
            _ => None,
        }
    }
}
/// Boolean value: 1 or 0, indicating whether the account is marked as disabled or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsDisabledEnum {
    Value1 = 1,
    Value0,
}

impl IsDisabledEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
            Self::Value0 => "0",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            "0" => Some(Self::Value0),
            _ => None,
        }
    }
}
/// Boolean value: 1 or 0, indicating whether the account is a virtual-money account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsVirtualEnum {
    Value1 = 1,
    Value0,
}

impl IsVirtualEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
            Self::Value0 => "0",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            "0" => Some(Self::Value0),
            _ => None,
        }
    }
}


/// Action name of the request made.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MsgTypeEnum {
    Account_List,
}

impl MsgTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Account_List => "account_list",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "account_list" => Some(Self::Account_List),
            _ => None,
        }
    }
}
