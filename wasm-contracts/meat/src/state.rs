use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const NAME: &str = "Market-Enabled Agricultural Token";
pub const SYMBOL: &str = "MEAT";
pub const DECIMALS: u8 = 18;

pub const OWNER: Item<Addr> = Item::new("owner");

pub const BALANCES: Map<&Addr, Uint128> = Map::new("balances");
pub const ALLOWANCES: Map<(&Addr, &Addr), Uint128> = Map::new("allowances");
pub const TOTAL_SUPPLY: Item<Uint128> = Item::new("total_supply");

pub const MINTERS: Map<&Addr, bool> = Map::new("minters");
pub const BURNERS: Map<&Addr, bool> = Map::new("burners");

pub const SUBTYPE_BALANCES: Map<(&Addr, &[u8]), Uint128> = Map::new("subtype_balances");
pub const SUBTYPE_TOTAL_SUPPLY: Map<&[u8], Uint128> = Map::new("subtype_total_supply");
pub const SUBTYPE_LINEAGE: Map<(&Addr, &[u8]), u64> = Map::new("subtype_lineage");
pub const USER_SUBTYPES: Map<&Addr, Vec<Vec<u8>>> = Map::new("user_subtypes");
pub const TRANSFER_CURSOR: Map<&Addr, u64> = Map::new("transfer_cursor");
pub const RATE_HANDLER: Item<Option<Addr>> = Item::new("rate_handler");

pub const DECIMAL_FACTOR: u128 = 1_000_000_000_000_000_000u128;
pub const INITIAL_SUPPLY: u128 = 1_000u128 * DECIMAL_FACTOR;
