
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/transaction/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Return transaction updates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TransactionResponse {
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
    /// Realtime stream of user transaction updates.
    #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
    pub transaction: Transaction,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Realtime stream of user transaction updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Transaction {
    /// The transaction type.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: ActionEnum,
    /// It is the amount of transaction performed.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: f64,
    /// Balance amount
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: f64,
    /// Barrier of the contract. Only applicable to single barrier contracts. Could be undefined if a contract does not have a barrier.
    #[serde(rename = "barrier", skip_serializing_if = "Option::is_none")]
    pub barrier: Option<Value>,
    /// It is the contract ID.
    #[serde(rename = "contract_id", skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<Value>,
    /// Transaction currency
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Epoch value of the expiry time of the contract. Please note that in case of buy transaction this is approximate value not exact one.
    #[serde(rename = "date_expiry", skip_serializing_if = "Option::is_none")]
    pub date_expiry: i64,
    /// Display name of symbol
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// The high barrier of a contract. Only applicable to double barrier contracts.
    #[serde(rename = "high_barrier", skip_serializing_if = "Option::is_none")]
    pub high_barrier: Value,
    /// A per-connection unique identifier. Can be passed to the `forget` API call to unsubscribe.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Description of contract purchased
    #[serde(rename = "longcode", skip_serializing_if = "Option::is_none")]
    pub longcode: String,
    /// The low barrier of a contract. Only applicable to double barrier contracts.
    #[serde(rename = "low_barrier", skip_serializing_if = "Option::is_none")]
    pub low_barrier: String,
    /// Time at which contract was purchased, present only for sell transaction
    #[serde(rename = "purchase_time", skip_serializing_if = "Option::is_none")]
    pub purchase_time: i64,
    /// The pip-sized target spot price where the contract will be closed automatically at the loss specified by the user.
    #[serde(rename = "stop_loss", skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<Value>,
    /// The pip-sized target spot price where the contract will be closed automatically when the value of the contract is close to zero. This is set by the us.
    #[serde(rename = "stop_out", skip_serializing_if = "Option::is_none")]
    pub stop_out: Option<Value>,
    /// Symbol code
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: String,
    /// The pip-sized target spot price where the contract will be closed automatically at the profit specified by the user.
    #[serde(rename = "take_profit", skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<Value>,
    /// It is the transaction ID. Every contract (buy or sell) or payment has a unique ID.
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: i64,
    /// Time at which transaction was performed: for buy it is purchase time, for sell it is sell time.
    #[serde(rename = "transaction_time", skip_serializing_if = "Option::is_none")]
    pub transaction_time: i64,
}




/// The transaction type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionEnum {
    Buy,
    Sell,
    Deposit,
    Withdrawal,
    Escrow,
    Adjustment,
    Virtual_Credit,
    Transfer,
}

impl ActionEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
            Self::Deposit => "deposit",
            Self::Withdrawal => "withdrawal",
            Self::Escrow => "escrow",
            Self::Adjustment => "adjustment",
            Self::Virtual_Credit => "virtual_credit",
            Self::Transfer => "transfer",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "buy" => Some(Self::Buy),
            "sell" => Some(Self::Sell),
            "deposit" => Some(Self::Deposit),
            "withdrawal" => Some(Self::Withdrawal),
            "escrow" => Some(Self::Escrow),
            "adjustment" => Some(Self::Adjustment),
            "virtual_credit" => Some(Self::Virtual_Credit),
            "transfer" => Some(Self::Transfer),
            _ => None,
        }
    }
}


