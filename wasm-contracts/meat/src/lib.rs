use cosmwasm_std::{
    entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::msg::{
    AllowanceResponse, BalanceResponse, BalanceSubtypeWithLineageResponse, ExecuteMsg,
    InstantiateMsg, QueryMsg, TokenInfoResponse, TotalSupplyResponse,
};
use crate::state::*;
use std::cmp;

pub mod msg;
pub mod state;

const CONTRACT_NAME: &str = "meat";
const CONTRACT_VERSION: &str = "0.1.0";
const GOATMEAT_SUBTYPE: &[u8] = b"GOATMEAT";

fn add_balance(
    store: &mut dyn cosmwasm_std::Storage,
    addr: &Addr,
    amount: Uint128,
) -> StdResult<()> {
    let bal = BALANCES.may_load(store, addr)?.unwrap_or_default() + amount;
    BALANCES.save(store, addr, &bal)
}

fn sub_balance(
    store: &mut dyn cosmwasm_std::Storage,
    addr: &Addr,
    amount: Uint128,
) -> StdResult<()> {
    let bal = BALANCES.may_load(store, addr)?.unwrap_or_default();
    if bal < amount {
        return Err(StdError::generic_err("Insufficient balance"));
    }
    BALANCES.save(store, addr, &(bal - amount))
}

fn add_user_subtype(
    store: &mut dyn cosmwasm_std::Storage,
    addr: &Addr,
    st: &[u8],
) -> StdResult<()> {
    let mut list = USER_SUBTYPES.may_load(store, addr)?.unwrap_or_default();
    if !list.iter().any(|x| x.as_slice() == st) {
        list.push(st.to_vec());
        USER_SUBTYPES.save(store, addr, &list)?;
    }
    Ok(())
}

fn remove_user_subtype(
    store: &mut dyn cosmwasm_std::Storage,
    addr: &Addr,
    st: &[u8],
) -> StdResult<()> {
    let mut list = USER_SUBTYPES.may_load(store, addr)?.unwrap_or_default();
    if let Some(pos) = list.iter().position(|x| x.as_slice() == st) {
        list.remove(pos);
        USER_SUBTYPES.save(store, addr, &list)?;
    }
    Ok(())
}

fn mint_subtype_internal(
    store: &mut dyn cosmwasm_std::Storage,
    to: &Addr,
    subtype: &[u8],
    amount: Uint128,
) -> StdResult<()> {
    add_balance(store, to, amount)?;
    let mut total = TOTAL_SUPPLY.load(store)? + amount;
    TOTAL_SUPPLY.save(store, &total)?;

    let mut sub_bal = SUBTYPE_BALANCES
        .may_load(store, (to, subtype))?
        .unwrap_or_default();
    if sub_bal.is_zero() {
        add_user_subtype(store, to, subtype)?;
    }
    sub_bal += amount;
    SUBTYPE_BALANCES.save(store, (to, subtype), &sub_bal)?;

    let mut sub_total = SUBTYPE_TOTAL_SUPPLY
        .may_load(store, subtype)?
        .unwrap_or_default();
    sub_total += amount;
    SUBTYPE_TOTAL_SUPPLY.save(store, subtype, &sub_total)?;
    Ok(())
}

fn burn_subtype_internal(
    store: &mut dyn cosmwasm_std::Storage,
    from: &Addr,
    subtype: &[u8],
    amount: Uint128,
) -> StdResult<()> {
    sub_balance(store, from, amount)?;
    let mut total = TOTAL_SUPPLY.load(store)?;
    total -= amount;
    TOTAL_SUPPLY.save(store, &total)?;

    let mut sub_bal = SUBTYPE_BALANCES
        .may_load(store, (from, subtype))?
        .unwrap_or_default();
    if sub_bal < amount {
        return Err(StdError::generic_err("Insufficient subtype balance"));
    }
    sub_bal -= amount;
    if sub_bal.is_zero() {
        remove_user_subtype(store, from, subtype)?;
    }
    SUBTYPE_BALANCES.save(store, (from, subtype), &sub_bal)?;

    let mut sub_total = SUBTYPE_TOTAL_SUPPLY
        .may_load(store, subtype)?
        .unwrap_or_default();
    sub_total -= amount;
    SUBTYPE_TOTAL_SUPPLY.save(store, subtype, &sub_total)?;
    Ok(())
}

fn move_subtypes(
    store: &mut dyn cosmwasm_std::Storage,
    from: &Addr,
    to: &Addr,
    mut amount: Uint128,
) -> StdResult<()> {
    if amount.is_zero() || from == to {
        return Ok(());
    }
    let mut list = USER_SUBTYPES.may_load(store, from)?.unwrap_or_default();
    let mut idx = TRANSFER_CURSOR.may_load(store, from)?.unwrap_or(0u64) as usize;
    if idx >= list.len() {
        idx = 0;
    }
    while amount > Uint128::zero() && !list.is_empty() {
        if idx >= list.len() {
            idx = 0;
        }
        let st = list[idx].clone();
        let mut from_bal = SUBTYPE_BALANCES
            .may_load(store, (from, st.as_slice()))?
            .unwrap_or_default();
        if from_bal.is_zero() {
            remove_user_subtype(store, from, st.as_slice())?;
            list = USER_SUBTYPES.may_load(store, from)?.unwrap_or_default();
            if idx >= list.len() {
                idx = 0;
            }
            continue;
        }
        let t_amt = cmp::min(from_bal, amount);
        from_bal -= t_amt;
        if from_bal.is_zero() {
            remove_user_subtype(store, from, st.as_slice())?;
            list = USER_SUBTYPES.may_load(store, from)?.unwrap_or_default();
            if idx >= list.len() {
                idx = 0;
            }
        } else {
            idx += 1;
        }
        SUBTYPE_BALANCES.save(store, (from, st.as_slice()), &from_bal)?;

        let mut to_bal = SUBTYPE_BALANCES
            .may_load(store, (to, st.as_slice()))?
            .unwrap_or_default();
        if to_bal.is_zero() {
            add_user_subtype(store, to, st.as_slice())?;
            let lineage = SUBTYPE_LINEAGE
                .may_load(store, (from, st.as_slice()))?
                .unwrap_or_default();
            SUBTYPE_LINEAGE.save(store, (to, st.as_slice()), &lineage)?;
        } else {
            let from_lineage = SUBTYPE_LINEAGE
                .may_load(store, (from, st.as_slice()))?
                .unwrap_or_default();
            let to_lineage = SUBTYPE_LINEAGE
                .may_load(store, (to, st.as_slice()))?
                .unwrap_or_default();
            if from_lineage != to_lineage {
                return Err(StdError::generic_err("Lineage mismatch"));
            }
        }
        to_bal += t_amt;
        SUBTYPE_BALANCES.save(store, (to, st.as_slice()), &to_bal)?;
        amount -= t_amt;
    }
    TRANSFER_CURSOR.save(store, from, &((idx) as u64))?;
    Ok(())
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let owner = info.sender.clone();
    OWNER.save(deps.storage, &owner)?;
    MINTERS.save(deps.storage, &owner, &true)?;
    RATE_HANDLER.save(deps.storage, &None)?;
    TOTAL_SUPPLY.save(deps.storage, &Uint128::zero())?;
    mint_subtype_internal(
        deps.storage,
        &owner,
        GOATMEAT_SUBTYPE,
        Uint128::new(INITIAL_SUPPLY),
    )?;
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

fn only_minter(deps: DepsMut, info: &MessageInfo) -> StdResult<()> {
    let is = MINTERS
        .may_load(deps.storage, &info.sender)?
        .unwrap_or(false);
    if !is {
        return Err(StdError::generic_err("Not minter"));
    }
    Ok(())
}

fn only_burner(deps: DepsMut, info: &MessageInfo) -> StdResult<()> {
    let is = BURNERS
        .may_load(deps.storage, &info.sender)?
        .unwrap_or(false);
    if !is {
        return Err(StdError::generic_err("Not burner"));
    }
    Ok(())
}

fn only_owner_or_minter(deps: DepsMut, info: &MessageInfo) -> StdResult<()> {
    let owner = OWNER.load(deps.storage)?;
    if info.sender == owner {
        return Ok(());
    }
    only_minter(deps, info)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => {
            execute_transfer(deps, info, recipient, amount)
        }
        ExecuteMsg::Approve { spender, amount } => execute_approve(deps, info, spender, amount),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => execute_transfer_from(deps, info, owner, recipient, amount),
        ExecuteMsg::RedeemForMeat { amount } => execute_redeem_for_meat(deps, info, amount),
        ExecuteMsg::MintSubtype {
            to,
            subtype,
            amount,
        } => execute_mint_subtype(deps, info, to, subtype, amount),
        ExecuteMsg::BurnSubtype {
            from,
            subtype,
            amount,
        } => execute_burn_subtype(deps, info, from, subtype, amount),
        ExecuteMsg::SetMinter { account, status } => {
            execute_set_minter(deps, info, account, status)
        }
        ExecuteMsg::SetBurner { account, status } => {
            execute_set_burner(deps, info, account, status)
        }
        ExecuteMsg::SetSubtypeLineage {
            user,
            subtype,
            lineage_id,
        } => execute_set_lineage(deps, info, user, subtype, lineage_id),
        ExecuteMsg::SetRateHandler { address } => {
            execute_set_rate_handler(deps, info, address)
        }
    }
}

fn execute_transfer(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> StdResult<Response> {
    let recipient = deps.api.addr_validate(&recipient)?;
    sub_balance(deps.storage, &info.sender, amount)?;
    add_balance(deps.storage, &recipient, amount)?;
    move_subtypes(deps.storage, &info.sender, &recipient, amount)?;
    Ok(Response::new())
}

fn execute_approve(
    deps: DepsMut,
    info: MessageInfo,
    spender: String,
    amount: Uint128,
) -> StdResult<Response> {
    let spender = deps.api.addr_validate(&spender)?;
    ALLOWANCES.save(deps.storage, (&info.sender, &spender), &amount)?;
    Ok(Response::new())
}

fn execute_transfer_from(
    deps: DepsMut,
    info: MessageInfo,
    owner: String,
    recipient: String,
    amount: Uint128,
) -> StdResult<Response> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let recipient = deps.api.addr_validate(&recipient)?;
    let mut allowance = ALLOWANCES
        .may_load(deps.storage, (&owner_addr, &info.sender))?
        .unwrap_or_default();
    if allowance < amount {
        return Err(StdError::generic_err("Allowance exceeded"));
    }
    allowance -= amount;
    ALLOWANCES.save(deps.storage, (&owner_addr, &info.sender), &allowance)?;
    sub_balance(deps.storage, &owner_addr, amount)?;
    add_balance(deps.storage, &recipient, amount)?;
    move_subtypes(deps.storage, &owner_addr, &recipient, amount)?;
    Ok(Response::new())
}

fn execute_mint_subtype(
    mut deps: DepsMut,
    info: MessageInfo,
    to: String,
    subtype: String,
    amount: Uint128,
) -> StdResult<Response> {
    only_minter(deps.branch(), &info)?;
    if amount.is_zero() || subtype.is_empty() {
        return Err(StdError::generic_err("Invalid params"));
    }
    let to_addr = deps.api.addr_validate(&to)?;
    mint_subtype_internal(deps.storage, &to_addr, subtype.as_bytes(), amount)?;
    Ok(Response::new()
        .add_attribute("action", "SubtypeMinted")
        .add_attribute("to", to)
        .add_attribute("subtype", subtype)
        .add_attribute("amount", amount))
}

fn execute_burn_subtype(
    mut deps: DepsMut,
    info: MessageInfo,
    from: String,
    subtype: String,
    amount: Uint128,
) -> StdResult<Response> {
    only_burner(deps.branch(), &info)?;
    if amount.is_zero() || subtype.is_empty() {
        return Err(StdError::generic_err("Invalid params"));
    }
    let from_addr = deps.api.addr_validate(&from)?;
    burn_subtype_internal(deps.storage, &from_addr, subtype.as_bytes(), amount)?;
    Ok(Response::new()
        .add_attribute("action", "SubtypeBurned")
        .add_attribute("from", from)
        .add_attribute("subtype", subtype)
        .add_attribute("amount", amount))
}

fn execute_set_minter(
    mut deps: DepsMut,
    info: MessageInfo,
    account: String,
    status: bool,
) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    let addr = deps.api.addr_validate(&account)?;
    MINTERS.save(deps.storage, &addr, &status)?;
    Ok(Response::new()
        .add_attribute("action", "MinterUpdated")
        .add_attribute("account", account)
        .add_attribute("status", status.to_string()))
}

