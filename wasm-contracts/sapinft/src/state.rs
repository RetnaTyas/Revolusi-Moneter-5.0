use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct SapiData {
    pub nfc_id: String,
    pub breed: String,
    pub birth_year: u64,
    pub weight: u64,
    pub minted_at: u64,
}

pub const OWNER: Item<Addr> = Item::new("owner");
pub const NEXT_ID: Item<u64> = Item::new("next_id");
pub const OWNER_OF: Map<u64, Addr> = Map::new("owner_of");
pub const SAPI_VALUE: Map<u64, Uint128> = Map::new("sapi_value");
pub const SAPI_METADATA: Map<u64, SapiData> = Map::new("sapi_metadata");
pub const APPROVALS: Map<u64, Addr> = Map::new("approvals");

/// Address of the burn hook contract
pub const BURN_HOOK: Item<Option<Addr>> = Item::new("burn_hook");

/// Timestamp of the last weight update for each token
pub const LAST_WEIGHT_UPDATE: Map<u64, u64> = Map::new("last_weight_update");

/// Mapping from NFC id bytes to token id to ensure uniqueness
pub const NFC_TO_TOKEN: Map<&[u8], u64> = Map::new("nfc_to_token");

/// Weight update validity window (7 days)
pub const WEIGHT_UPDATE_VALIDITY: u64 = 60 * 60 * 24 * 7;
