use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use goatnft::msg::{
    ExecuteMsg as NftExecute, InstantiateMsg as NftInstantiate, QueryMsg as NftQuery,
};
use goatnft::{execute, instantiate, query};
use starter::msg as goat_msg;
use starter::{execute as goat_execute, instantiate as goat_instantiate, query as goat_query};
use meat::msg as meat_msg;
use meat::{execute as meat_execute, instantiate as meat_instantiate, query as meat_query};
use goatnftburnhook::msg as hook_msg;
use goatnftburnhook::{execute as hook_execute, instantiate as hook_instantiate, query as hook_query};

fn contract_goat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(
        goat_execute,
        goat_instantiate,
        goat_query,
    ))
}

fn contract_nft() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

fn contract_meat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(meat_execute, meat_instantiate, meat_query))
}

fn contract_hook() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(
        hook_execute,
        hook_instantiate,
        hook_query,
    ))
}

#[test]
fn burn_nft_for_goat() {
    let mut app = App::default();
    let goat_id = app.store_code(contract_goat());
    let nft_id = app.store_code(contract_nft());

    let goat_addr = app
        .instantiate_contract(
            goat_id,
            Addr::unchecked("owner"),
            &starter::msg::InstantiateMsg {
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
            &NftInstantiate {},
            &[],
            "nft",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        goat_addr.clone(),
        &goat_msg::ExecuteMsg::SetNftAddress {
            nft_address: nft_addr.to_string(),
        },
        &[],
    )
    .unwrap();

    let resp = app
        .execute_contract(
            Addr::unchecked("owner"),
            nft_addr.clone(),
        &NftExecute::Mint {
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

    app.execute_contract(
        Addr::unchecked("user"),
        nft_addr.clone(),
        &NftExecute::Approve {
            spender: goat_addr.to_string(),
            token_id: token_id.to_string(),
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        goat_addr.clone(),
        &goat_msg::ExecuteMsg::BurnAndMint { token_id },
        &[],
    )
    .unwrap();

    let goat_bal: goat_msg::BalanceResponse = app
        .wrap()
        .query_wasm_smart(
            goat_addr,
            &goat_msg::QueryMsg::Balance {
                address: "user".into(),
            },
        )
        .unwrap();
    assert_eq!(goat_bal.balance, Uint128::new(10));
    let owner_res: Result<String, _> = app
        .wrap()
        .query_wasm_smart(nft_addr, &NftQuery::Owner { token_id });
    assert!(owner_res.is_err());
}

#[test]
fn burn_nft_unauthorized() {
    let mut app = App::default();
    let goat_id = app.store_code(contract_goat());
    let nft_id = app.store_code(contract_nft());

    let goat_addr = app
        .instantiate_contract(
            goat_id,
            Addr::unchecked("owner"),
            &starter::msg::InstantiateMsg {
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
            &NftInstantiate {},
            &[],
            "nft",
            None,
        )
        .unwrap();


    let resp = app
        .execute_contract(
            Addr::unchecked("owner"),
            nft_addr.clone(),
        &NftExecute::Mint {
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

    let err = app
        .execute_contract(
            Addr::unchecked("intruder"),
            nft_addr.clone(),
            &NftExecute::Burn {
                token_id: token_id.to_string(),
            },
            &[],
        )
        .unwrap_err();
    assert!(!err.to_string().is_empty());

    // ensure burn was rejected
}

#[test]
fn burn_invokes_hook() {
    let mut app = App::default();
    let nft_id = app.store_code(contract_nft());
    let hook_id = app.store_code(contract_hook());
    let meat_id = app.store_code(contract_meat());

    let meat_addr = app
        .instantiate_contract(meat_id, Addr::unchecked("owner"), &meat_msg::InstantiateMsg {}, &[], "meat", None)
        .unwrap();

    let nft_addr = app
        .instantiate_contract(nft_id, Addr::unchecked("owner"), &NftInstantiate {}, &[], "nft", None)
        .unwrap();

    let hook_addr = app
        .instantiate_contract(
            hook_id,
            Addr::unchecked("owner"),
            &hook_msg::InstantiateMsg {
                nft_contract: nft_addr.to_string(),
                meat_contract: meat_addr.to_string(),
            },
            &[],
            "hook",
            None,
        )
        .unwrap();

    app
        .execute_contract(
            Addr::unchecked("owner"),
            meat_addr.clone(),
            &meat_msg::ExecuteMsg::SetMinter {
                account: hook_addr.to_string(),
                status: true,
            },
            &[],
        )
        .unwrap();

    app
        .execute_contract(
            Addr::unchecked("owner"),
            nft_addr.clone(),
            &NftExecute::SetBurnHook {
                hook: hook_addr.to_string(),
            },
            &[],
        )
        .unwrap();

    let resp = app
        .execute_contract(
            Addr::unchecked("owner"),
            nft_addr.clone(),
            &NftExecute::Mint {
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

    let res = app
        .execute_contract(
            Addr::unchecked("user"),
            nft_addr,
            &NftExecute::Burn {
                token_id: token_id.to_string(),
            },
            &[],
        )
        .unwrap();

    assert!(res.events.iter().any(|e| {
        e.attributes
            .iter()
            .any(|a| a.key == "action" && a.value == "GoatMeatMinted")
    }));
}
