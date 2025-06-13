use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const OWNER: Item<Addr> = Item::new("owner");
pub const GOAT_NFT: Item<Addr> = Item::new("goat_nft");
pub const MEAT: Item<Addr> = Item::new("meat");
