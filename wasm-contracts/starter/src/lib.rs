use cosmwasm_std::{entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use cw2::set_contract_version;
use crate::msg::{
    InstantiateMsg,
    ExecuteMsg,
    QueryMsg,
    BalanceResponse,
    AllowanceResponse,
    TokenInfoResponse,
    StakingInfoResponse,
    PendingRewardResponse,
    NextClaimResponse,

};
use crate::state::*;

pub mod msg;
pub mod state;

const CONTRACT_NAME: &str = "starter";
const CONTRACT_VERSION: &str = "0.1.0";

#[entry_point]
pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, _msg: InstantiateMsg) -> StdResult<Response> {
    let owner = info.sender.clone();
    OWNER.save(deps.storage, &owner)?;
    WRAPPER_CONTRACT.save(deps.storage, &None)?;
    TOTAL_SUPPLY.save(deps.storage, &Uint128::zero())?;
    REWARD_RATE.save(deps.storage, &Uint128::new(5_000_000_000_000_000_000))?;
    REWARD_INTERVAL.save(deps.storage, &(365u64 * 24 * 60 * 60))?;
    MIN_CLAIM_INTERVAL.save(deps.storage, &(7u64 * 24 * 60 * 60))?;
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

fn only_wrapper(deps: DepsMut, info: &MessageInfo) -> StdResult<()> {
    let wrapper = WRAPPER_CONTRACT.load(deps.storage)?.ok_or_else(|| StdError::generic_err("Wrapper not set"))?;
    if info.sender != wrapper {
        return Err(StdError::generic_err("Unauthorized"));
    }
    Ok(())
}

fn add_balance(store: &mut dyn cosmwasm_std::Storage, addr: &Addr, amount: Uint128) -> StdResult<()> {
    let bal = BALANCES.may_load(store, addr)?.unwrap_or_default() + amount;
    BALANCES.save(store, addr, &bal)
}

fn sub_balance(store: &mut dyn cosmwasm_std::Storage, addr: &Addr, amount: Uint128) -> StdResult<()> {
    let bal = BALANCES.may_load(store, addr)?.unwrap_or_default();
    if bal < amount {
        return Err(StdError::generic_err("Insufficient balance"));
    }
    BALANCES.save(store, addr, &(bal - amount))
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => execute_transfer(deps, info, recipient, amount),
        ExecuteMsg::Approve { spender, amount } => execute_approve(deps, info, spender, amount),
        ExecuteMsg::TransferFrom { owner, recipient, amount } => execute_transfer_from(deps, info, owner, recipient, amount),
        ExecuteMsg::Mint { to, amount } => execute_mint(deps, info, to, amount),
        ExecuteMsg::BurnFrom { from, amount } => execute_burn_from(deps, info, from, amount),
        ExecuteMsg::Stake { amount } => execute_stake(deps, env, info, amount),
        ExecuteMsg::EmergencyUnstake {} => execute_emergency_unstake(deps, env, info),
        ExecuteMsg::Unstake {} => execute_unstake(deps, env, info),
        ExecuteMsg::ClaimReward {} => execute_claim_reward(deps, env, info),
        ExecuteMsg::CompoundReward {} => execute_compound_reward(deps, env, info),
        ExecuteMsg::SetWrapperContract { wrapper_address } => execute_set_wrapper_contract(deps, info, wrapper_address),
        ExecuteMsg::SetRewardConfig { new_rate, new_interval, new_min_claim } => execute_set_reward_config(deps, info, new_rate, new_interval, new_min_claim),
    }
}

fn execute_transfer(deps: DepsMut, info: MessageInfo, recipient: String, amount: Uint128) -> StdResult<Response> {
    let recipient = deps.api.addr_validate(&recipient)?;
    sub_balance(deps.storage, &info.sender, amount)?;
    add_balance(deps.storage, &recipient, amount)?;
    Ok(Response::new())
}

