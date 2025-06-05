use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use starter::{execute, instantiate, query};
use starter::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, BalanceResponse, StakingInfoResponse, TokenInfoResponse};

fn contract_goat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

fn init_app() -> (App, Addr) {
    let mut app = App::default();
    let code_id = app.store_code(contract_goat());
    let msg = InstantiateMsg { meat_contract: "meat".into() };
    let addr = app
        .instantiate_contract(code_id, Addr::unchecked("owner"), &msg, &[], "goat", None)
        .unwrap();
    (app, addr)
}

#[test]
fn instantiate_sets_owner_and_defaults() {
    let (app, addr) = init_app();
    let owner: String = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::Owner {}).unwrap();
    assert_eq!(owner, "owner");
    let info: TokenInfoResponse = app.wrap().query_wasm_smart(addr, &QueryMsg::TokenInfo {}).unwrap();
    assert_eq!(info.total_supply, Uint128::zero());
}

#[test]
fn stake_and_claim() {
    let (mut app, addr) = init_app();
    // set high reward config for easier testing
    app.execute_contract(
        Addr::unchecked("owner"),
        addr.clone(),
        &ExecuteMsg::SetRewardConfig { new_rate: Uint128::new(1_000_000_000_000_000_000), new_interval: 1, new_min_claim: 1 },
        &[]
    ).unwrap();
    // mint and stake
    app.execute_contract(Addr::unchecked("meat"), addr.clone(), &ExecuteMsg::MintTo { to: "staker".into(), amount: Uint128::new(100) }, &[]).unwrap();
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::Stake { amount: Uint128::new(100) }, &[]).unwrap();
    app.update_block(|b| b.time = b.time.plus_seconds(1));
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::ClaimReward {}, &[]).unwrap();
    let bal: BalanceResponse = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::Balance { address: "staker".into() }).unwrap();
    assert_eq!(bal.balance, Uint128::new(100));
}

#[test]
fn stake_and_compound() {
    let (mut app, addr) = init_app();
    app.execute_contract(
        Addr::unchecked("owner"),
        addr.clone(),
        &ExecuteMsg::SetRewardConfig { new_rate: Uint128::new(1_000_000_000_000_000_000), new_interval: 1, new_min_claim: 1 },
        &[]
    ).unwrap();
    app.execute_contract(Addr::unchecked("meat"), addr.clone(), &ExecuteMsg::MintTo { to: "staker".into(), amount: Uint128::new(100) }, &[]).unwrap();
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::Stake { amount: Uint128::new(100) }, &[]).unwrap();
    app.update_block(|b| b.time = b.time.plus_seconds(1));
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::CompoundReward {}, &[]).unwrap();
    let staking: StakingInfoResponse = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::StakingBalance { address: "staker".into() }).unwrap();
    assert_eq!(staking.balance, Uint128::new(200));
}

#[test]
fn stake_and_unstake() {
    let (mut app, addr) = init_app();
    app.execute_contract(
        Addr::unchecked("owner"),
        addr.clone(),
        &ExecuteMsg::SetRewardConfig { new_rate: Uint128::new(1_000_000_000_000_000_000), new_interval: 1, new_min_claim: 1 },
        &[]
    ).unwrap();
    app.execute_contract(Addr::unchecked("meat"), addr.clone(), &ExecuteMsg::MintTo { to: "staker".into(), amount: Uint128::new(100) }, &[]).unwrap();
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::Stake { amount: Uint128::new(100) }, &[]).unwrap();
    app.update_block(|b| b.time = b.time.plus_seconds(1));
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::Unstake {}, &[]).unwrap();
    let bal: BalanceResponse = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::Balance { address: "staker".into() }).unwrap();
    assert_eq!(bal.balance, Uint128::new(200));
}

#[test]
fn owner_only() {
    let (mut app, addr) = init_app();
    let _err = app.execute_contract(
        Addr::unchecked("not_owner"),
        addr.clone(),
        &ExecuteMsg::SetMeatAddress { meat_address: "new".into() },
        &[]
    ).unwrap_err();
    let _err = app.execute_contract(
        Addr::unchecked("not_owner"),
        addr.clone(),
        &ExecuteMsg::SetRewardConfig { new_rate: Uint128::one(), new_interval: 1, new_min_claim: 1 },
        &[]
    ).unwrap_err();
}
