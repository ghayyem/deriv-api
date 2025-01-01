
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/balance/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Return details of user account balance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct BalanceResponse {
    /// Current balance of one or more accounts.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Balance,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// For subscription requests only.
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Subscription,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Current balance of one or more accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Balance {
    /// List of active accounts.
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none", flatten)]
    pub accounts: HashMap<String, Accountsvalue>,
    /// Balance of current account.
    #[serde(rename = "balance")]
    pub balance: f64,
    /// Currency of current account.
    #[serde(rename = "currency")]
    pub currency: String,
    /// A per-connection unique identifier. Can be passed to the `forget` API call to unsubscribe.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Client loginid.
    #[serde(rename = "loginid")]
    pub loginid: String,
    /// Summary totals of accounts by type.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Total,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Individual accounts details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Accountsvalue {
    /// Account balance
    #[serde(rename = "balance")]
    pub balance: f64,
    /// Account balance converted the total currency.
    #[serde(rename = "converted_amount")]
    pub converted_amount: f64,
    /// Account currency.
    #[serde(rename = "currency")]
    pub currency: String,
    /// If set to 1, this is a demo account.
    #[serde(rename = "demo_account")]
    pub demo_account: DemoAccountEnum,
    /// Boolean value of 1 or 0. Indicates the status of account. 1 indicates account is good and accessible.
    #[serde(rename = "status")]
    pub status: StatusEnum,
    /// Type of account.
    #[serde(rename = "type")]
    pub type: TypeEnum,
}




/// If set to 1, this is a demo account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DemoAccountEnum {
    Value0,
    Value1 = 1,
}

impl DemoAccountEnum {
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
/// Boolean value of 1 or 0. Indicates the status of account. 1 indicates account is good and accessible.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusEnum {
    Value1 = 1,
    Value0,
}

impl StatusEnum {
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
/// Type of account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeEnum {
    Mt5,
    Deriv,
}

impl TypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mt5 => "mt5",
            Self::Deriv => "deriv",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "mt5" => Some(Self::Mt5),
            "deriv" => Some(Self::Deriv),
            _ => None,
        }
    }
}


// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Summary totals of accounts by type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Total {
    /// Total balance of all real money Deriv accounts.
    #[serde(rename = "deriv", skip_serializing_if = "Option::is_none")]
    pub deriv: Deriv,
    /// Total balance of all demo Deriv accounts.
    #[serde(rename = "deriv_demo", skip_serializing_if = "Option::is_none")]
    pub deriv_demo: DerivDemo,
    /// Total balance of all MT5 real money accounts.
    #[serde(rename = "mt5", skip_serializing_if = "Option::is_none")]
    pub mt_5: Mt5,
    /// Total balance of all MT5 demo accounts.
    #[serde(rename = "mt5_demo", skip_serializing_if = "Option::is_none")]
    pub mt_5_demo: Mt5Demo,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Total balance of all real money Deriv accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Deriv {
    /// Total of balances.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Currency of total.
    #[serde(rename = "currency")]
    pub currency: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Total balance of all demo Deriv accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct DerivDemo {
    /// Total of balances.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Currency of total.
    #[serde(rename = "currency")]
    pub currency: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Total balance of all MT5 real money accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5 {
    /// Total balance of all MT5 accounts
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Currency of total.
    #[serde(rename = "currency")]
    pub currency: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Total balance of all MT5 demo accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5Demo {
    /// Total of balances.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Currency of total.
    #[serde(rename = "currency")]
    pub currency: String,
}










// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// For subscription requests only.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Subscription {
    /// A per-connection unique identifier. Can be passed to the `forget` API call to unsubscribe.
    #[serde(rename = "id")]
    pub id: String,
}






