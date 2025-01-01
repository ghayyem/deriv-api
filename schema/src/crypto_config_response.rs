
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/crypto_config/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The response will display the configuration details related to cryptocurrencies
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CryptoConfigResponse {
    /// Provides cryptocurrencies configuration.
    #[serde(rename = "crypto_config", skip_serializing_if = "Option::is_none")]
    pub crypto_config: CryptoConfig,
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
use std::collections::HashMap;


/// Provides cryptocurrencies configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CryptoConfig {
    /// Currency configuration including limitiations for each crypto currency.
    #[serde(rename = "currencies_config", flatten)]
    pub currencies_config: HashMap<String, CurrenciesConfigvalue>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Cryptocurrency code
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CurrenciesConfigvalue {
    /// Indicates whether Priority Withdrawal is enabled for the cryptocurrency.
    #[serde(rename = "is_priority_withdrawal_enabled", skip_serializing_if = "Option::is_none")]
    pub is_priority_withdrawal_enabled: IsPriorityWithdrawalEnabledEnum,
    /// Minimum deposit amount in corresponding cryptocurrency value.
    #[serde(rename = "minimum_deposit", skip_serializing_if = "Option::is_none")]
    pub minimum_deposit: f64,
    /// Minimum withdrawal for the cryptocurrency in USD.
    #[serde(rename = "minimum_withdrawal", skip_serializing_if = "Option::is_none")]
    pub minimum_withdrawal: f64,
}




/// Indicates whether Priority Withdrawal is enabled for the cryptocurrency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsPriorityWithdrawalEnabledEnum {
    Value0,
    Value1 = 1,
}

impl IsPriorityWithdrawalEnabledEnum {
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




