use cosmwasm_std::{
    entry_point, to_binary, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, Uint128, WasmMsg,
};
use cw2::set_contract_version;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, RateResponse};
use crate::state::{MEAT, OWNER, RATE_HANDLER};
use ratehandler::{QueryMsg as RateQueryMsg, RateResponse as RateHandlerResponse};

pub mod msg;
pub mod state;

const CONTRACT_NAME: &str = "barterengine";
const CONTRACT_VERSION: &str = "0.1.0";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;
    RATE_HANDLER.save(deps.storage, &deps.api.addr_validate(&msg.rate_handler)?)?;
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
        ExecuteMsg::SetRateHandler { rate_handler } => {
            execute_set_rate_handler(deps, info, rate_handler)
        }
        ExecuteMsg::SetMeat { meat } => execute_set_meat(deps, info, meat),
        ExecuteMsg::BarterProductToProduct {
            from_subtype,
            to_subtype,
            from_amount,
        } => execute_barter(deps, env, info, from_subtype, to_subtype, from_amount),
        ExecuteMsg::EmergencyWithdrawMEATSubtype { subtype } => {
            execute_withdraw(deps, env, info, subtype)
        }
    }
}

fn execute_set_rate_handler(
    mut deps: DepsMut,
    info: MessageInfo,
    rate_handler: String,
) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    let addr = deps.api.addr_validate(&rate_handler)?;
    RATE_HANDLER.save(deps.storage, &addr)?;
    Ok(Response::new())
}

fn execute_set_meat(mut deps: DepsMut, info: MessageInfo, meat: String) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    let addr = deps.api.addr_validate(&meat)?;
    MEAT.save(deps.storage, &addr)?;
    Ok(Response::new())
}

fn execute_barter(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    from_sub: String,
    to_sub: String,
    from_amount: u128,
) -> StdResult<Response> {
    if from_sub == to_sub {
        return Err(StdError::generic_err("Cannot swap same subtype"));
    }
    if from_amount == 0 {
        return Err(StdError::generic_err("Amount must be > 0"));
    }
    let rate_handler = RATE_HANDLER.load(deps.storage)?;
    let rate_resp: RateHandlerResponse = deps.querier.query_wasm_smart(
        rate_handler,
        &RateQueryMsg::GetRate {
            from_commodity: from_sub.clone(),
            from_layer: "PRODUCT".into(),
            to_commodity: to_sub.clone(),
            to_layer: "PRODUCT".into(),
        },
    )?;
    if rate_resp.rate.is_zero() {
        return Err(StdError::generic_err("Invalid barter rate"));
    }
    if rate_resp.rate.is_zero() {
        return Err(StdError::generic_err("Invalid barter rate"));
    }

    let to_amount =
        Uint128::new(from_amount) * rate_resp.rate / Uint128::new(1_000_000_000_000_000_000u128);

    let meat = MEAT.load(deps.storage)?;
    let bal: meat::msg::BalanceSubtypeWithLineageResponse = deps.querier.query_wasm_smart(
        meat.clone(),
        &meat::msg::QueryMsg::BalanceOfSubtypeWithLineage {
            user: info.sender.to_string(),
            subtype: from_sub.clone(),
        },
    )?;
    if bal.balance < Uint128::new(from_amount) {
        return Err(StdError::generic_err("Insufficient subtype balance"));
    }
    if bal.lineage_id == 0 {
        return Err(StdError::generic_err("Invalid lineage"));
    }

    let burn = WasmMsg::Execute {
        contract_addr: meat.to_string(),
        msg: to_json_binary(&meat::msg::ExecuteMsg::BurnSubtype {
            from: info.sender.to_string(),
            subtype: from_sub.clone(),
            amount: Uint128::new(from_amount),
        })?,
        funds: vec![],
    };

    let mint = WasmMsg::Execute {
        contract_addr: meat.to_string(),
        msg: to_json_binary(&meat::msg::ExecuteMsg::MintSubtype {
            to: info.sender.to_string(),
            subtype: to_sub.clone(),
            amount: to_amount,
        })?,
        funds: vec![],
    };

    let lineage = WasmMsg::Execute {
        contract_addr: meat.to_string(),
        msg: to_json_binary(&meat::msg::ExecuteMsg::SetSubtypeLineage {
            user: info.sender.to_string(),
            subtype: to_sub.clone(),
            lineage_id: bal.lineage_id,
        })?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(burn)
        .add_message(mint)
        .add_message(lineage)
        .add_attribute("action", "barter")
        .add_attribute("from", from_sub)
        .add_attribute("to", to_sub)
        .add_attribute("from_amount", from_amount.to_string())
        .add_attribute("to_amount", to_amount))
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
        QueryMsg::GetRate {
            from_subtype,
            to_subtype,
        } => {
            let addr = RATE_HANDLER.load(deps.storage)?;
            let resp: RateHandlerResponse = deps.querier.query_wasm_smart(
                addr,
                &RateQueryMsg::GetRate {
                    from_commodity: from_subtype,
                    from_layer: "PRODUCT".into(),
                    to_commodity: to_subtype,
                    to_layer: "PRODUCT".into(),
                },
            )?;
            Ok(to_binary(&RateResponse {
                rate: resp.rate.u128(),
            })?)
        }
        QueryMsg::Owner {} => {
            let owner = OWNER.load(deps.storage)?;
            Ok(to_binary(&owner.into_string())?)
        }
    }
}
