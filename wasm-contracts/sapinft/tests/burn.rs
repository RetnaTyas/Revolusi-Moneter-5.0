use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, ContractWrapper, Executor};

use sapinft::msg::{ExecuteMsg as NftExecute, InstantiateMsg as NftInstantiate, QueryMsg as NftQuery};
use sapinft::{execute, instantiate, query};
use meat::msg as meat_msg;
use meat::{execute as meat_execute, instantiate as meat_instantiate, query as meat_query};
use sapinftburnhook::msg as hook_msg;
use sapinftburnhook::{execute as hook_execute, instantiate as hook_instantiate, query as hook_query};

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
fn burn_nft_removes_owner() {
    let mut app = App::default();
    let nft_id = app.store_code(contract_nft());
    let nft_addr = app
        .instantiate_contract(nft_id, Addr::unchecked("owner"), &NftInstantiate {}, &[], "nft", None)
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
            .any(|a| a.key == "action" && a.value == "BeefMeatMinted")
    }));
}
