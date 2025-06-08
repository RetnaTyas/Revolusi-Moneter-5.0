use cosmwasm_std::{
    entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CONTRACT_NAME: &str = "ratehandler";
const CONTRACT_VERSION: &str = "0.1.0";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateRate { new_rate: Uint128 },
    InvalidateRate {},
    TransferOwnership { new_owner: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetRate {},
    Owner {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RateResponse {
    pub rate: Uint128,
    pub last_update: u64,
    pub valid: bool,
}

pub const OWNER: Item<Addr> = Item::new("owner");
pub const DYNAMIC_RATE: Item<Uint128> = Item::new("dynamic_rate");
pub const LAST_UPDATE: Item<u64> = Item::new("last_update");
pub const VALID: Item<bool> = Item::new("valid");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;
    DYNAMIC_RATE.save(deps.storage, &Uint128::zero())?;
    LAST_UPDATE.save(deps.storage, &0u64)?;
    VALID.save(deps.storage, &false)?;
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
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UpdateRate { new_rate } => execute_update_rate(deps, env, info, new_rate),
        ExecuteMsg::InvalidateRate {} => execute_invalidate_rate(deps, info),
        ExecuteMsg::TransferOwnership { new_owner } => {
            execute_transfer_ownership(deps, info, new_owner)
        }
    }
}

fn execute_update_rate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_rate: Uint128,
) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    if new_rate.is_zero() {
        return Err(StdError::generic_err("Rate must be > 0"));
    }
    DYNAMIC_RATE.save(deps.storage, &new_rate)?;
    LAST_UPDATE.save(deps.storage, &env.block.time.seconds())?;
    VALID.save(deps.storage, &true)?;
    Ok(Response::new())
}

fn execute_invalidate_rate(mut deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    VALID.save(deps.storage, &false)?;
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
        QueryMsg::GetRate {} => {
            let rate = DYNAMIC_RATE.load(deps.storage)?;
            let last = LAST_UPDATE.load(deps.storage)?;
            let valid = VALID.load(deps.storage)?;
            to_json_binary(&RateResponse {
                rate,
                last_update: last,
                valid,
            })
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