fn execute_approve(deps: DepsMut, info: MessageInfo, spender: String, amount: Uint128) -> StdResult<Response> {
    let spender = deps.api.addr_validate(&spender)?;
    ALLOWANCES.save(deps.storage, (&info.sender, &spender), &amount)?;
    Ok(Response::new())
}

fn execute_transfer_from(deps: DepsMut, info: MessageInfo, owner: String, recipient: String, amount: Uint128) -> StdResult<Response> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let recipient = deps.api.addr_validate(&recipient)?;
    let mut allowance = ALLOWANCES.may_load(deps.storage, (&owner_addr, &info.sender))?.unwrap_or_default();
    if allowance < amount {
        return Err(StdError::generic_err("Allowance exceeded"));
    }
    allowance -= amount;
    ALLOWANCES.save(deps.storage, (&owner_addr, &info.sender), &allowance)?;
    sub_balance(deps.storage, &owner_addr, amount)?;
    add_balance(deps.storage, &recipient, amount)?;
    Ok(Response::new())
}

fn execute_burn_from(mut deps: DepsMut, info: MessageInfo, from: String, amount: Uint128) -> StdResult<Response> {
    only_wrapper(deps.branch(), &info)?;
    let from_addr = deps.api.addr_validate(&from)?;
    sub_balance(deps.storage, &from_addr, amount)?;
    let supply = TOTAL_SUPPLY.load(deps.storage)?.checked_sub(amount)?;
    TOTAL_SUPPLY.save(deps.storage, &supply)?;
    Ok(Response::new())
}

fn execute_set_wrapper_contract(mut deps: DepsMut, info: MessageInfo, wrapper_address: String) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    let addr = deps.api.addr_validate(&wrapper_address)?;
    WRAPPER_CONTRACT.save(deps.storage, &Some(addr))?;
    Ok(Response::new())
}

fn execute_mint(mut deps: DepsMut, info: MessageInfo, to: String, amount: Uint128) -> StdResult<Response> {
    only_wrapper(deps.branch(), &info)?;
    let to = deps.api.addr_validate(&to)?;
    add_balance(deps.storage, &to, amount)?;
    let supply = TOTAL_SUPPLY.load(deps.storage)? + amount;
    TOTAL_SUPPLY.save(deps.storage, &supply)?;
    Ok(Response::new())
}


fn execute_set_reward_config(mut deps: DepsMut, info: MessageInfo, new_rate: Uint128, new_interval: u64, new_min_claim: u64) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    REWARD_RATE.save(deps.storage, &new_rate)?;
    REWARD_INTERVAL.save(deps.storage, &new_interval)?;
    MIN_CLAIM_INTERVAL.save(deps.storage, &new_min_claim)?;
    Ok(Response::new())
}

fn calculate_reward(deps: Deps, env: &Env, addr: &Addr) -> StdResult<Uint128> {
    let staked = STAKING_BALANCE.may_load(deps.storage, addr)?.unwrap_or_default();
    if staked.is_zero() {
        return Ok(Uint128::zero());
    }
    let last = LAST_STAKED_TIME.load(deps.storage, addr)?;
    let duration = env.block.time.seconds().saturating_sub(last) as u128;
    let rate = REWARD_RATE.load(deps.storage)?.u128();
    let interval = REWARD_INTERVAL.load(deps.storage)? as u128;
    let reward = staked.u128() * duration * rate / interval / REWARD_PRECISION.u128();
    Ok(Uint128::new(reward))
}

fn execute_stake(deps: DepsMut, env: Env, info: MessageInfo, amount: Uint128) -> StdResult<Response> {
    if amount.is_zero() {
        return Err(StdError::generic_err("Amount must be > 0"));
    }
    sub_balance(deps.storage, &info.sender, amount)?;
    add_balance(deps.storage, &env.contract.address, amount)?;
    STAKING_BALANCE.update(deps.storage, &info.sender, |b| -> StdResult<_> { Ok(b.unwrap_or_default() + amount) })?;
    LAST_STAKED_TIME.save(deps.storage, &info.sender, &env.block.time.seconds())?;
    Ok(Response::new()
        .add_attribute("action", "Staked")
        .add_attribute("user", info.sender)
        .add_attribute("amount", amount))
}

