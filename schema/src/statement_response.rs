
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/statement/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A summary of account statement is received
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct StatementResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Account statement.
    #[serde(rename = "statement", skip_serializing_if = "Option::is_none")]
    pub statement: Statement,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Account statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Statement {
    /// Number of transactions returned in this call
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: f64,
    /// Array of returned transactions
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Vec<Transactionsitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Transactionsitem {
    /// It is the type of action.
    #[serde(rename = "action_type", skip_serializing_if = "Option::is_none")]
    pub action_type: ActionTypeEnum,
    /// It is the amount of transaction.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: f64,
    /// ID of the application where this contract was purchased.
    #[serde(rename = "app_id", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<Value>,
    /// It is the remaining balance.
    #[serde(rename = "balance_after", skip_serializing_if = "Option::is_none")]
    pub balance_after: f64,
    /// It is the contract ID.
    #[serde(rename = "contract_id", skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<Value>,
    /// Contains details about fees used for transfer. It is present only when action type is transfer.
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Fees,
    /// Contains details of account from which amount was transferred. It is present only when action type is transfer.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: From,
    /// The description of contract purchased if description is set to `1`.
    #[serde(rename = "longcode", skip_serializing_if = "Option::is_none")]
    pub longcode: String,
    /// Payout price
    #[serde(rename = "payout", skip_serializing_if = "Option::is_none")]
    pub payout: Option<Value>,
    /// Time at which contract was purchased, present only for sell transaction
    #[serde(rename = "purchase_time", skip_serializing_if = "Option::is_none")]
    pub purchase_time: i64,
    /// Internal transaction identifier for the corresponding buy transaction ( set only for contract selling )
    #[serde(rename = "reference_id", skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<Value>,
    /// Compact description of the contract purchased if description is set to `1`.
    #[serde(rename = "shortcode", skip_serializing_if = "Option::is_none")]
    pub shortcode: Option<Value>,
    /// Contains details of account to which amount was transferred. It is present only when action type is transfer.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: To,
    /// It is the transaction ID. In statement every contract (buy or sell) and every payment has a unique ID.
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: i64,
    /// It is the time of transaction.
    #[serde(rename = "transaction_time", skip_serializing_if = "Option::is_none")]
    pub transaction_time: i64,
    /// Additional withdrawal details such as typical processing times, if description is set to `1`.
    #[serde(rename = "withdrawal_details", skip_serializing_if = "Option::is_none")]
    pub withdrawal_details: String,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Contains details about fees used for transfer. It is present only when action type is transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Fees {
    /// Fees amount
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: f64,
    /// Fees currency
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Minimum amount of fees
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: f64,
    /// Fees percentage
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: f64,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Contains details of account from which amount was transferred. It is present only when action type is transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct From {
    /// Login id of the account from which money was transferred.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Contains details of account to which amount was transferred. It is present only when action type is transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct To {
    /// Login id of the account to which money was transferred.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
}










