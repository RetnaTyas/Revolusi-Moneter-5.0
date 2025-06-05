use cosmwasm_std::{entry_point, to_json_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg, BankMsg};
use cw2::set_contract_version;

use starter::msg as goat_msg;

use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, BalanceResponse, AllowanceResponse, TokenInfoResponse, RateResponse, EnabledResponse, EquivalentResponse};
use crate::state::*;

pub mod msg;
pub mod state;

const CONTRACT_NAME: &str = "meat";
const CONTRACT_VERSION: &str = "0.1.0";

fn add_balance(store: &mut dyn cosmwasm_std::Storage, addr: &Addr, amount: Uint128) -> StdResult<()> {
    let bal = BALANCES.may_load(store, addr)?.unwrap_or_default() + amount;
    BALANCES.save(store, addr, &bal)
}

fn sub_balance(store: &mut dyn cosmwasm_std::Storage, addr: &Addr, amount: Uint128) -> StdResult<()> {
    let bal = BALANCES.may_load(store, addr)?.unwrap_or_default();
    if bal < amount { return Err(StdError::generic_err("Insufficient balance")); }
    BALANCES.save(store, addr, &(bal - amount))
}

#[entry_point]
pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    let owner = info.sender.clone();
    OWNER.save(deps.storage, &owner)?;
    let goat_addr = deps.api.addr_validate(&msg.goat_contract)?;
    GOAT_CONTRACT.save(deps.storage, &goat_addr)?;
    RATE.save(deps.storage, &Uint128::new(100))?;
    SWAP_ENABLED.save(deps.storage, &true)?;
    let init = Uint128::new(INITIAL_SUPPLY);
    BALANCES.save(deps.storage, &owner, &init)?;
    TOTAL_SUPPLY.save(deps.storage, &init)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

fn only_owner(deps: DepsMut, info: &MessageInfo) -> StdResult<()> {
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner { return Err(StdError::generic_err("Not the owner")); }
    Ok(())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => execute_transfer(deps, info, recipient, amount),
        ExecuteMsg::Approve { spender, amount } => execute_approve(deps, info, spender, amount),
        ExecuteMsg::TransferFrom { owner, recipient, amount } => execute_transfer_from(deps, info, owner, recipient, amount),
        ExecuteMsg::MintWithNative {} => execute_mint_with_native(deps, env, info),
        ExecuteMsg::WithdrawNative { to } => execute_withdraw_native(deps, env, info, to),
        ExecuteMsg::ChangeDepositRate { new_rate } => execute_change_rate(deps, info, new_rate),
        ExecuteMsg::SwapGoatForMeat { goat_amount } => execute_swap_goat_for_meat(deps, env, info, goat_amount),
        ExecuteMsg::SwapMeatForGoat { meat_amount } => execute_swap_meat_for_goat(deps, env, info, meat_amount),
        ExecuteMsg::SetSwapEnabled { enabled } => execute_set_swap_enabled(deps, info, enabled),
        ExecuteMsg::SetGoatAddress { goat_address } => execute_set_goat(deps, info, goat_address),
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
    if allowance < amount { return Err(StdError::generic_err("Allowance exceeded")); }
    allowance -= amount;
    ALLOWANCES.save(deps.storage, (&owner_addr, &info.sender), &allowance)?;
    sub_balance(deps.storage, &owner_addr, amount)?;
    add_balance(deps.storage, &recipient, amount)?;
    Ok(Response::new())
}

fn execute_mint_with_native(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    let coin: &Coin = info.funds.first().ok_or_else(|| StdError::generic_err("No funds"))?;
    if coin.amount.is_zero() { return Err(StdError::generic_err("No funds")); }
    let rate = RATE.load(deps.storage)?.u128();
    let meat_amount = coin.amount.u128() * rate / 1000u128;
    let mint = Uint128::new(meat_amount);
    let contract = env.contract.address.clone();
    let mut contract_bal = BALANCES.may_load(deps.storage, &contract)?.unwrap_or_default();
    if contract_bal >= mint {
        contract_bal -= mint;
        BALANCES.save(deps.storage, &contract, &contract_bal)?;
    } else {
        if !contract_bal.is_zero() {
            BALANCES.save(deps.storage, &contract, &Uint128::zero())?;
            add_balance(deps.storage, &info.sender, contract_bal)?;
        }
        let to_mint = mint.checked_sub(contract_bal)?;
        add_balance(deps.storage, &info.sender, to_mint)?;
        let total = TOTAL_SUPPLY.load(deps.storage)? + to_mint;
        TOTAL_SUPPLY.save(deps.storage, &total)?;
        return Ok(Response::new());
    }
    add_balance(deps.storage, &info.sender, mint)?;
    Ok(Response::new())
}

fn execute_withdraw_native(mut deps: DepsMut, env: Env, info: MessageInfo, to: Option<String>) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    let bal = deps.querier.query_all_balances(env.contract.address)?;
    if bal.is_empty() { return Err(StdError::generic_err("No funds")); }
    let to_addr = to.unwrap_or_else(|| info.sender.to_string());
    Ok(Response::new().add_message(BankMsg::Send { to_address: to_addr, amount: bal }))
}

