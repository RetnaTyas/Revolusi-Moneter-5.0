use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, ContractWrapper, Executor};

use sapinft::msg::{ExecuteMsg as NftExecute, InstantiateMsg as NftInstantiate, QueryMsg as NftQuery};
use sapinft::{execute, instantiate, query};

fn contract_nft() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
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
