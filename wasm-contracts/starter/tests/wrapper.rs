use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use goatnft::msg as nft_msg;
use goatnft::{execute as nft_execute, instantiate as nft_instantiate, query as nft_query};
use goatnftburnhook::msg as hook_msg;
use goatnftburnhook::{
    execute as hook_execute, instantiate as hook_instantiate, query as hook_query,
};
use goatnftwrapper::msg as wrap_msg;
use goatnftwrapper::{
    execute as wrap_execute, instantiate as wrap_instantiate, query as wrap_query,
};
use starter::msg as goat_msg;
use starter::{execute as goat_execute, instantiate as goat_instantiate, query as goat_query};

fn contract_goat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(
        goat_execute,
        goat_instantiate,
        goat_query,
    ))
}

fn contract_nft() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(
        nft_execute,
        nft_instantiate,
        nft_query,
    ))
}

fn contract_wrapper() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(
        wrap_execute,
        wrap_instantiate,
        wrap_query,
    ))
}

fn contract_hook() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(
        hook_execute,
        hook_instantiate,
        hook_query,
    ))
}

#[test]
fn wrap_and_unwrap_flow() {
    let mut app = App::default();
    let goat_id = app.store_code(contract_goat());
    let nft_id = app.store_code(contract_nft());
    let wrap_id = app.store_code(contract_wrapper());
    let hook_id = app.store_code(contract_hook());

    let goat_addr = app
        .instantiate_contract(
            goat_id,
            Addr::unchecked("owner"),
            &goat_msg::InstantiateMsg {
                meat_contract: "meat".into(),
            },
            &[],
            "goat",
            None,
        )
        .unwrap();
    let nft_addr = app
        .instantiate_contract(
            nft_id,
            Addr::unchecked("owner"),
            &nft_msg::InstantiateMsg {},
            &[],
            "nft",
            None,
        )
        .unwrap();
    let _hook_addr = app
        .instantiate_contract(
            hook_id,
            Addr::unchecked("owner"),
            &hook_msg::InstantiateMsg {
                nft_contract: nft_addr.to_string(),
            },
            &[],
            "hook",
            None,
        )
        .unwrap();
    let wrapper_addr = app
        .instantiate_contract(
            wrap_id,
            Addr::unchecked("owner"),
            &wrap_msg::InstantiateMsg {
                nft_contract: nft_addr.to_string(),
                goat_contract: goat_addr.to_string(),
            },
            &[],
            "wrapper",
            None,
        )
        .unwrap();
    app.execute_contract(
        Addr::unchecked("owner"),
        goat_addr.clone(),
        &goat_msg::ExecuteMsg::SetMeatAddress { meat_address: wrapper_addr.to_string() },
        &[],
    ).unwrap();

    // mint nft
    let resp = app
        .execute_contract(
            Addr::unchecked("owner"),
            nft_addr.clone(),
            &nft_msg::ExecuteMsg::Mint {
                to: "user".into(),
                nfc_id: "nfc".into(),
                breed: "breed".into(),
                birth_year: 2024,
                weight: 10,
            },
            &[],
        )
        .unwrap();
    let token_id: u64 = resp
        .events
        .iter()
        .find(|e| e.ty == "wasm")
        .unwrap()
        .attributes
        .iter()
        .find(|a| a.key == "token_id")
        .unwrap()
        .value
        .parse()
        .unwrap();

    // approve wrapper
    app.execute_contract(
        Addr::unchecked("user"),
        nft_addr.clone(),
        &nft_msg::ExecuteMsg::Approve {
            spender: wrapper_addr.to_string(),
            token_id: token_id.to_string(),
        },
        &[],
    )
    .unwrap();

    // wrap nft
    let res = app
        .execute_contract(
            Addr::unchecked("user"),
            wrapper_addr.clone(),
            &wrap_msg::ExecuteMsg::Wrap { token_id },
            &[],
        )
        .unwrap();
    assert!(res.events.iter().any(|e| e
        .attributes
        .iter()
        .any(|a| a.key == "action" && a.value == "Wrapped")));

    // approve goat burn
    app.execute_contract(
        Addr::unchecked("user"),
        goat_addr.clone(),
        &goat_msg::ExecuteMsg::Approve {
            spender: wrapper_addr.to_string(),
            amount: Uint128::new(117647058823529400),
        },
        &[],
    )
    .unwrap();
    let res = app
        .execute_contract(
            Addr::unchecked("user"),
            wrapper_addr.clone(),
            &wrap_msg::ExecuteMsg::Unwrap { token_id },
            &[],
        )
        .unwrap();
    assert!(res.events.iter().any(|e| e
        .attributes
        .iter()
        .any(|a| a.key == "action" && a.value == "Unwrapped")));
}

#[test]
fn burn_hook_emits_event() {
    let mut app = App::default();
    let hook_id = app.store_code(contract_hook());
    let _nft_id = app.store_code(contract_nft());
    let hook_addr = app
        .instantiate_contract(
            hook_id,
            Addr::unchecked("owner"),
            &hook_msg::InstantiateMsg {
                nft_contract: "nft".into(),
            },
            &[],
            "hook",
            None,
        )
        .unwrap();
    let res = app
        .execute_contract(
            Addr::unchecked("nft"),
            hook_addr,
            &hook_msg::ExecuteMsg::OnBurn {
                to: "user".into(),
                weight: 10,
            },
            &[],
        )
        .unwrap();
    assert!(res.events.iter().any(|e| e
        .attributes
        .iter()
        .any(|a| a.key == "action" && a.value == "GoatMeatMinted")));
}
