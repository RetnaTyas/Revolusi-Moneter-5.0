use cosmwasm_std::{coin, Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use meat::msg::{ExecuteMsg as MeatExec, InstantiateMsg as MeatInstantiate};
use meat::{execute as meat_execute, instantiate as meat_init, query as meat_query};
use redeemengine::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, RedeemConfigResponse};
use redeemengine::{execute, instantiate, query};
use starter::{execute as goat_execute, instantiate as goat_init, query as goat_query};

fn contract_goat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(goat_execute, goat_init, goat_query))
}

fn contract_meat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(meat_execute, meat_init, meat_query))
}

fn contract_engine() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

#[test]
fn redeem_burns_meat() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("user"), vec![coin(1000, "ucosm")])
            .unwrap();
    });
    let goat_id = app.store_code(contract_goat());
    let meat_id = app.store_code(contract_meat());
    let eng_id = app.store_code(contract_engine());

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

    let meat_addr = app
        .instantiate_contract(
            meat_id,
            Addr::unchecked("owner"),
            &MeatInstantiate {
                goat_contract: goat_addr.to_string(),
            },
            &[],
            "meat",
            None,
        )
        .unwrap();

    let eng_addr = app
        .instantiate_contract(
            eng_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                meat_contract: meat_addr.to_string(),
            },
            &[],
            "engine",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        meat_addr.clone(),
        &MeatExec::MintWithNative {},
        &[coin(1000, "ucosm")],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        eng_addr.clone(),
        &ExecuteMsg::SetRedeemConfig {
            subtype: "GOATMEAT".into(),
            grams_per_token_unit: 100,
            active: true,
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        meat_addr.clone(),
        &MeatExec::Approve {
            spender: eng_addr.to_string(),
            amount: Uint128::new(50),
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        eng_addr.clone(),
        &ExecuteMsg::Redeem {
            subtype: "GOATMEAT".into(),
            amount: 50,
        },
        &[],
    )
    .unwrap();

    let bal: meat::msg::BalanceResponse = app
        .wrap()
        .query_wasm_smart(
            meat_addr,
            &meat::msg::QueryMsg::Balance {
                address: "user".into(),
            },
        )
        .unwrap();
    assert_eq!(bal.balance, Uint128::new(50));
}
