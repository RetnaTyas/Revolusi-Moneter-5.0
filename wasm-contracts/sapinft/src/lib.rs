use base64::{engine::general_purpose, Engine as _};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};
use cw2::set_contract_version;
use serde_json_wasm;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SapiDataResponse, SapiValueResponse};
use crate::state::*;

pub mod msg;
pub mod state;

const CONTRACT_NAME: &str = "sapinft";
const CONTRACT_VERSION: &str = "0.1.0";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;
    NEXT_ID.save(deps.storage, &0u64)?;
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

fn handle_execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Mint {
            to,
            nfc_id,
            breed,
            birth_year,
            weight,
        } => execute_mint(deps, env, info, to, weight, nfc_id, breed, birth_year),
        ExecuteMsg::Burn { token_id } => execute_burn(deps, env, info, token_id),
        ExecuteMsg::Approve { spender, token_id } => execute_approve(deps, info, spender, token_id),
        ExecuteMsg::Transfer { to, token_id } => execute_transfer(deps, info, to, token_id),
        ExecuteMsg::TransferFrom {
            owner,
            to,
            token_id,
        } => execute_transfer_from(deps, info, owner, to, token_id),
        ExecuteMsg::UpdateWeight {
            token_id,
            new_weight,
        } => execute_update_weight(deps, env, info, token_id, new_weight),
    }
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    handle_execute(deps, env, info, msg)
}

fn execute_mint(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    to: String,
    weight: u64,
    nfc_id: String,
    breed: String,
    birth_year: u64,
) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    if weight == 0 {
        return Err(StdError::generic_err("Weight must be > 0"));
    }
    if NFC_TO_TOKEN
        .may_load(deps.storage, nfc_id.as_bytes())?
        .is_some()
    {
        return Err(StdError::generic_err("NFC ID already used"));
    }
    let to_addr = deps.api.addr_validate(&to)?;
    let id = NEXT_ID.load(deps.storage)? + 1;
    NEXT_ID.save(deps.storage, &id)?;
    OWNER_OF.save(deps.storage, id, &to_addr)?;
    SAPI_VALUE.save(deps.storage, id, &Uint128::from(weight as u128))?;
    let data = SapiData {
        nfc_id: nfc_id.clone(),
        breed,
        birth_year,
        weight,
        minted_at: env.block.time.seconds(),
    };
    SAPI_METADATA.save(deps.storage, id, &data)?;
    LAST_WEIGHT_UPDATE.save(deps.storage, id, &env.block.time.seconds())?;
    NFC_TO_TOKEN.save(deps.storage, nfc_id.as_bytes(), &id)?;
    Ok(Response::new().add_attribute("token_id", id.to_string()))
}

fn execute_burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
) -> StdResult<Response> {
    let id: u64 = token_id
        .parse()
        .map_err(|_| StdError::generic_err("invalid id"))?;
    let owner = OWNER_OF.load(deps.storage, id)?;
    if owner != info.sender {
        let approved = APPROVALS.may_load(deps.storage, id)?;
        match approved {
            Some(addr) if addr == info.sender => {}
            _ => return Err(StdError::generic_err("Unauthorized")),
        }
    }
    let last_update = LAST_WEIGHT_UPDATE.load(deps.storage, id)?;
    if env.block.time.seconds() - last_update > WEIGHT_UPDATE_VALIDITY {
        return Err(StdError::generic_err("Weight update too old"));
    }
    let data = SAPI_METADATA.load(deps.storage, id)?;
    OWNER_OF.remove(deps.storage, id);
    SAPI_VALUE.remove(deps.storage, id);
    SAPI_METADATA.remove(deps.storage, id);
    APPROVALS.remove(deps.storage, id);
    LAST_WEIGHT_UPDATE.remove(deps.storage, id);
    NFC_TO_TOKEN.remove(deps.storage, data.nfc_id.as_bytes());
    Ok(Response::new())
}

fn execute_approve(
    deps: DepsMut,
    info: MessageInfo,
    spender: String,
    token_id: String,
) -> StdResult<Response> {
    let id: u64 = token_id
        .parse()
        .map_err(|_| StdError::generic_err("invalid id"))?;
    let owner = OWNER_OF.load(deps.storage, id)?;
    if owner != info.sender {
        return Err(StdError::generic_err("Unauthorized"));
    }
    let spender_addr = deps.api.addr_validate(&spender)?;
    APPROVALS.save(deps.storage, id, &spender_addr)?;
    Ok(Response::new())
}

