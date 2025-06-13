use cosmwasm_std::{
    entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CONTRACT_NAME: &str = "ratehandler";
const CONTRACT_VERSION: &str = "0.1.0";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetCommodityRepresentation {
        commodity_id: String,
        data: CommodityRepresentation,
    },
    TransferOwnership { new_owner: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetLODPerDay { commodity_id: String, layer: String },
    GetRate {
        from_commodity: String,
        from_layer: String,
        to_commodity: String,
        to_layer: String,
    },
    Owner {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RateResponse {
    pub rate: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LODResponse {
    pub lod_per_day: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CommodityRepresentation {
    pub nft_address: String,
    pub token_virtual_address: String,
    pub token_product_address: String,
    pub token_product_subtype: String,
    pub is_nft_active: bool,
    pub is_token_virtual_active: bool,
    pub is_token_product_active: bool,
    pub lod_per_day_nft: Uint128,
    pub lod_per_day_virtual: Uint128,
    pub lod_per_day_product: Uint128,
    pub protein_g_per_kg: Uint128,
    pub fat_g_per_kg: Uint128,
    pub micronutrient_index_x1000: Uint128,
    pub yield_per_cycle_kg: Uint128,
    pub cycle_time_days: Uint128,
}

pub const OWNER: Item<Addr> = Item::new("owner");
pub const COMMODITY_REGISTRY: Map<&[u8], CommodityRepresentation> = Map::new("registry");
pub const DECIMAL_FACTOR: u128 = 1_000_000_000_000_000_000u128;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

fn only_owner(deps: DepsMut, info: &MessageInfo) -> StdResult<()> {
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner {
        return Err(StdError::generic_err("Not the owner"));
    }
    Ok(())
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SetCommodityRepresentation { commodity_id, data } => {
            execute_set_commodity_representation(deps, info, commodity_id, data)
        }
        ExecuteMsg::TransferOwnership { new_owner } => {
            execute_transfer_ownership(deps, info, new_owner)
        }
    }
}


fn execute_set_commodity_representation(
    mut deps: DepsMut,
    info: MessageInfo,
    commodity_id: String,
    data: CommodityRepresentation,
) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    COMMODITY_REGISTRY.save(deps.storage, commodity_id.as_bytes(), &data)?;
    Ok(Response::new())
}

fn execute_transfer_ownership(
    mut deps: DepsMut,
    info: MessageInfo,
    new_owner: String,
) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    let addr = deps.api.addr_validate(&new_owner)?;
    OWNER.save(deps.storage, &addr)?;
    Ok(Response::new())
}

fn handle_query(deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetLODPerDay { commodity_id, layer } => {
            let cr = COMMODITY_REGISTRY.load(deps.storage, commodity_id.as_bytes())?;
            let lod = match layer.as_str() {
                "NFT" => cr.lod_per_day_nft,
                "VIRTUAL" => cr.lod_per_day_virtual,
                "PRODUCT" => cr.lod_per_day_product,
                _ => return Err(StdError::generic_err("Invalid layer")),
            };
            to_json_binary(&LODResponse { lod_per_day: lod })
        }
        QueryMsg::GetRate {
            from_commodity,
            from_layer,
            to_commodity,
            to_layer,
        } => {
            if from_layer != "PRODUCT" {
                return Err(StdError::generic_err("FROM layer must be PRODUCT"));
            }
            if to_layer != "PRODUCT" {
                return Err(StdError::generic_err("TO layer must be PRODUCT"));
            }
            let from_cr = COMMODITY_REGISTRY.load(deps.storage, from_commodity.as_bytes())?;
            let to_cr = COMMODITY_REGISTRY.load(deps.storage, to_commodity.as_bytes())?;
            let from_lod = from_cr.lod_per_day_product;
            let to_lod = to_cr.lod_per_day_product;

            if from_lod.is_zero() {
                return Err(StdError::generic_err("Invalid FROM LOD"));
            }
            if to_lod.is_zero() {
                return Err(StdError::generic_err("Invalid TO LOD"));
            }

            let rate = Uint128::from((from_lod.u128() * DECIMAL_FACTOR) / to_lod.u128());
            to_json_binary(&RateResponse { rate })
        }
        QueryMsg::Owner {} => {
            let owner = OWNER.load(deps.storage)?;
            to_json_binary(&owner.into_string())
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    handle_query(deps, msg)
}
