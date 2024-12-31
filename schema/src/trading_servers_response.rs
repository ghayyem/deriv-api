
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_servers/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get list of servers for the platform provided.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradingServersResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Array containing platform server objects.
    #[serde(rename = "trading_servers", skip_serializing_if = "Option::is_none")]
    pub trading_servers: Vec<TradingServersitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradingServersitem {
    /// Supported trading account type.
    #[serde(rename = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: AccountTypeEnum,
    /// Flag to represent if this server is currently disabled or not
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: DisabledEnum,
    /// Current environment (installation instance) where servers are deployed. Currently, there are one demo and two real environments.
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: EnvironmentEnum,
    /// Object containing geolocation information of the server.
    #[serde(rename = "geolocation", skip_serializing_if = "Option::is_none")]
    pub geolocation: Geolocation,
    /// Server unique id.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: IdEnum,
    /// Market type
    #[serde(rename = "market_type", skip_serializing_if = "Option::is_none")]
    pub market_type: String,
    /// Error message to client when server is disabled
    #[serde(rename = "message_to_client", skip_serializing_if = "Option::is_none")]
    pub message_to_client: String,
    /// Flag to represent if this is server is recommended based on client's country of residence.
    #[serde(rename = "recommended", skip_serializing_if = "Option::is_none")]
    pub recommended: RecommendedEnum,
    /// Account type supported by the server.
    #[serde(rename = "supported_accounts", skip_serializing_if = "Option::is_none")]
    pub supported_accounts: Vec<String>,
}




/// Server unique id.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdEnum {
    P01_Ts01,
    P01_Ts02,
    P01_Ts03,
    P01_Ts04,
    P02_Ts02,
    P03_Ts01,
    P03_Ts02,
}

impl IdEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::P01_Ts01 => "p01_ts01",
            Self::P01_Ts02 => "p01_ts02",
            Self::P01_Ts03 => "p01_ts03",
            Self::P01_Ts04 => "p01_ts04",
            Self::P02_Ts02 => "p02_ts02",
            Self::P03_Ts01 => "p03_ts01",
            Self::P03_Ts02 => "p03_ts02",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "p01_ts01" => Some(Self::P01_Ts01),
            "p01_ts02" => Some(Self::P01_Ts02),
            "p01_ts03" => Some(Self::P01_Ts03),
            "p01_ts04" => Some(Self::P01_Ts04),
            "p02_ts02" => Some(Self::P02_Ts02),
            "p03_ts01" => Some(Self::P03_Ts01),
            "p03_ts02" => Some(Self::P03_Ts02),
            _ => None,
        }
    }
}