fn execute_change_rate(mut deps: DepsMut, info: MessageInfo, new_rate: Uint128) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    RATE.save(deps.storage, &new_rate)?;
    Ok(Response::new())
}

fn execute_swap_goat_for_meat(deps: DepsMut, env: Env, info: MessageInfo, goat_amount: Uint128) -> StdResult<Response> {
    if !SWAP_ENABLED.load(deps.storage)? { return Err(StdError::generic_err("Swap disabled")); }
    if goat_amount.is_zero() { return Err(StdError::generic_err("Amount must be > 0")); }
    let goat = GOAT_CONTRACT.load(deps.storage)?;
    let transfer = WasmMsg::Execute { contract_addr: goat.to_string(), msg: to_json_binary(&goat_msg::ExecuteMsg::TransferFrom { owner: info.sender.to_string(), recipient: env.contract.address.to_string(), amount: goat_amount })?, funds: vec![] };
    let meat_needed = Uint128::new(goat_amount.u128() * SWAP_RATE);
    let contract = env.contract.address;
    let bal = BALANCES.may_load(deps.storage, &contract)?.unwrap_or_default();
    let resp = Response::new().add_message(transfer);
    if bal >= meat_needed {
        sub_balance(deps.storage, &contract, meat_needed)?;
        add_balance(deps.storage, &info.sender, meat_needed)?;
    } else {
        if !bal.is_zero() {
            sub_balance(deps.storage, &contract, bal)?;
            add_balance(deps.storage, &info.sender, bal)?;
        }
        let mint_amt = meat_needed.checked_sub(bal)?;
        add_balance(deps.storage, &info.sender, mint_amt)?;
        let total = TOTAL_SUPPLY.load(deps.storage)? + mint_amt;
        TOTAL_SUPPLY.save(deps.storage, &total)?;
    }
    Ok(resp)
}

fn execute_swap_meat_for_goat(deps: DepsMut, env: Env, info: MessageInfo, meat_amount: Uint128) -> StdResult<Response> {
    if !SWAP_ENABLED.load(deps.storage)? { return Err(StdError::generic_err("Swap disabled")); }
    if meat_amount.is_zero() { return Err(StdError::generic_err("Amount must be > 0")); }
    let goat = GOAT_CONTRACT.load(deps.storage)?;
    let mut allowance = ALLOWANCES.may_load(deps.storage, (&info.sender, &env.contract.address))?.unwrap_or_default();
    if allowance < meat_amount { return Err(StdError::generic_err("Allowance exceeded")); }
    allowance -= meat_amount;
    ALLOWANCES.save(deps.storage, (&info.sender, &env.contract.address), &allowance)?;
    sub_balance(deps.storage, &info.sender, meat_amount)?;
    add_balance(deps.storage, &env.contract.address, meat_amount)?;
    let goat_amount = Uint128::new(meat_amount.u128() / SWAP_RATE);
    let query = goat_msg::QueryMsg::Balance { address: env.contract.address.to_string() };
    let goat_bal: goat_msg::BalanceResponse = deps.querier.query_wasm_smart(goat.clone(), &query)?;
    let mut resp = Response::new();
    if goat_bal.balance < goat_amount {
        let diff = goat_amount.checked_sub(goat_bal.balance)?;
        resp = resp.add_message(WasmMsg::Execute {
            contract_addr: goat.to_string(),
            msg: to_json_binary(&goat_msg::ExecuteMsg::MintTo { to: env.contract.address.to_string(), amount: diff })?,
            funds: vec![],
        });
    }
    resp = resp.add_message(WasmMsg::Execute {
        contract_addr: goat.to_string(),
        msg: to_json_binary(&goat_msg::ExecuteMsg::Transfer { recipient: info.sender.to_string(), amount: goat_amount })?,
        funds: vec![],
    });
    Ok(resp)
}

fn execute_set_swap_enabled(mut deps: DepsMut, info: MessageInfo, enabled: bool) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    SWAP_ENABLED.save(deps.storage, &enabled)?;
    Ok(Response::new())
}

fn execute_set_goat(mut deps: DepsMut, info: MessageInfo, goat_address: String) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    let addr = deps.api.addr_validate(&goat_address)?;
    GOAT_CONTRACT.save(deps.storage, &addr)?;
    Ok(Response::new())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
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
        QueryMsg::DepositRate {} => {
            let rate = RATE.load(deps.storage)?;
            to_json_binary(&RateResponse { rate })
        }
        QueryMsg::SwapEnabled {} => {
            let enabled = SWAP_ENABLED.load(deps.storage)?;
            to_json_binary(&EnabledResponse { enabled })
        }
        QueryMsg::Owner {} => {
            let owner = OWNER.load(deps.storage)?;
            to_json_binary(&owner.into_string())
        }
        QueryMsg::EquivalentMeat { goat_amount } => {
            let amount = Uint128::new(goat_amount.u128() * SWAP_RATE);
            to_json_binary(&EquivalentResponse { amount })
        }
        QueryMsg::EquivalentGoat { meat_amount } => {
            let amount = Uint128::new(meat_amount.u128() / SWAP_RATE);
            to_json_binary(&EquivalentResponse { amount })
        }
    }
}
