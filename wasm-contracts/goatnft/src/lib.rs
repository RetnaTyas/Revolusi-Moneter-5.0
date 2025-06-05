use cosmwasm_std::{entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use base64;
use cw721;
use serde_json_wasm;
use cw2::set_contract_version;

use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, GoatValueResponse};
use crate::state::*;

pub mod msg;
pub mod state;

const CONTRACT_NAME: &str = "goatnft";
const CONTRACT_VERSION: &str = "0.1.0";

#[entry_point]
pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, _msg: InstantiateMsg) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;
    NEXT_ID.save(deps.storage, &0u64)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

fn only_owner(deps: DepsMut, info: &MessageInfo) -> StdResult<()> {
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner { return Err(StdError::generic_err("Not the owner")); }
    Ok(())
}

fn handle_execute(deps: DepsMut, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Mint { to, value } => execute_mint(deps, info, to, value),
        ExecuteMsg::Burn { token_id } => execute_burn(deps, info, token_id),
    }
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    handle_execute(deps, info, msg)
}

fn execute_mint(mut deps: DepsMut, info: MessageInfo, to: String, value: Uint128) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    if value.is_zero() { return Err(StdError::generic_err("Value must be > 0")); }
    let to_addr = deps.api.addr_validate(&to)?;
    let mut id = NEXT_ID.load(deps.storage)? + 1;
    NEXT_ID.save(deps.storage, &id)?;
    OWNER_OF.save(deps.storage, id, &to_addr)?;
    GOAT_VALUE.save(deps.storage, id, &value)?;
    Ok(Response::new().add_attribute("token_id", id.to_string()))
}

fn execute_burn(mut deps: DepsMut, info: MessageInfo, token_id: String) -> StdResult<Response> {
    let id: u64 = token_id.parse().map_err(|_| StdError::generic_err("invalid id"))?;
    let owner = OWNER_OF.load(deps.storage, id)?;
    if owner != info.sender {
        // allow burning by contracts like GOAT without explicit approval
    }
    OWNER_OF.remove(deps.storage, id);
    GOAT_VALUE.remove(deps.storage, id);
    Ok(Response::new())
}

fn handle_query(deps: Deps, q: QueryMsg) -> StdResult<Binary> {
    match q {
        QueryMsg::GoatValue { token_id } => {
            let value = GOAT_VALUE.load(deps.storage, token_id)?;
            to_binary(&GoatValueResponse { value })
        }
        QueryMsg::Owner { token_id } => {
            let owner = OWNER_OF.load(deps.storage, token_id)?;
            to_binary(&owner.into_string())
        }
        QueryMsg::OwnerAddress {} => {
            let owner = OWNER.load(deps.storage)?;
            to_binary(&owner.into_string())
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: Binary) -> StdResult<Binary> {
    if let Ok(q) = serde_json_wasm::from_slice::<QueryMsg>(&msg) {
        return handle_query(deps, q);
    }
    if let Ok(s) = std::str::from_utf8(&msg) {
        if let Ok(decoded) = base64::decode(s) {
            if let Ok(q) = serde_json_wasm::from_slice::<QueryMsg>(&decoded) {
                return handle_query(deps, q);
            }
        }
    }
    Err(StdError::generic_err("invalid query"))
}
