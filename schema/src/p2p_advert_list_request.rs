
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advert_list/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Returns available adverts for use with `p2p_order_create` .
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertListRequest {
    /// [Optional] ID of the advertiser to list adverts for.
    #[serde(rename = "advertiser_id", skip_serializing_if = "Option::is_none")]
    pub advertiser_id: String,
    /// [Optional] Search for advertiser by name. Partial matches will be returned.
    #[serde(rename = "advertiser_name", skip_serializing_if = "Option::is_none")]
    pub advertiser_name: String,
    /// [Optional] How much to buy or sell, used to calculate prices.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: f64,
    /// [Optional] Return block trade adverts when 1, non-block trade adverts when 0 (default).
    #[serde(rename = "block_trade", skip_serializing_if = "Option::is_none")]
    pub block_trade: BlockTradeEnum,
    /// [Optional] Filter the adverts by `counterparty_type`.
    #[serde(rename = "counterparty_type", skip_serializing_if = "Option::is_none")]
    pub counterparty_type: CounterpartyTypeEnum,
    /// [Optional] Only show adverts from favourite advertisers. Default is 0.
    #[serde(rename = "favourites_only", skip_serializing_if = "Option::is_none")]
    pub favourites_only: FavouritesOnlyEnum,
    /// [Optional] If set to 1, adverts for which the current user's shcedule does not have availability from now until the full possible order expiry are not returned.
    #[serde(rename = "hide_client_schedule_unavailable", skip_serializing_if = "Option::is_none")]
    pub hide_client_schedule_unavailable: HideClientScheduleUnavailableEnum,
    /// [Optional] If set to 1, adverts for which the current user does not meet counteryparty terms are not returned.
    #[serde(rename = "hide_ineligible", skip_serializing_if = "Option::is_none")]
    pub hide_ineligible: HideIneligibleEnum,
    /// [Optional] Used for paging.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i64,
    /// [Optional] Currency to conduct payment transaction in. If not provided, only ads from country of residence will be returned.
    #[serde(rename = "local_currency", skip_serializing_if = "Option::is_none")]
    pub local_currency: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used for paging.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: i64,
    /// Must be 1
    #[serde(rename = "p2p_advert_list")]
    pub p_2p_advert_list: P2pAdvertListEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Search by supported payment methods.
    #[serde(rename = "payment_method", skip_serializing_if = "Option::is_none")]
    pub payment_method: Vec<String>,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] How the results are sorted.
    #[serde(rename = "sort_by", skip_serializing_if = "Option::is_none")]
    pub sort_by: SortByEnum,
    /// [Optional] If set to 1, ads that exceed this account's balance or turnover limits will not be shown.
    #[serde(rename = "use_client_limits", skip_serializing_if = "Option::is_none")]
    pub use_client_limits: UseClientLimitsEnum,
}




/// [Optional] Only show adverts from favourite advertisers. Default is 0.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FavouritesOnlyEnum {
    Value0,
    Value1 = 1,
}

impl FavouritesOnlyEnum {
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
/// [Optional] If set to 1, adverts for which the current user's shcedule does not have availability from now until the full possible order expiry are not returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HideClientScheduleUnavailableEnum {
    Value0,
    Value1 = 1,
}

impl HideClientScheduleUnavailableEnum {
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
/// [Optional] If set to 1, adverts for which the current user does not meet counteryparty terms are not returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HideIneligibleEnum {
    Value0,
    Value1 = 1,
}

impl HideIneligibleEnum {
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
/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pAdvertListEnum {
    Value1 = 1,
}

impl P2pAdvertListEnum {
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
/// [Optional] How the results are sorted.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortByEnum {
    Completion,
    Rate,
    Rating,
    Recommended,
}

impl SortByEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Completion => "completion",
            Self::Rate => "rate",
            Self::Rating => "rating",
            Self::Recommended => "recommended",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "completion" => Some(Self::Completion),
            "rate" => Some(Self::Rate),
            "rating" => Some(Self::Rating),
            "recommended" => Some(Self::Recommended),
            _ => None,
        }
    }
}
/// [Optional] If set to 1, ads that exceed this account's balance or turnover limits will not be shown.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UseClientLimitsEnum {
    Value0,
    Value1 = 1,
}

impl UseClientLimitsEnum {
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
