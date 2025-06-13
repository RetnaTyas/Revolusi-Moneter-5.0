use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use meat::msg::{BalanceSubtypeWithLineageResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use meat::{execute, instantiate, query};

fn contract_meat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

#[test]
fn mint_burn_and_lineage() {
    let mut app = App::default();
    let meat_id = app.store_code(contract_meat());

    let meat_addr = app
        .instantiate_contract(
            meat_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {},
            &[],
            "meat",
            None,
        )
        .unwrap();

    // owner is minter by default, mint to user
    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &ExecuteMsg::MintSubtype {
            to: "user".into(),
            subtype: "GOATMEAT".into(),
            amount: Uint128::new(10),
        },
        &[],
    )
    .unwrap();

    // set lineage
    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &ExecuteMsg::SetSubtypeLineage {
            user: "user".into(),
            subtype: "GOATMEAT".into(),
            lineage_id: 1,
        },
        &[],
    )
    .unwrap();

    // query
    let res: BalanceSubtypeWithLineageResponse = app
        .wrap()
        .query_wasm_smart(
            meat_addr.clone(),
            &QueryMsg::BalanceOfSubtypeWithLineage {
                user: "user".into(),
                subtype: "GOATMEAT".into(),
            },
        )
        .unwrap();
    assert_eq!(res.balance, Uint128::new(10));
    assert_eq!(res.lineage_id, 1);

    // grant burner and burn
    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &ExecuteMsg::SetBurner {
            account: "burner".into(),
            status: true,
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("burner"),
        meat_addr.clone(),
        &ExecuteMsg::BurnSubtype {
            from: "user".into(),
            subtype: "GOATMEAT".into(),
            amount: Uint128::new(10),
        },
        &[],
    )
    .unwrap();

    let res2: BalanceSubtypeWithLineageResponse = app
        .wrap()
        .query_wasm_smart(
            meat_addr,
            &QueryMsg::BalanceOfSubtypeWithLineage {
                user: "user".into(),
                subtype: "GOATMEAT".into(),
            },
        )
        .unwrap();
    assert_eq!(res2.balance, Uint128::zero());
}
