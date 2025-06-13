use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RedeemConfig {
    pub grams_per_token_unit: u128,
    pub is_active: bool,
}

pub const OWNER: Item<Addr> = Item::new("owner");
pub const MEAT: Item<Addr> = Item::new("meat");
pub const CONFIGS: Map<&[u8], RedeemConfig> = Map::new("configs");
