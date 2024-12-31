
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/cashier/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Request the cashier info for the specified type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CashierRequest {
    /// [Optional] Address for crypto withdrawal. Only applicable for `api` type.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: String,
    /// [Optional] Amount for crypto withdrawal. Only applicable for `api` type.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: f64,
    /// Operation which needs to be requested from cashier
    #[serde(rename = "cashier")]
    pub cashier: CashierEnum,
    /// [Optional] If set to `1`, only validation is performed. Only applicable for `withdraw` using `crypto` provider and `api` type.
    #[serde(rename = "dry_run", skip_serializing_if = "Option::is_none")]
    pub dry_run: DryRunEnum,
    /// [Optional] The `unique_id` of the estimated fee received from `crypto_estimations` call in case the client is willing to pay the returned fee in order to prioritise their withdrawal request.
    #[serde(rename = "estimated_fee_unique_id", skip_serializing_if = "Option::is_none")]
    pub estimated_fee_unique_id: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Cashier provider.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: ProviderEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] Data is returned from the cashier. The `crypto` provider only supports `api` (not `url`) for crypto accounts.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type: TypeEnum,
    /// [Optional] Email verification code (received from a `verify_email` call, which must be done first)
    #[serde(rename = "verification_code", skip_serializing_if = "Option::is_none")]
    pub verification_code: String,
}




/// Operation which needs to be requested from cashier
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CashierEnum {
    Deposit,
    Withdraw,
}

impl CashierEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Deposit => "deposit",
            Self::Withdraw => "withdraw",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "deposit" => Some(Self::Deposit),
            "withdraw" => Some(Self::Withdraw),
            _ => None,
        }
    }
}
/// [Optional] If set to `1`, only validation is performed. Only applicable for `withdraw` using `crypto` provider and `api` type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DryRunEnum {
    Value0,
    Value1 = 1,
}

impl DryRunEnum {
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
/// [Optional] Cashier provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEnum {
    Doughflow,
    Crypto,
}

impl ProviderEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Doughflow => "doughflow",
            Self::Crypto => "crypto",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "doughflow" => Some(Self::Doughflow),
            "crypto" => Some(Self::Crypto),
            _ => None,
        }
    }
}
