
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_limits/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Trading and Withdrawal Limits
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct GetLimitsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Trading limits of real account user
    #[serde(rename = "get_limits", skip_serializing_if = "Option::is_none")]
    pub get_limits: GetLimits,
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


/// Trading limits of real account user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct GetLimits {
    /// Maximum account cash balance
    #[serde(rename = "account_balance", skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<Value>,
    /// Cumulative daily transfer limits
    #[serde(rename = "daily_cumulative_amount_transfers", skip_serializing_if = "Option::is_none")]
    pub daily_cumulative_amount_transfers: DailyCumulativeAmountTransfers,
    /// Daily transfers
    #[serde(rename = "daily_transfers", skip_serializing_if = "Option::is_none")]
    pub daily_transfers: DailyTransfers,
    /// Maximum daily turnover
    #[serde(rename = "daily_turnover", skip_serializing_if = "Option::is_none")]
    pub daily_turnover: f64,
    /// Lifetime withdrawal limit
    #[serde(rename = "lifetime_limit", skip_serializing_if = "Option::is_none")]
    pub lifetime_limit: f64,
    /// Lifetime transfer limits. Only present when applicable to the current accout.
    #[serde(rename = "lifetime_transfers", skip_serializing_if = "Option::is_none")]
    pub lifetime_transfers: LifetimeTransfers,
    /// Contains limitation information for each market.
    #[serde(rename = "market_specific", skip_serializing_if = "Option::is_none", flatten)]
    pub market_specific: HashMap<String, Vec<MarketSpecificvalueitem>>,
    /// Number of days for num_of_days_limit withdrawal limit
    #[serde(rename = "num_of_days", skip_serializing_if = "Option::is_none")]
    pub num_of_days: i64,
    /// Withdrawal limit for num_of_days days
    #[serde(rename = "num_of_days_limit", skip_serializing_if = "Option::is_none")]
    pub num_of_days_limit: f64,
    /// Maximum number of open positions
    #[serde(rename = "open_positions", skip_serializing_if = "Option::is_none")]
    pub open_positions: i64,
    /// Maximum aggregate payouts on open positions
    #[serde(rename = "payout", skip_serializing_if = "Option::is_none")]
    pub payout: f64,
    /// Maximum payout for each symbol based on different barrier types.
    #[serde(rename = "payout_per_symbol", skip_serializing_if = "Option::is_none")]
    pub payout_per_symbol: Option<Value>,
    /// Maximum aggregate payouts on open positions per symbol and contract type. This limit can be exceeded up to the overall payout limit if there is no prior open position.
    #[serde(rename = "payout_per_symbol_and_contract_type", skip_serializing_if = "Option::is_none")]
    pub payout_per_symbol_and_contract_type: f64,
    /// Amount left to reach withdrawal limit
    #[serde(rename = "remainder", skip_serializing_if = "Option::is_none")]
    pub remainder: f64,
    /// Total withdrawal for num_of_days days
    #[serde(rename = "withdrawal_for_x_days_monetary", skip_serializing_if = "Option::is_none")]
    pub withdrawal_for_x_days_monetary: f64,
    /// Total withdrawal since inception
    #[serde(rename = "withdrawal_since_inception_monetary", skip_serializing_if = "Option::is_none")]
    pub withdrawal_since_inception_monetary: f64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Lifetime transfer limits. Only present when applicable to the current accout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LifetimeTransfers {
    /// Lifetime transfer limit for crypto to crypto currencies.
    #[serde(rename = "crypto_to_crypto", skip_serializing_if = "Option::is_none")]
    pub crypto_to_crypto: CryptoToCrypto,
    /// Lifetime transfer limit for crypto to fiat currencies.
    #[serde(rename = "crypto_to_fiat", skip_serializing_if = "Option::is_none")]
    pub crypto_to_fiat: CryptoToFiat,
    /// Lifetime transfer limit for fiat to crypto currencies.
    #[serde(rename = "fiat_to_crypto", skip_serializing_if = "Option::is_none")]
    pub fiat_to_crypto: FiatToCrypto,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Lifetime transfer limit for crypto to crypto currencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CryptoToCrypto {
    /// Total limit in client's currency.
    #[serde(rename = "allowed", skip_serializing_if = "Option::is_none")]
    pub allowed: f64,
    /// Remaining limit in client's currency.
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: f64,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Lifetime transfer limit for crypto to fiat currencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CryptoToFiat {
    /// Total limit in client's currency.
    #[serde(rename = "allowed", skip_serializing_if = "Option::is_none")]
    pub allowed: f64,
    /// Remaining limit in client's currency.
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: f64,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Lifetime transfer limit for fiat to crypto currencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct FiatToCrypto {
    /// Total limit in client's currency.
    #[serde(rename = "allowed", skip_serializing_if = "Option::is_none")]
    pub allowed: f64,
    /// Remaining limit in client's currency.
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: f64,
}








// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct MarketSpecificvalueitem {
    /// The group the profile belong to.
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: String,
    /// The market or submarket display name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The limit of payout for the submarket
    #[serde(rename = "payout_limit", skip_serializing_if = "Option::is_none")]
    pub payout_limit: f64,
    /// The limitation profile name.
    #[serde(rename = "profile_name", skip_serializing_if = "Option::is_none")]
    pub profile_name: String,
    /// The limit of turnover for the submarket
    #[serde(rename = "turnover_limit", skip_serializing_if = "Option::is_none")]
    pub turnover_limit: f64,
}








