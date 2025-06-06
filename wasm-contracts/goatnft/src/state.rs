use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct GoatData {
    pub nfc_id: String,
    pub breed: String,
    pub birth_year: u64,
    pub weight: u64,
    pub minted_at: u64,
}

pub const OWNER: Item<Addr> = Item::new("owner");
pub const NEXT_ID: Item<u64> = Item::new("next_id");
pub const OWNER_OF: Map<u64, Addr> = Map::new("owner_of");
pub const GOAT_VALUE: Map<u64, Uint128> = Map::new("goat_value");
pub const GOAT_METADATA: Map<u64, GoatData> = Map::new("goat_metadata");
pub const APPROVALS: Map<u64, Addr> = Map::new("approvals");

