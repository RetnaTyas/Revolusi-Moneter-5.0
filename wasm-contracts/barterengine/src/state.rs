use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const OWNER: Item<Addr> = Item::new("owner");
pub const RATE_HANDLER: Item<Addr> = Item::new("rate_handler");
pub const MEAT: Item<Addr> = Item::new("meat");