fn execute_emergency_unstake(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    let staked = STAKING_BALANCE.may_load(deps.storage, &info.sender)?.unwrap_or_default();
    if staked.is_zero() {
        return Err(StdError::generic_err("Nothing to unstake"));
    }
    STAKING_BALANCE.remove(deps.storage, &info.sender);
    LAST_STAKED_TIME.remove(deps.storage, &info.sender);

    let contract = env.contract.address;
    let available = BALANCES.may_load(deps.storage, &contract)?.unwrap_or_default();
    if available >= staked {
        sub_balance(deps.storage, &contract, staked)?;
        add_balance(deps.storage, &info.sender, staked)?;
    } else {
        if !available.is_zero() {
            sub_balance(deps.storage, &contract, available)?;
            add_balance(deps.storage, &info.sender, available)?;
        }
        let diff = staked.checked_sub(available)?;
        add_balance(deps.storage, &info.sender, diff)?;
        let supply = TOTAL_SUPPLY.load(deps.storage)? + diff;
        TOTAL_SUPPLY.save(deps.storage, &supply)?;
    }
    Ok(Response::new()
        .add_attribute("action", "EmergencyUnstaked")
        .add_attribute("user", info.sender)
        .add_attribute("amount", staked))
}

fn execute_unstake(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    let staked = STAKING_BALANCE.may_load(deps.storage, &info.sender)?.unwrap_or_default();
    if staked.is_zero() {
        return Err(StdError::generic_err("Nothing to unstake"));
    }
    let last = LAST_STAKED_TIME.load(deps.storage, &info.sender)?;
    let min = MIN_CLAIM_INTERVAL.load(deps.storage)?;
    if env.block.time.seconds().saturating_sub(last) < min {
        return Err(StdError::generic_err("Claim not allowed yet"));
    }
    let reward = calculate_reward(deps.as_ref(), &env, &info.sender)?;
    STAKING_BALANCE.remove(deps.storage, &info.sender);
    LAST_STAKED_TIME.remove(deps.storage, &info.sender);
    let total = staked + reward;
    let contract = env.contract.address;
    let available = BALANCES.may_load(deps.storage, &contract)?.unwrap_or_default();
    if available >= total {
        sub_balance(deps.storage, &contract, total)?;
        add_balance(deps.storage, &info.sender, total)?;
    } else {
        if !available.is_zero() {
            sub_balance(deps.storage, &contract, available)?;
            add_balance(deps.storage, &info.sender, available)?;
        }
        let diff = total.checked_sub(available)?;
        add_balance(deps.storage, &info.sender, diff)?;
        let supply = TOTAL_SUPPLY.load(deps.storage)? + diff;
        TOTAL_SUPPLY.save(deps.storage, &supply)?;
    }
    Ok(Response::new()
        .add_attribute("action", "Unstaked")
        .add_attribute("user", info.sender)
        .add_attribute("amount", staked)
        .add_attribute("reward", reward))
}

fn execute_claim_reward(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    let reward = calculate_reward(deps.as_ref(), &env, &info.sender)?;
    if reward.is_zero() {
        return Err(StdError::generic_err("No reward to claim"));
    }
    let last = LAST_STAKED_TIME.load(deps.storage, &info.sender)?;
    let min = MIN_CLAIM_INTERVAL.load(deps.storage)?;
    if env.block.time.seconds().saturating_sub(last) < min {
        return Err(StdError::generic_err("Claim not allowed yet"));
    }
    let contract = env.contract.address;
    let available = BALANCES.may_load(deps.storage, &contract)?.unwrap_or_default();
    if available >= reward {
        sub_balance(deps.storage, &contract, reward)?;
        add_balance(deps.storage, &info.sender, reward)?;
    } else {
        if !available.is_zero() {
            sub_balance(deps.storage, &contract, available)?;
            add_balance(deps.storage, &info.sender, available)?;
        }
        let diff = reward.checked_sub(available)?;
        add_balance(deps.storage, &info.sender, diff)?;
        let supply = TOTAL_SUPPLY.load(deps.storage)? + diff;
        TOTAL_SUPPLY.save(deps.storage, &supply)?;
    }
    LAST_STAKED_TIME.save(deps.storage, &info.sender, &env.block.time.seconds())?;
    Ok(Response::new()
        .add_attribute("action", "RewardClaimed")
        .add_attribute("user", info.sender)
        .add_attribute("amount", reward))
}

