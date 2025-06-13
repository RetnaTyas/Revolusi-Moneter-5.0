use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use goatnftburnhook::{execute as hook_execute, instantiate as hook_instantiate, query as hook_query};
use goatnftburnhook::msg as hook_msg;
use goatnft::{execute as nft_execute, instantiate as nft_instantiate, query as nft_query};
use meat::{execute as meat_execute, instantiate as meat_instantiate, query as meat_query};

fn contract_nft() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(nft_execute, nft_instantiate, nft_query))
}

fn contract_meat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(meat_execute, meat_instantiate, meat_query))
}

fn contract_hook() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(hook_execute, hook_instantiate, hook_query))
}

#[test]
fn mint_on_burn() {
    let mut app = App::default();
    let nft_id = app.store_code(contract_nft());
    let meat_id = app.store_code(contract_meat());
    let hook_id = app.store_code(contract_hook());

    let nft_addr = app
        .instantiate_contract(nft_id, Addr::unchecked("owner"), &goatnft::msg::InstantiateMsg {}, &[], "nft", None)
        .unwrap();
    let meat_addr = app
        .instantiate_contract(meat_id, Addr::unchecked("owner"), &meat::msg::InstantiateMsg {}, &[], "meat", None)
        .unwrap();
    let hook_addr = app
        .instantiate_contract(
            hook_id,
            Addr::unchecked("owner"),
            &hook_msg::InstantiateMsg { nft_contract: nft_addr.to_string(), meat_contract: meat_addr.to_string() },
            &[],
            "hook",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &meat::msg::ExecuteMsg::SetMinter { account: hook_addr.to_string(), status: true },
        &[],
    ).unwrap();
    app.execute_contract(
        Addr::unchecked("owner"),
        nft_addr.clone(),
        &goatnft::msg::ExecuteMsg::SetBurnHook { hook: hook_addr.to_string() },
        &[],
    ).unwrap();

    let resp = app
        .execute_contract(
            Addr::unchecked("owner"),
            nft_addr.clone(),
            &goatnft::msg::ExecuteMsg::Mint { to: "user".into(), nfc_id: "id".into(), breed: "Boer".into(), birth_year: 2024, weight: 500 },
            &[],
        )
        .unwrap();
    let token_id: u64 = resp.events.iter().find(|e| e.ty == "wasm").unwrap().attributes.iter().find(|a| a.key == "token_id").unwrap().value.parse().unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        nft_addr.clone(),
        &goatnft::msg::ExecuteMsg::UpdateWeight { token_id: token_id.to_string(), new_weight: 500 },
        &[],
    ).unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        nft_addr.clone(),
        &goatnft::msg::ExecuteMsg::Burn { token_id: token_id.to_string() },
        &[],
    ).unwrap();

    let bal: meat::msg::BalanceSubtypeWithLineageResponse = app
        .wrap()
        .query_wasm_smart(meat_addr, &meat::msg::QueryMsg::BalanceOfSubtypeWithLineage { user: "user".into(), subtype: "GOATMEAT".into() })
        .unwrap();
    assert_eq!(bal.balance, Uint128::new(30_000_000_000_000_000_000u128));
}

#[test]
fn owner_query() {
    let mut app = App::default();
    let hook_id = app.store_code(contract_hook());
    let addr = app
        .instantiate_contract(hook_id, Addr::unchecked("owner"), &hook_msg::InstantiateMsg { nft_contract: "nft".into(), meat_contract: "meat".into() }, &[], "hook", None)
        .unwrap();

    let owner: String = app.wrap().query_wasm_smart(addr, &hook_msg::QueryMsg::Owner {}).unwrap();
    assert_eq!(owner, "owner");
}