fn execute_set_burner(
    mut deps: DepsMut,
    info: MessageInfo,
    account: String,
    status: bool,
) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    let addr = deps.api.addr_validate(&account)?;
    BURNERS.save(deps.storage, &addr, &status)?;
    Ok(Response::new()
        .add_attribute("action", "BurnerUpdated")
        .add_attribute("account", account)
        .add_attribute("status", status.to_string()))
}

fn execute_set_lineage(
    mut deps: DepsMut,
    info: MessageInfo,
    user: String,
    subtype: String,
    lineage_id: u64,
) -> StdResult<Response> {
    only_owner_or_minter(deps.branch(), &info)?;
    let addr = deps.api.addr_validate(&user)?;
    SUBTYPE_LINEAGE.save(deps.storage, (&addr, subtype.as_bytes()), &lineage_id)?;
    Ok(Response::new()
        .add_attribute("action", "SubtypeLineageUpdated")
        .add_attribute("user", user)
        .add_attribute("subtype", subtype)
        .add_attribute("lineage_id", lineage_id.to_string()))
}

fn execute_set_rate_handler(
    mut deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> StdResult<Response> {
    only_owner(deps.branch(), &info)?;
    let addr = deps.api.addr_validate(&address)?;
    RATE_HANDLER.save(deps.storage, &Some(addr.clone()))?;
    Ok(Response::new()
        .add_attribute("action", "RateHandlerUpdated")
        .add_attribute("address", addr.into_string()))
}

fn execute_redeem_for_meat(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> StdResult<Response> {
    if amount.is_zero() {
        return Err(StdError::generic_err("Amount must be > 0"));
    }
    sub_balance(deps.storage, &info.sender, amount)?;
    let total = TOTAL_SUPPLY.load(deps.storage)?;
    if total < amount {
        return Err(StdError::generic_err("Insufficient total supply"));
    }
    TOTAL_SUPPLY.save(deps.storage, &(total - amount))?;
    Ok(Response::new()
        .add_attribute("action", "MeatRedeemed")
        .add_attribute("user", info.sender)
        .add_attribute("amount", amount))
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
            let allowance = ALLOWANCES
                .may_load(deps.storage, (&owner, &spender))?
                .unwrap_or_default();
            to_json_binary(&AllowanceResponse { allowance })
        }
        QueryMsg::TokenInfo {} => {
            let total_supply = TOTAL_SUPPLY.load(deps.storage)?;
            to_json_binary(&TokenInfoResponse {
                name: NAME.to_string(),
                symbol: SYMBOL.to_string(),
                decimals: DECIMALS,
                total_supply,
            })
        }
        QueryMsg::Owner {} => {
            let owner = OWNER.load(deps.storage)?;
            to_json_binary(&owner.into_string())
        }
        QueryMsg::BalanceOfSubtypeWithLineage { user, subtype } => {
            let addr = deps.api.addr_validate(&user)?;
            let bal = SUBTYPE_BALANCES
                .may_load(deps.storage, (&addr, subtype.as_bytes()))?
                .unwrap_or_default();
            let lineage = SUBTYPE_LINEAGE
                .may_load(deps.storage, (&addr, subtype.as_bytes()))?
                .unwrap_or_default();
            to_json_binary(&BalanceSubtypeWithLineageResponse {
                balance: bal,
                lineage_id: lineage,
            })
        }
        QueryMsg::BalanceOfSubtype { user, subtype } => {
            let addr = deps.api.addr_validate(&user)?;
            let bal = SUBTYPE_BALANCES
                .may_load(deps.storage, (&addr, subtype.as_bytes()))?
                .unwrap_or_default();
            to_json_binary(&BalanceResponse { balance: bal })
        }
        QueryMsg::TotalSupplyOfSubtype { subtype } => {
            let total = SUBTYPE_TOTAL_SUPPLY
                .may_load(deps.storage, subtype.as_bytes())?
                .unwrap_or_default();
            to_json_binary(&TotalSupplyResponse { total })
        }
        QueryMsg::RateHandler {} => {
            let addr = RATE_HANDLER.load(deps.storage)?.map(|a| a.into_string());
            to_json_binary(&addr)
        }
    }
}
