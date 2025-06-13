use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const NAME: &str = "Guardian of Agricultural Trade";
pub const SYMBOL: &str = "GOAT";
pub const DECIMALS: u8 = 18;

pub const OWNER: Item<Addr> = Item::new("owner");
pub const WRAPPER_CONTRACT: Item<Option<Addr>> = Item::new("wrapper_contract");

pub const BALANCES: Map<&Addr, Uint128> = Map::new("balances");
pub const ALLOWANCES: Map<(&Addr, &Addr), Uint128> = Map::new("allowances");
pub const TOTAL_SUPPLY: Item<Uint128> = Item::new("total_supply");

pub const STAKING_BALANCE: Map<&Addr, Uint128> = Map::new("staking_balance");
pub const LAST_STAKED_TIME: Map<&Addr, u64> = Map::new("last_staked_time");

pub const REWARD_RATE: Item<Uint128> = Item::new("reward_rate");
pub const REWARD_INTERVAL: Item<u64> = Item::new("reward_interval");
pub const MIN_CLAIM_INTERVAL: Item<u64> = Item::new("min_claim_interval");

pub const REWARD_PRECISION: Uint128 = Uint128::new(1_000_000_000_000_000_000);
