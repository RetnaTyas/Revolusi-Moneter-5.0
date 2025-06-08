use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use meat::{execute, instantiate, query};
use ratehandler::{
    execute as handler_execute, instantiate as handler_instantiate, query as handler_query,
    ExecuteMsg as HandlerExecute, InstantiateMsg as HandlerInstantiate,
};
use starter::{execute as goat_execute, instantiate as goat_instantiate, query as goat_query};

use meat::msg::{ExecuteMsg as MeatExecute, InstantiateMsg as MeatInstantiate};
use starter::msg as goat_msg;

fn contract_goat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(
        goat_execute,
        goat_instantiate,
        goat_query,
    ))
}

fn contract_meat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

fn contract_handler() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(
        handler_execute,
        handler_instantiate,
        handler_query,
    ))
}

#[test]
fn rate_updates_affect_swaps() {
    let mut app = App::default();
    let goat_id = app.store_code(contract_goat());
    let meat_id = app.store_code(contract_meat());
    let handler_id = app.store_code(contract_handler());

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
    app.execute_contract(
        Addr::unchecked("owner"),
        goat_addr.clone(),
        &goat_msg::ExecuteMsg::SetMeatAddress {
            meat_address: meat_addr.to_string(),
        },
        &[],
    )
    .unwrap();
    let handler_addr = app
        .instantiate_contract(
            handler_id,
            Addr::unchecked("owner"),
            &HandlerInstantiate {},
            &[],
            "handler",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &MeatExecute::SetRateHandler {
            addr: handler_addr.to_string(),
        },
        &[],
    )
    .unwrap();

    // update rate to 50
    app.execute_contract(
        Addr::unchecked("owner"),
        handler_addr.clone(),
        &HandlerExecute::UpdateRate {
            new_rate: Uint128::new(50),
        },
        &[],
    )
    .unwrap();

    // give meat to user
    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &MeatExecute::Transfer {
            recipient: "user".into(),
            amount: Uint128::new(1000),
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        meat_addr.clone(),
        &MeatExecute::Approve {
            spender: meat_addr.to_string(),
            amount: Uint128::new(1000),
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        meat_addr.clone(),
        &MeatExecute::SwapMeatForGoat {
            meat_amount: Uint128::new(1000),
        },
        &[],
    )
    .unwrap();
    let goat_bal: goat_msg::BalanceResponse = app
        .wrap()
        .query_wasm_smart(
            goat_addr.clone(),
            &goat_msg::QueryMsg::Balance {
                address: "user".into(),
            },
        )
        .unwrap();
    assert_eq!(goat_bal.balance, Uint128::new(1000 / 50));

    // invalidate rate
    app.execute_contract(
        Addr::unchecked("owner"),
        handler_addr,
        &HandlerExecute::InvalidateRate {},
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &MeatExecute::Transfer {
            recipient: "user".into(),
            amount: Uint128::new(850),
        },
        &[],
    )
    .unwrap();
    app.execute_contract(
        Addr::unchecked("user"),
        meat_addr.clone(),
        &MeatExecute::Approve {
            spender: meat_addr.to_string(),
            amount: Uint128::new(850),
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        meat_addr.clone(),
        &MeatExecute::SwapMeatForGoat {
            meat_amount: Uint128::new(850),
        },
        &[],
    )
    .unwrap();
    let goat_bal2: goat_msg::BalanceResponse = app
        .wrap()
        .query_wasm_smart(
            goat_addr,
            &goat_msg::QueryMsg::Balance {
                address: "user".into(),
            },
        )
        .unwrap();
    // should add 10 GOAT more
    assert_eq!(goat_bal2.balance, Uint128::new(1000 / 50 + 850 / 85));
}