fn execute_compound_reward(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    let reward = calculate_reward(deps.as_ref(), &env, &info.sender)?;
    if reward.is_zero() {
        return Err(StdError::generic_err("No reward to compound"));
    }
    let last = LAST_STAKED_TIME.load(deps.storage, &info.sender)?;
    let min = MIN_CLAIM_INTERVAL.load(deps.storage)?;
    if env.block.time.seconds().saturating_sub(last) < min {
        return Err(StdError::generic_err("Claim not allowed yet"));
    }
    let contract = env.contract.address;
    let available = BALANCES.may_load(deps.storage, &contract)?.unwrap_or_default();
    if available < reward {
        let diff = reward.checked_sub(available)?;
        add_balance(deps.storage, &contract, diff)?;
        let supply = TOTAL_SUPPLY.load(deps.storage)? + diff;
        TOTAL_SUPPLY.save(deps.storage, &supply)?;
    }
    STAKING_BALANCE.update(deps.storage, &info.sender, |b| -> StdResult<_> { Ok(b.unwrap_or_default() + reward) })?;
    LAST_STAKED_TIME.save(deps.storage, &info.sender, &env.block.time.seconds())?;
    Ok(Response::new()
        .add_attribute("action", "Compounded")
        .add_attribute("user", info.sender)
        .add_attribute("amount", reward))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => {
            let addr = deps.api.addr_validate(&address)?;
            let balance = BALANCES.may_load(deps.storage, &addr)?.unwrap_or_default();
            to_json_binary(&BalanceResponse { balance })
        }
        QueryMsg::Allowance { owner, spender } => {
            let owner = deps.api.addr_validate(&owner)?;
            let spender = deps.api.addr_validate(&spender)?;
            let allowance = ALLOWANCES.may_load(deps.storage, (&owner, &spender))?.unwrap_or_default();
            to_json_binary(&AllowanceResponse { allowance })
        }
        QueryMsg::TokenInfo {} => {
            let total_supply = TOTAL_SUPPLY.load(deps.storage)?;
            to_json_binary(&TokenInfoResponse { name: NAME.to_string(), symbol: SYMBOL.to_string(), decimals: DECIMALS, total_supply })
        }
        QueryMsg::StakingBalance { address } => {
            let addr = deps.api.addr_validate(&address)?;
            let balance = STAKING_BALANCE.may_load(deps.storage, &addr)?.unwrap_or_default();
            let last = LAST_STAKED_TIME.may_load(deps.storage, &addr)?.unwrap_or_default();
            to_json_binary(&StakingInfoResponse { balance, last_staked: last })
        }
        QueryMsg::PendingReward { address } => {
            let addr = deps.api.addr_validate(&address)?;
            let reward = calculate_reward(deps, &env, &addr)?;
            to_json_binary(&PendingRewardResponse { reward })
        }
        QueryMsg::NextClaimTime { address } => {
            let addr = deps.api.addr_validate(&address)?;
            let last = LAST_STAKED_TIME.may_load(deps.storage, &addr)?.unwrap_or_default();
            if last == 0 {
                return to_json_binary(&NextClaimResponse { timestamp: 0 });
            }
            let min = MIN_CLAIM_INTERVAL.load(deps.storage)?;
            to_json_binary(&NextClaimResponse { timestamp: last + min })
        }
        QueryMsg::Owner {} => {
            let owner = OWNER.load(deps.storage)?;
            to_json_binary(&owner.into_string())
        }
    }
}
