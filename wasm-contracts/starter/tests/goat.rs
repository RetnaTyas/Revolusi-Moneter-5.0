use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use starter::{execute, instantiate, query};
use starter::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, BalanceResponse, StakingInfoResponse, TokenInfoResponse, PendingRewardResponse};
use goatnft::{execute as nft_execute, instantiate as nft_instantiate, query as nft_query};
use goatnft::msg as nft_msg;
use goatnft::state::WEIGHT_UPDATE_VALIDITY;

fn contract_goat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

fn contract_nft() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(nft_execute, nft_instantiate, nft_query))
}

fn init_app() -> (App, Addr) {
    let mut app = App::default();
    let code_id = app.store_code(contract_goat());
    let msg = InstantiateMsg {};
    let addr = app
        .instantiate_contract(code_id, Addr::unchecked("owner"), &msg, &[], "goat", None)
        .unwrap();
    (app, addr)
}

fn mint_to(app: &mut App, addr: Addr, to: &str, amount: Uint128) {
    app.execute_contract(
        Addr::unchecked("owner"),
        addr.clone(),
        &ExecuteMsg::SetWrapperContract { wrapper_address: "wrapper".into() },
        &[],
    )
    .unwrap();
    app.execute_contract(
        Addr::unchecked("wrapper"),
        addr,
        &ExecuteMsg::Mint { to: to.into(), amount },
        &[],
    )
    .unwrap();
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
    mint_to(&mut app, addr.clone(), "staker", Uint128::new(100));
    let resp = app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::Stake { amount: Uint128::new(100) }, &[]).unwrap();
    assert!(resp.events.iter().any(|e| e.ty == "wasm" && e.attributes.iter().any(|a| a.key == "action" && a.value == "Staked")));
    app.update_block(|b| b.time = b.time.plus_seconds(1));
    let claim_res = app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::ClaimReward {}, &[]).unwrap();
    assert!(claim_res.events.iter().any(|e| e.ty == "wasm" && e.attributes.iter().any(|a| a.key == "action" && a.value == "RewardClaimed")));
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
    mint_to(&mut app, addr.clone(), "staker", Uint128::new(100));
    let stake_res = app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::Stake { amount: Uint128::new(100) }, &[]).unwrap();
    assert!(stake_res.events.iter().any(|e| e.ty == "wasm" && e.attributes.iter().any(|a| a.key == "action" && a.value == "Staked")));
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
    mint_to(&mut app, addr.clone(), "staker", Uint128::new(100));
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::Stake { amount: Uint128::new(100) }, &[]).unwrap();
    app.update_block(|b| b.time = b.time.plus_seconds(1));
    let unstake_res = app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::Unstake {}, &[]).unwrap();
    assert!(unstake_res.events.iter().any(|e| e.ty == "wasm" && e.attributes.iter().any(|a| a.key == "action" && a.value == "Unstaked")));
    let bal: BalanceResponse = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::Balance { address: "staker".into() }).unwrap();
    assert_eq!(bal.balance, Uint128::new(200));
}

#[test]
fn owner_only() {
    let (mut app, addr) = init_app();
    let _err = app.execute_contract(
        Addr::unchecked("not_owner"),
        addr.clone(),
        &ExecuteMsg::SetWrapperContract { wrapper_address: "x".into() },
        &[]
    ).unwrap_err();
    let _err = app.execute_contract(
        Addr::unchecked("not_owner"),
        addr.clone(),
        &ExecuteMsg::SetRewardConfig { new_rate: Uint128::one(), new_interval: 1, new_min_claim: 1 },
        &[]
    ).unwrap_err();
}


#[test]
fn claim_reward_multiple_times() {
    let (mut app, addr) = init_app();
    app.execute_contract(
        Addr::unchecked("owner"),
        addr.clone(),
        &ExecuteMsg::SetRewardConfig { new_rate: Uint128::new(1_000_000_000_000_000_000), new_interval: 1, new_min_claim: 1 },
        &[]
    ).unwrap();
    mint_to(&mut app, addr.clone(), "staker", Uint128::new(100));
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::Stake { amount: Uint128::new(100) }, &[]).unwrap();

    app.update_block(|b| b.time = b.time.plus_seconds(1));
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::ClaimReward {}, &[]).unwrap();
    let bal1: BalanceResponse = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::Balance { address: "staker".into() }).unwrap();
    assert_eq!(bal1.balance, Uint128::new(100));

    app.update_block(|b| b.time = b.time.plus_seconds(1));
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::ClaimReward {}, &[]).unwrap();
    let bal2: BalanceResponse = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::Balance { address: "staker".into() }).unwrap();
    assert_eq!(bal2.balance, Uint128::new(200));
}

#[test]
fn emergency_unstake_returns_only_stake() {
    let (mut app, addr) = init_app();
    app.execute_contract(
        Addr::unchecked("owner"),
        addr.clone(),
        &ExecuteMsg::SetRewardConfig { new_rate: Uint128::new(1_000_000_000_000_000_000), new_interval: 1, new_min_claim: 10 },
        &[]
    ).unwrap();
    mint_to(&mut app, addr.clone(), "user", Uint128::new(50));
    app.execute_contract(Addr::unchecked("user"), addr.clone(), &ExecuteMsg::Stake { amount: Uint128::new(50) }, &[]).unwrap();
    app.update_block(|b| b.time = b.time.plus_seconds(5));
    app.execute_contract(Addr::unchecked("user"), addr.clone(), &ExecuteMsg::EmergencyUnstake {}, &[]).unwrap();
    let bal: BalanceResponse = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::Balance { address: "user".into() }).unwrap();
    assert_eq!(bal.balance, Uint128::new(50));
    let pending: PendingRewardResponse = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::PendingReward { address: "user".into() }).unwrap();
    assert_eq!(pending.reward, Uint128::zero());
}

