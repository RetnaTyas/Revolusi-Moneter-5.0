use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use sapinft::msg::{ExecuteMsg as NftExecute, InstantiateMsg as NftInstantiate, QueryMsg as NftQuery};
use sapinft::{execute, instantiate, query};

fn contract_nft() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

fn mint_sample(app: &mut App, nft_addr: Addr) -> u64 {
    let resp = app
        .execute_contract(
            Addr::unchecked("owner"),
            nft_addr.clone(),
            &NftExecute::Mint {
                to: "user1".into(),
                nfc_id: "nfc".into(),
                breed: "breed".into(),
                birth_year: 2024,
                weight: 5,
            },
            &[],
        )
        .unwrap();
    resp
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
        .unwrap()
}

#[test]
fn transfer_changes_owner_and_clears_approval() {
    let mut app = App::default();
    let nft_id = app.store_code(contract_nft());
    let nft_addr = app
        .instantiate_contract(nft_id, Addr::unchecked("owner"), &NftInstantiate {}, &[], "nft", None)
        .unwrap();

    let token_id = mint_sample(&mut app, nft_addr.clone());

    app.execute_contract(
        Addr::unchecked("user1"),
        nft_addr.clone(),
        &NftExecute::Approve {
            spender: "spender".into(),
            token_id: token_id.to_string(),
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("user1"),
        nft_addr.clone(),
        &NftExecute::Transfer {
            to: "user2".into(),
            token_id: token_id.to_string(),
        },
        &[],
    )
    .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked("spender"),
            nft_addr.clone(),
            &NftExecute::Burn {
                token_id: token_id.to_string(),
            },
            &[],
        )
        .unwrap_err();
    assert!(!err.to_string().is_empty());

    let err = app
        .execute_contract(
            Addr::unchecked("user1"),
            nft_addr.clone(),
            &NftExecute::Burn {
                token_id: token_id.to_string(),
            },
            &[],
        )
        .unwrap_err();
    assert!(!err.to_string().is_empty());

    app.execute_contract(
        Addr::unchecked("user2"),
        nft_addr.clone(),
        &NftExecute::Burn {
            token_id: token_id.to_string(),
        },
        &[],
    )
    .unwrap();

    let res: Result<String, _> = app
        .wrap()
        .query_wasm_smart(nft_addr.clone(), &NftQuery::Owner { token_id });
    assert!(res.is_err());
}

#[test]
fn transfer_from_by_approved() {
    let mut app = App::default();
    let nft_id = app.store_code(contract_nft());
    let nft_addr = app
        .instantiate_contract(nft_id, Addr::unchecked("owner"), &NftInstantiate {}, &[], "nft", None)
        .unwrap();

    let token_id = mint_sample(&mut app, nft_addr.clone());

    app.execute_contract(
        Addr::unchecked("user1"),
        nft_addr.clone(),
        &NftExecute::Approve {
            spender: "spender".into(),
            token_id: token_id.to_string(),
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("spender"),
        nft_addr.clone(),
        &NftExecute::TransferFrom {
            owner: "user1".into(),
            to: "user2".into(),
            token_id: token_id.to_string(),
        },
        &[],
    )
    .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked("user1"),
            nft_addr.clone(),
            &NftExecute::Burn {
                token_id: token_id.to_string(),
            },
            &[],
        )
        .unwrap_err();
    assert!(!err.to_string().is_empty());

    app.execute_contract(
        Addr::unchecked("user2"),
        nft_addr.clone(),
        &NftExecute::Burn {
            token_id: token_id.to_string(),
        },
        &[],
    )
    .unwrap();

    let res: Result<String, _> = app
        .wrap()
        .query_wasm_smart(nft_addr, &NftQuery::Owner { token_id });
    assert!(res.is_err());
}