fn execute_transfer(
    deps: DepsMut,
    info: MessageInfo,
    to: String,
    token_id: String,
) -> StdResult<Response> {
    let id: u64 = token_id
        .parse()
        .map_err(|_| StdError::generic_err("invalid id"))?;
    let owner = OWNER_OF.load(deps.storage, id)?;
    if owner != info.sender {
        let approved = APPROVALS.may_load(deps.storage, id)?;
        match approved {
            Some(addr) if addr == info.sender => {}
            _ => return Err(StdError::generic_err("Unauthorized")),
        }
    }
    let to_addr = deps.api.addr_validate(&to)?;
    OWNER_OF.save(deps.storage, id, &to_addr)?;
    APPROVALS.remove(deps.storage, id);
    Ok(Response::new())
}

fn execute_transfer_from(
    deps: DepsMut,
    info: MessageInfo,
    owner: String,
    to: String,
    token_id: String,
) -> StdResult<Response> {
    let id: u64 = token_id
        .parse()
        .map_err(|_| StdError::generic_err("invalid id"))?;
    let owner_addr = deps.api.addr_validate(&owner)?;
    let current_owner = OWNER_OF.load(deps.storage, id)?;
    if current_owner != owner_addr {
        return Err(StdError::generic_err("Owner mismatch"));
    }
    if owner_addr != info.sender {
        let approved = APPROVALS.may_load(deps.storage, id)?;
        match approved {
            Some(addr) if addr == info.sender => {}
            _ => return Err(StdError::generic_err("Unauthorized")),
        }
    }
    let to_addr = deps.api.addr_validate(&to)?;
    OWNER_OF.save(deps.storage, id, &to_addr)?;
    APPROVALS.remove(deps.storage, id);
    Ok(Response::new())
}

fn execute_update_weight(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    new_weight: u64,
) -> StdResult<Response> {
    if new_weight == 0 {
        return Err(StdError::generic_err("Weight must be > 0"));
    }
    let id: u64 = token_id
        .parse()
        .map_err(|_| StdError::generic_err("invalid id"))?;
    let owner = OWNER_OF.load(deps.storage, id)?;
    if owner != info.sender {
        return Err(StdError::generic_err("Not token owner"));
    }
    SAPI_VALUE.save(deps.storage, id, &Uint128::from(new_weight as u128))?;
    let mut data = SAPI_METADATA.load(deps.storage, id)?;
    data.weight = new_weight;
    SAPI_METADATA.save(deps.storage, id, &data)?;
    LAST_WEIGHT_UPDATE.save(deps.storage, id, &env.block.time.seconds())?;
    Ok(Response::new())
}

fn handle_query(deps: Deps, q: QueryMsg) -> StdResult<Binary> {
    match q {
        QueryMsg::SapiValue { token_id } => {
            let value = SAPI_VALUE.load(deps.storage, token_id)?;
            to_json_binary(&SapiValueResponse { value })
        }
        QueryMsg::SapiData { token_id } => {
            let data = SAPI_METADATA.load(deps.storage, token_id)?;
            to_json_binary(&SapiDataResponse {
                nfc_id: data.nfc_id,
                breed: data.breed,
                birth_year: data.birth_year,
                weight: data.weight,
                minted_at: data.minted_at,
            })
        }
        QueryMsg::Owner { token_id } => {
            let owner = OWNER_OF.load(deps.storage, token_id)?;
            to_json_binary(&owner.into_string())
        }
        QueryMsg::OwnerAddress {} => {
            let owner = OWNER.load(deps.storage)?;
            to_json_binary(&owner.into_string())
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: Binary) -> StdResult<Binary> {
    if let Ok(q) = serde_json_wasm::from_slice::<QueryMsg>(&msg) {
        return handle_query(deps, q);
    }
    if let Ok(s) = std::str::from_utf8(&msg) {
        if let Ok(decoded) = general_purpose::STANDARD.decode(s) {
            if let Ok(q) = serde_json_wasm::from_slice::<QueryMsg>(&decoded) {
                return handle_query(deps, q);
            }
        }
    }
    Err(StdError::generic_err("invalid query"))
}
