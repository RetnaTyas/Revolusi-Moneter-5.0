use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use sapinft::msg::{
    ExecuteMsg as NftExecute, InstantiateMsg as NftInstantiate, QueryMsg as NftQuery,
};
use sapinft::state::WEIGHT_UPDATE_VALIDITY;
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
                to: "user".into(),
                nfc_id: "nfc".into(),
                breed: "breed".into(),
                birth_year: 2024,
                weight: 5,
            },
            &[],
        )
        .unwrap();
    resp.events
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
fn duplicate_nfc_fails() {
    let mut app = App::default();
    let nft_id = app.store_code(contract_nft());
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

    let _ = mint_sample(&mut app, nft_addr.clone());
    let err = app
        .execute_contract(
            Addr::unchecked("owner"),
            nft_addr.clone(),
            &NftExecute::Mint {
                to: "other".into(),
                nfc_id: "nfc".into(),
                breed: "breed".into(),
                birth_year: 2024,
                weight: 5,
            },
            &[],
        )
        .unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[test]
fn burn_requires_recent_update() {
    let mut app = App::default();
    let nft_id = app.store_code(contract_nft());
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

    let token_id = mint_sample(&mut app, nft_addr.clone());

    app.update_block(|b| b.time = b.time.plus_seconds(WEIGHT_UPDATE_VALIDITY + 1));

    app.execute_contract(
        Addr::unchecked("user"),
        nft_addr.clone(),
        &NftExecute::UpdateWeight {
            token_id: token_id.to_string(),
            new_weight: 8,
        },
        &[],
    )
    .unwrap();

    app.update_block(|b| b.time = b.time.plus_seconds(WEIGHT_UPDATE_VALIDITY + 1));

    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            nft_addr,
            &NftExecute::Burn {
                token_id: token_id.to_string(),
            },
            &[],
        )
        .unwrap_err();
    assert!(!err.to_string().is_empty());
}
