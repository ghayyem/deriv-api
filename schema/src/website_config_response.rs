
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_config/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// All config related settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct WebsiteConfigResponse {
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
    /// Server status and other information regarding general settings
    #[serde(rename = "website_config", skip_serializing_if = "Option::is_none")]
    pub website_config: WebsiteConfig,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Server status and other information regarding general settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct WebsiteConfig {
    /// Available currencies and their information
    #[serde(rename = "currencies_config", flatten)]
    pub currencies_config: HashMap<String, CurrenciesConfigvalue>,
    /// Feature flags related to the website/server for various features and options: 
///  - 'signup_with_optional_email_verification': Allow signup with optional email verification.
    #[serde(rename = "feature_flags", skip_serializing_if = "Option::is_none")]
    pub feature_flags: Vec<String>,
    /// Payments Agents system settings.
    #[serde(rename = "payment_agents", skip_serializing_if = "Option::is_none")]
    pub payment_agents: PaymentAgents,
    /// Provides codes for languages supported.
    #[serde(rename = "supported_languages", skip_serializing_if = "Option::is_none")]
    pub supported_languages: Vec<String>,
    /// Latest terms and conditions version.
    #[serde(rename = "terms_conditions_version", skip_serializing_if = "Option::is_none")]
    pub terms_conditions_version: String,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Currency code
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CurrenciesConfigvalue {
    /// Number of fractional digits.
    #[serde(rename = "fractional_digits")]
    pub fractional_digits: f64,
    /// Current status for payment deposit for the currency
    #[serde(rename = "is_deposit_suspended")]
    pub is_deposit_suspended: IsDepositSuspendedEnum,
    /// Current status for the currency
    #[serde(rename = "is_suspended")]
    pub is_suspended: IsSuspendedEnum,
    /// Current status for payment withdrawal for the currency
    #[serde(rename = "is_withdrawal_suspended")]
    pub is_withdrawal_suspended: IsWithdrawalSuspendedEnum,
    /// Name of the currency.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// List of cashier platforms supported for this currency. It is categorized by cashier and ramp (on-ramp, off-ramp) platforms.
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Platform,
    /// Default stake value for the currency.
    #[serde(rename = "stake_default")]
    pub stake_default: f64,
    /// Fees and range of allowed amount for transfer between accounts with different types of currencies.
    #[serde(rename = "transfer_between_accounts")]
    pub transfer_between_accounts: TransferBetweenAccounts,
    /// Type of the currency.
    #[serde(rename = "type")]
    pub type: TypeEnum,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// List of cashier platforms supported for this currency. It is categorized by cashier and ramp (on-ramp, off-ramp) platforms.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Platform {
    /// Supported platforms for the cashier, this is passed to provider attribute of `cashier` call
    #[serde(rename = "cashier")]
    pub cashier: Vec<Value>,
    /// Supported platforms for the ramp (on-ramp, off-ramp)
    #[serde(rename = "ramp")]
    pub ramp: Vec<Value>,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Fees and range of allowed amount for transfer between accounts with different types of currencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TransferBetweenAccounts {
    /// The fee that applies for transfer between accounts with different types of currencies.
    #[serde(rename = "fees", flatten)]
    pub fees: HashMap<String, f64>,
    /// Range of allowed amount for transfer between accounts.
    #[serde(rename = "limits")]
    pub limits: Value,
    /// Range of allowed amount for transfer between ctrader accounts.
    #[serde(rename = "limits_ctrader", skip_serializing_if = "Option::is_none")]
    pub limits_ctrader: LimitsCtrader,
    /// Range of allowed amount for transfer between dxtrade accounts.
    #[serde(rename = "limits_dxtrade", skip_serializing_if = "Option::is_none")]
    pub limits_dxtrade: LimitsDxtrade,
    /// Range of allowed amount for transfer between mt5 accounts.
    #[serde(rename = "limits_mt5", skip_serializing_if = "Option::is_none")]
    pub limits_mt_5: LimitsMt5,
}






/// Current status for payment deposit for the currency
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsDepositSuspendedEnum {
    Value0,
    Value1 = 1,
}

impl IsDepositSuspendedEnum {
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
/// Current status for the currency
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsSuspendedEnum {
    Value0,
    Value1 = 1,
}

impl IsSuspendedEnum {
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
/// Current status for payment withdrawal for the currency
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsWithdrawalSuspendedEnum {
    Value0,
    Value1 = 1,
}

impl IsWithdrawalSuspendedEnum {
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
use std::collections::HashMap;


/// Payments Agents system settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentAgents {
    /// Initial deposit requirement per country.
    #[serde(rename = "initial_deposit_per_country", flatten)]
    pub initial_deposit_per_country: HashMap<String, f64>,
}








