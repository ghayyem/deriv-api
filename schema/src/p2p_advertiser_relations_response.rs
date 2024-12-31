
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_relations/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Returns information about favourite and blocked advertisers.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertiserRelationsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// P2P advertiser relations information.
    #[serde(rename = "p2p_advertiser_relations", skip_serializing_if = "Option::is_none")]
    pub p_2p_advertiser_relations: P2pAdvertiserRelations,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// P2P advertiser relations information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertiserRelations {
    /// List of advertisers blocked by the current user.
    #[serde(rename = "blocked_advertisers")]
    pub blocked_advertisers: Vec<BlockedAdvertisersitem>,
    /// Favourite advertisers of the current user.
    #[serde(rename = "favourite_advertisers")]
    pub favourite_advertisers: Vec<FavouriteAdvertisersitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

/// Advertiser details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct BlockedAdvertisersitem {
    /// The epoch time that the advertiser was blocked.
    #[serde(rename = "created_time", skip_serializing_if = "Option::is_none")]
    pub created_time: i64,
    /// Advertiser unique identifer.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Advertiser displayed name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

/// Advertiser details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct FavouriteAdvertisersitem {
    /// The epoch time that the advertiser was set as favourite.
    #[serde(rename = "created_time", skip_serializing_if = "Option::is_none")]
    pub created_time: i64,
    /// Advertiser unique identifer.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Advertiser displayed name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
}








