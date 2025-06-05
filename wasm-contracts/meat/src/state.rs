use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const NAME: &str = "Market-Enabled Agricultural Token";
pub const SYMBOL: &str = "MEAT";
pub const DECIMALS: u8 = 18;

pub const OWNER: Item<Addr> = Item::new("owner");
pub const GOAT_CONTRACT: Item<Addr> = Item::new("goat_contract");
pub const RATE: Item<Uint128> = Item::new("rate");
pub const SWAP_ENABLED: Item<bool> = Item::new("swap_enabled");

pub const BALANCES: Map<&Addr, Uint128> = Map::new("balances");
pub const ALLOWANCES: Map<(&Addr, &Addr), Uint128> = Map::new("allowances");
pub const TOTAL_SUPPLY: Item<Uint128> = Item::new("total_supply");

pub const DECIMAL_FACTOR: u128 = 1_000_000_000_000_000_000u128;
pub const INITIAL_SUPPLY: u128 = 1_000u128 * DECIMAL_FACTOR;
pub const SWAP_RATE: u128 = 85u128;
