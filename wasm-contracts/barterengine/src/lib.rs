use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
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
        ExecuteMsg::SetRateHandler { rate_handler } => execute_set_rate_handler(deps, info, rate_handler),
        ExecuteMsg::SetMeat { meat } => execute_set_meat(deps, info, meat),
        ExecuteMsg::BarterProductToProduct { from_subtype, to_subtype, from_amount } => {
            execute_barter(deps, env, info, from_subtype, to_subtype, from_amount)
        }
        ExecuteMsg::EmergencyWithdraw { subtype: _ } => only_owner(deps, &info).map(|_| Response::new()),
    }
}

fn execute_set_rate_handler(mut deps: DepsMut, info: MessageInfo, rate_handler: String) -> StdResult<Response> {
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

fn execute_barter(deps: DepsMut, _env: Env, _info: MessageInfo, from_sub: String, to_sub: String, from_amount: u128) -> StdResult<Response> {
    if from_sub == to_sub {
        return Err(StdError::generic_err("Cannot swap same subtype"));
    }
    if from_amount == 0 {
        return Err(StdError::generic_err("Amount must be > 0"));
    }
    let rate_handler = RATE_HANDLER.load(deps.storage)?;
    let rate_resp: RateHandlerResponse = deps
        .querier
        .query_wasm_smart(rate_handler, &RateQueryMsg::GetRate {})?;
    if rate_resp.rate.is_zero() {
        return Err(StdError::generic_err("Invalid barter rate"));
    }
    let to_amount = Uint128::new(from_amount) * rate_resp.rate / Uint128::new(1_000_000_000_000_000_000u128);
    Ok(Response::new()
        .add_attribute("action", "barter")
        .add_attribute("from", from_sub)
        .add_attribute("to", to_sub)
        .add_attribute("from_amount", from_amount.to_string())
        .add_attribute("to_amount", to_amount))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRate { from_subtype: _, to_subtype: _ } => {
            let addr = RATE_HANDLER.load(deps.storage)?;
            let resp: RateHandlerResponse = deps
                .querier
                .query_wasm_smart(addr, &RateQueryMsg::GetRate {})?;
            Ok(to_binary(&RateResponse { rate: resp.rate.u128() })?)
        }
        QueryMsg::Owner {} => {
            let owner = OWNER.load(deps.storage)?;
            Ok(to_binary(&owner.into_string())?)
        }
    }
}
