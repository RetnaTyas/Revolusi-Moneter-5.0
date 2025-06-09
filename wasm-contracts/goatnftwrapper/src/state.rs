use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const GOAT_NFT: Item<Addr> = Item::new("goat_nft");
pub const GOAT_TOKEN: Item<Addr> = Item::new("goat_token");

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WrappedInfo {
    pub owner: Addr,
    pub goat_amount: Uint128,
}

pub const WRAPPED: Map<u64, WrappedInfo> = Map::new("wrapped");
