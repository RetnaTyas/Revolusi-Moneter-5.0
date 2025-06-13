use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use goatnft::{execute as nft_execute, instantiate as nft_instantiate, query as nft_query};
use goatnftwrapper::{execute as wrap_execute, instantiate as wrap_instantiate, query as wrap_query};
use goatnftwrapper::msg as wrap_msg;
use starter::{execute as goat_execute, instantiate as goat_instantiate, query as goat_query};

fn contract_goat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(goat_execute, goat_instantiate, goat_query))
}

fn contract_nft() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(nft_execute, nft_instantiate, nft_query))
}

fn contract_wrapper() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(wrap_execute, wrap_instantiate, wrap_query))
}

#[test]
fn wrap_and_unwrap() {
    let mut app = App::default();
    let goat_id = app.store_code(contract_goat());
    let nft_id = app.store_code(contract_nft());
    let wrap_id = app.store_code(contract_wrapper());

    let goat_addr = app
        .instantiate_contract(goat_id, Addr::unchecked("owner"), &starter::msg::InstantiateMsg {}, &[], "goat", None)
        .unwrap();
    let nft_addr = app
        .instantiate_contract(nft_id, Addr::unchecked("owner"), &goatnft::msg::InstantiateMsg {}, &[], "nft", None)
        .unwrap();
    let wrapper_addr = app
        .instantiate_contract(
            wrap_id,
            Addr::unchecked("owner"),
            &wrap_msg::InstantiateMsg { nft_contract: nft_addr.to_string(), goat_contract: goat_addr.to_string() },
            &[],
            "wrapper",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        goat_addr.clone(),
        &starter::msg::ExecuteMsg::SetWrapperContract { wrapper_address: wrapper_addr.to_string() },
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
        &goatnft::msg::ExecuteMsg::Approve { spender: wrapper_addr.to_string(), token_id: token_id.to_string() },
        &[],
    ).unwrap();

    let wrap_res = app
        .execute_contract(
            Addr::unchecked("user"),
            wrapper_addr.clone(),
            &wrap_msg::ExecuteMsg::Wrap { token_id },
            &[],
        )
        .unwrap();
    assert!(wrap_res.events.iter().any(|e| e.attributes.iter().any(|a| a.key == "action" && a.value == "Wrapped")));

    app.execute_contract(
        Addr::unchecked("user"),
        goat_addr.clone(),
        &starter::msg::ExecuteMsg::Approve { spender: wrapper_addr.to_string(), amount: Uint128::new(117_647_058_823_529_400) },
        &[],
    ).unwrap();

    let unwrap_res = app
        .execute_contract(
            Addr::unchecked("user"),
            wrapper_addr.clone(),
            &wrap_msg::ExecuteMsg::Unwrap { token_id },
            &[],
        )
        .unwrap();
    assert!(unwrap_res.events.iter().any(|e| e.attributes.iter().any(|a| a.key == "action" && a.value == "Unwrapped")));
}

#[test]
fn unauthorized_actions() {
    let mut app = App::default();
    let goat_id = app.store_code(contract_goat());
    let nft_id = app.store_code(contract_nft());
    let wrap_id = app.store_code(contract_wrapper());

    let goat_addr = app
        .instantiate_contract(goat_id, Addr::unchecked("owner"), &starter::msg::InstantiateMsg {}, &[], "goat", None)
        .unwrap();
    let nft_addr = app
        .instantiate_contract(nft_id, Addr::unchecked("owner"), &goatnft::msg::InstantiateMsg {}, &[], "nft", None)
        .unwrap();
    let wrapper_addr = app
        .instantiate_contract(
            wrap_id,
            Addr::unchecked("owner"),
            &wrap_msg::InstantiateMsg { nft_contract: nft_addr.to_string(), goat_contract: goat_addr.to_string() },
            &[],
            "wrapper",
            None,
        )
        .unwrap();

    let resp = app
        .execute_contract(
            Addr::unchecked("owner"),
            nft_addr.clone(),
            &goatnft::msg::ExecuteMsg::Mint { to: "owner".into(), nfc_id: "id".into(), breed: "Boer".into(), birth_year: 2024, weight: 500 },
            &[],
        )
        .unwrap();
    let token_id: u64 = resp.events.iter().find(|e| e.ty == "wasm").unwrap().attributes.iter().find(|a| a.key == "token_id").unwrap().value.parse().unwrap();
    app.execute_contract(
        Addr::unchecked("owner"),
        nft_addr.clone(),
        &goatnft::msg::ExecuteMsg::Approve { spender: wrapper_addr.to_string(), token_id: token_id.to_string() },
        &[],
    ).unwrap();

    let err = app
        .execute_contract(Addr::unchecked("user"), wrapper_addr.clone(), &wrap_msg::ExecuteMsg::Wrap { token_id }, &[])
        .unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[test]
fn owner_query() {
    let mut app = App::default();
    let wrap_id = app.store_code(contract_wrapper());
    let addr = app
        .instantiate_contract(wrap_id, Addr::unchecked("owner"), &wrap_msg::InstantiateMsg { nft_contract: "nft".into(), goat_contract: "goat".into() }, &[], "wrapper", None)
        .unwrap();

    let owner: String = app.wrap().query_wasm_smart(addr, &wrap_msg::QueryMsg::Owner {}).unwrap();
    assert_eq!(owner, "owner");
}
