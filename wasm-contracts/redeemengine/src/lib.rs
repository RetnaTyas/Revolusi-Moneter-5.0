use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128, WasmMsg,
};
use cw2::set_contract_version;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, RedeemConfigResponse};
use crate::state::{RedeemConfig, CONFIGS, MEAT, OWNER};

pub mod msg;
pub mod state;

const CONTRACT_NAME: &str = "redeemengine";
const CONTRACT_VERSION: &str = "0.1.0";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;
    MEAT.save(deps.storage, &deps.api.addr_validate(&msg.meat_contract)?)?;
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
        ExecuteMsg::SetRedeemConfig {
            subtype,
            grams_per_token_unit,
            active,
        } => execute_set_config(deps, info, subtype, grams_per_token_unit, active),
        ExecuteMsg::Redeem { subtype, amount } => {
            execute_redeem(deps, env.clone(), info, subtype, amount)
        }
        ExecuteMsg::EmergencyWithdrawMEATSubtype { subtype } => {
            execute_withdraw(deps, env, info, subtype)
        }
    }
}

fn execute_set_config(
    mut deps: DepsMut,
    info: MessageInfo,
    subtype: String,
    grams: u128,
    active: bool,
) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    if subtype.is_empty() {
        return Err(StdError::generic_err("Invalid subtype"));
    }
    CONFIGS.save(
        deps.storage,
        subtype.as_bytes(),
        &RedeemConfig {
            grams_per_token_unit: grams,
            is_active: active,
        },
    )?;
    Ok(Response::new())
}

fn execute_redeem(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    subtype: String,
    amount: u128,
) -> StdResult<Response> {
    if amount == 0 {
        return Err(StdError::generic_err("Invalid amount"));
    }
    let cfg = CONFIGS.load(deps.storage, subtype.as_bytes())?;
    if !cfg.is_active {
        return Err(StdError::generic_err("Redeem inactive"));
    }
    let meat = MEAT.load(deps.storage)?;
    let bal: meat::msg::BalanceSubtypeWithLineageResponse = deps.querier.query_wasm_smart(
        meat.clone(),
        &meat::msg::QueryMsg::BalanceOfSubtypeWithLineage {
            user: info.sender.to_string(),
            subtype: subtype.clone(),
        },
    )?;
    if bal.balance < Uint128::new(amount) {
        return Err(StdError::generic_err("Insufficient subtype balance"));
    }
    if bal.lineage_id == 0 {
        return Err(StdError::generic_err("Lineage not set"));
    }
    let burn = WasmMsg::Execute {
        contract_addr: meat.to_string(),
        msg: to_json_binary(&meat::msg::ExecuteMsg::BurnSubtype {
            from: info.sender.to_string(),
            subtype: subtype.clone(),
            amount: Uint128::new(amount),
        })?,
        funds: vec![],
    };
    let grams = amount * cfg.grams_per_token_unit / 1_000_000_000_000_000_000u128;
    Ok(Response::new()
        .add_message(burn)
        .add_attribute("action", "redeem")
        .add_attribute("user", info.sender)
        .add_attribute("subtype", subtype)
        .add_attribute("lineage_id", bal.lineage_id.to_string())
        .add_attribute("amount", amount.to_string())
        .add_attribute("grams", grams.to_string()))
}

fn execute_withdraw(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    subtype: String,
) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    if subtype.is_empty() {
        return Err(StdError::generic_err("Invalid subtype"));
    }
    let meat = MEAT.load(deps.storage)?;
    let bal: meat::msg::BalanceSubtypeWithLineageResponse = deps.querier.query_wasm_smart(
        meat.clone(),
        &meat::msg::QueryMsg::BalanceOfSubtypeWithLineage {
            user: env.contract.address.to_string(),
            subtype: subtype.clone(),
        },
    )?;
    if bal.balance.is_zero() {
        return Err(StdError::generic_err("No subtype balance"));
    }
    let burn = WasmMsg::Execute {
        contract_addr: meat.to_string(),
        msg: to_json_binary(&meat::msg::ExecuteMsg::BurnSubtype {
            from: env.contract.address.to_string(),
            subtype: subtype.clone(),
            amount: bal.balance,
        })?,
        funds: vec![],
    };
    let mint = WasmMsg::Execute {
        contract_addr: meat.to_string(),
        msg: to_json_binary(&meat::msg::ExecuteMsg::MintSubtype {
            to: info.sender.to_string(),
            subtype: subtype.clone(),
            amount: bal.balance,
        })?,
        funds: vec![],
    };
    Ok(Response::new()
        .add_message(burn)
        .add_message(mint)
        .add_attribute("action", "emergency_withdraw_meat_subtype")
        .add_attribute("subtype", subtype)
        .add_attribute("amount", bal.balance))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::RedeemConfig { subtype } => {
            let cfg = CONFIGS.load(deps.storage, subtype.as_bytes())?;
            to_json_binary(&RedeemConfigResponse {
                grams_per_token_unit: cfg.grams_per_token_unit,
                is_active: cfg.is_active,
            })
        }
        QueryMsg::Owner {} => {
            let owner = OWNER.load(deps.storage)?;
            to_json_binary(&owner.into_string())
        }
    }
}
