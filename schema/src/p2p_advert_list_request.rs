
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advert_list/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::sort_by_enum::SortByEnum;
use crate::p2p_advert_list_enum::P2pAdvertListEnum;
use crate::hide_client_schedule_unavailable_enum::HideClientScheduleUnavailableEnum;
use crate::favourites_only_enum::FavouritesOnlyEnum;
use crate::hide_ineligible_enum::HideIneligibleEnum;
use crate::use_client_limits_enum::UseClientLimitsEnum;
use crate::block_trade_enum::BlockTradeEnum;
use crate::counterparty_type_enum::CounterpartyTypeEnum;

/// Returns available adverts for use with `p2p_order_create` .
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pAdvertListRequest {
    /// [Optional] ID of the advertiser to list adverts for.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub advertiser_id: Option<String>,
    /// [Optional] Search for advertiser by name. Partial matches will be returned.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub advertiser_name: Option<String>,
    /// [Optional] How much to buy or sell, used to calculate prices.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub amount: Option<f64>,
    /// [Optional] Return block trade adverts when 1, non-block trade adverts when 0 (default).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub block_trade: Option<BlockTradeEnum>,
    /// [Optional] Filter the adverts by `counterparty_type`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub counterparty_type: Option<CounterpartyTypeEnum>,
    /// [Optional] Only show adverts from favourite advertisers. Default is 0.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub favourites_only: Option<FavouritesOnlyEnum>,
    /// [Optional] If set to 1, adverts for which the current user's shcedule does not have availability from now until the full possible order expiry are not returned.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub hide_client_schedule_unavailable: Option<HideClientScheduleUnavailableEnum>,
    /// [Optional] If set to 1, adverts for which the current user does not meet counteryparty terms are not returned.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub hide_ineligible: Option<HideIneligibleEnum>,
    /// [Optional] Used for paging.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub limit: Option<i64>,
    /// [Optional] Currency to conduct payment transaction in. If not provided, only ads from country of residence will be returned.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub local_currency: Option<String>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Used for paging.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub offset: Option<i64>,
    /// Must be 1\n
    // Correct serde attribute construction - Use helper
    
    pub p2p_advert_list: P2pAdvertListEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Search by supported payment methods.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_method: Option<Vec<String>>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] How the results are sorted.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sort_by: Option<SortByEnum>,
    /// [Optional] If set to 1, ads that exceed this account's balance or turnover limits will not be shown.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub use_client_limits: Option<UseClientLimitsEnum>,
}

