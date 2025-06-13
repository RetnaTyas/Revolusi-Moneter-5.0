use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg};
use cw2::set_contract_version;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, WrappedResponse};
use crate::state::{WrappedInfo, GOAT_NFT, GOAT_TOKEN, OWNER, WRAPPED};

pub mod msg;
pub mod state;

const CONTRACT_NAME: &str = "goatnftwrapper";
const CONTRACT_VERSION: &str = "0.1.0";

const GOAT_WRAP_RATE: u64 = 85;
const WEIGHT_DECIMALS: u32 = 1;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;
    GOAT_NFT.save(deps.storage, &deps.api.addr_validate(&msg.nft_contract)?)?;
    GOAT_TOKEN.save(deps.storage, &deps.api.addr_validate(&msg.goat_contract)?)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Wrap { token_id } => execute_wrap(deps, env, info, token_id),
        ExecuteMsg::Unwrap { token_id } => execute_unwrap(deps, env, info, token_id),
    }
}

fn execute_wrap(deps: DepsMut, env: Env, info: MessageInfo, token_id: u64) -> StdResult<Response> {
    let nft = GOAT_NFT.load(deps.storage)?;
    let goat = GOAT_TOKEN.load(deps.storage)?;

    let owner_query = to_json_binary(&goatnft::msg::QueryMsg::Owner { token_id })?;
    let owner: String = deps.querier.query_wasm_smart(nft.clone(), &owner_query)?;
    if owner != info.sender {
        return Err(StdError::generic_err("Not token owner"));
    }

    let value_query = to_json_binary(&goatnft::msg::QueryMsg::GoatValue { token_id })?;
    let value: goatnft::msg::GoatValueResponse =
        deps.querier.query_wasm_smart(nft.clone(), &value_query)?;

    let goat_amount = Uint128::new(
        value.value.u128() * 1_000_000_000_000_000_000u128
            / GOAT_WRAP_RATE as u128
            / 10u128.pow(WEIGHT_DECIMALS),
    );

    let transfer = WasmMsg::Execute {
        contract_addr: nft.to_string(),
        msg: to_json_binary(&goatnft::msg::ExecuteMsg::TransferFrom {
            owner: info.sender.to_string(),
            to: env.contract.address.to_string(),
            token_id: token_id.to_string(),
        })?,
        funds: vec![],
    };

    let mint = WasmMsg::Execute {
        contract_addr: goat.to_string(),
        msg: to_json_binary(&starter::msg::ExecuteMsg::Mint {
            to: info.sender.to_string(),
            amount: goat_amount,
        })?,
        funds: vec![],
    };

    WRAPPED.save(
        deps.storage,
        token_id,
        &WrappedInfo {
            owner: info.sender.clone(),
            goat_amount,
        },
    )?;

    Ok(Response::new()
        .add_message(transfer)
        .add_message(mint)
        .add_attribute("action", "Wrapped")
        .add_attribute("user", info.sender)
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("goat_amount", goat_amount))
}

fn execute_unwrap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: u64,
) -> StdResult<Response> {
    let nft = GOAT_NFT.load(deps.storage)?;
    let goat = GOAT_TOKEN.load(deps.storage)?;
    let info_wrapped = WRAPPED.load(deps.storage, token_id)?;
    if info_wrapped.owner != info.sender {
        return Err(StdError::generic_err("Not owner"));
    }

    let burn = WasmMsg::Execute {
        contract_addr: goat.to_string(),
        msg: to_json_binary(&starter::msg::ExecuteMsg::BurnFrom {
            from: info.sender.to_string(),
            amount: info_wrapped.goat_amount,
        })?,
        funds: vec![],
    };

    let transfer = WasmMsg::Execute {
        contract_addr: nft.to_string(),
        msg: to_json_binary(&goatnft::msg::ExecuteMsg::Transfer {
            to: info.sender.to_string(),
            token_id: token_id.to_string(),
        })?,
        funds: vec![],
    };

    WRAPPED.remove(deps.storage, token_id);

    Ok(Response::new()
        .add_message(burn)
        .add_message(transfer)
        .add_attribute("action", "Unwrapped")
        .add_attribute("user", info.sender)
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("goat_amount", info_wrapped.goat_amount))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Wrapped { token_id } => {
            let info = WRAPPED.load(deps.storage, token_id)?;
            Ok(to_json_binary(&WrappedResponse {
                owner: info.owner.into_string(),
                goat_amount: info.goat_amount,
            })?)
        }
        QueryMsg::Owner {} => {
            let owner = OWNER.load(deps.storage)?;
            Ok(to_json_binary(&owner.into_string())?)
        }
    }
}
