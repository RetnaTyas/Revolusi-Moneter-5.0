use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use goatnft::msg::{
    ExecuteMsg as NftExecute, InstantiateMsg as NftInstantiate, QueryMsg as NftQuery,
};
use goatnft::{execute, instantiate, query};
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
    Box::new(ContractWrapper::new(execute, instantiate, query))
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
                value: Uint128::new(50),
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
    assert_eq!(goat_bal.balance, Uint128::new(50));
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
                value: Uint128::new(50),
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
