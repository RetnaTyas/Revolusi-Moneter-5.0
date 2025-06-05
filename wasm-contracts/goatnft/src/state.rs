use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const NEXT_ID: Item<u64> = Item::new("next_id");
pub const OWNER_OF: Map<u64, Addr> = Map::new("owner_of");
pub const GOAT_VALUE: Map<u64, Uint128> = Map::new("goat_value");
pub const ALLOWED_CONTRACT: Item<Option<Addr>> = Item::new("allowed_contract");