#[test]
fn claim_without_stake_fails() {
    let (mut app, addr) = init_app();
    let err = app.execute_contract(
        Addr::unchecked("user"),
        addr,
        &ExecuteMsg::ClaimReward {},
        &[]
    ).unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[test]
fn emergency_unstake_mints_shortfall() {
    let (mut app, addr) = init_app();
    mint_to(&mut app, addr.clone(), "user", Uint128::new(50));
    app.execute_contract(Addr::unchecked("user"), addr.clone(), &ExecuteMsg::Stake { amount: Uint128::new(50) }, &[]).unwrap();
    // drain contract balance
    app.execute_contract(addr.clone(), addr.clone(), &ExecuteMsg::Transfer { recipient: "other".into(), amount: Uint128::new(30) }, &[]).unwrap();
    app.execute_contract(Addr::unchecked("user"), addr.clone(), &ExecuteMsg::EmergencyUnstake {}, &[]).unwrap();
    let bal: BalanceResponse = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::Balance { address: "user".into() }).unwrap();
    assert_eq!(bal.balance, Uint128::new(50));
    let info: TokenInfoResponse = app.wrap().query_wasm_smart(addr, &QueryMsg::TokenInfo {}).unwrap();
    assert_eq!(info.total_supply, Uint128::new(80));
}

#[test]
fn unstake_mints_shortfall() {
    let (mut app, addr) = init_app();
    app.execute_contract(Addr::unchecked("owner"), addr.clone(), &ExecuteMsg::SetRewardConfig { new_rate: Uint128::new(1_000_000_000_000_000_000), new_interval: 1, new_min_claim: 1 }, &[]).unwrap();
    mint_to(&mut app, addr.clone(), "user", Uint128::new(50));
    app.execute_contract(Addr::unchecked("user"), addr.clone(), &ExecuteMsg::Stake { amount: Uint128::new(50) }, &[]).unwrap();
    app.execute_contract(addr.clone(), addr.clone(), &ExecuteMsg::Transfer { recipient: "other".into(), amount: Uint128::new(30) }, &[]).unwrap();
    app.update_block(|b| b.time = b.time.plus_seconds(1));
    app.execute_contract(Addr::unchecked("user"), addr.clone(), &ExecuteMsg::Unstake {}, &[]).unwrap();
    let bal: BalanceResponse = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::Balance { address: "user".into() }).unwrap();
    assert_eq!(bal.balance, Uint128::new(100));
    let info: TokenInfoResponse = app.wrap().query_wasm_smart(addr, &QueryMsg::TokenInfo {}).unwrap();
    assert_eq!(info.total_supply, Uint128::new(130));
}

#[test]
fn claim_reward_mints_shortfall() {
    let (mut app, addr) = init_app();
    app.execute_contract(Addr::unchecked("owner"), addr.clone(), &ExecuteMsg::SetRewardConfig { new_rate: Uint128::new(1_000_000_000_000_000_000), new_interval: 1, new_min_claim: 1 }, &[]).unwrap();
    mint_to(&mut app, addr.clone(), "staker", Uint128::new(50));
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::Stake { amount: Uint128::new(50) }, &[]).unwrap();
    app.execute_contract(addr.clone(), addr.clone(), &ExecuteMsg::Transfer { recipient: "other".into(), amount: Uint128::new(40) }, &[]).unwrap();
    app.update_block(|b| b.time = b.time.plus_seconds(1));
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::ClaimReward {}, &[]).unwrap();
    let bal: BalanceResponse = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::Balance { address: "staker".into() }).unwrap();
    assert_eq!(bal.balance, Uint128::new(50));
    let info: TokenInfoResponse = app.wrap().query_wasm_smart(addr, &QueryMsg::TokenInfo {}).unwrap();
    assert_eq!(info.total_supply, Uint128::new(90));
}

#[test]
fn compound_reward_mints_shortfall() {
    let (mut app, addr) = init_app();
    app.execute_contract(Addr::unchecked("owner"), addr.clone(), &ExecuteMsg::SetRewardConfig { new_rate: Uint128::new(1_000_000_000_000_000_000), new_interval: 1, new_min_claim: 1 }, &[]).unwrap();
    mint_to(&mut app, addr.clone(), "staker", Uint128::new(50));
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::Stake { amount: Uint128::new(50) }, &[]).unwrap();
    app.execute_contract(addr.clone(), addr.clone(), &ExecuteMsg::Transfer { recipient: "other".into(), amount: Uint128::new(30) }, &[]).unwrap();
    app.update_block(|b| b.time = b.time.plus_seconds(1));
    app.execute_contract(Addr::unchecked("staker"), addr.clone(), &ExecuteMsg::CompoundReward {}, &[]).unwrap();
    let stake: StakingInfoResponse = app.wrap().query_wasm_smart(addr.clone(), &QueryMsg::StakingBalance { address: "staker".into() }).unwrap();
    assert_eq!(stake.balance, Uint128::new(100));
    let info: TokenInfoResponse = app.wrap().query_wasm_smart(addr, &QueryMsg::TokenInfo {}).unwrap();
    assert_eq!(info.total_supply, Uint128::new(80));
}
