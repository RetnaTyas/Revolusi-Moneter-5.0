use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use starter::{execute as goat_execute, instantiate as goat_instantiate, query as goat_query};
use meat::{execute, instantiate, query};
use starter::msg as goat_msg;
use meat::msg::{InstantiateMsg as MeatInstantiate, ExecuteMsg as MeatExecute, QueryMsg as MeatQuery, BalanceResponse};

fn contract_goat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(goat_execute, goat_instantiate, goat_query))
}

fn contract_meat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

#[test]
fn swap_meat_and_goat() {
    let mut app = App::default();
    let goat_id = app.store_code(contract_goat());
    let meat_id = app.store_code(contract_meat());

    let goat_addr = app.instantiate_contract(goat_id, Addr::unchecked("owner"), &starter::msg::InstantiateMsg { meat_contract: "meat".into() }, &[], "goat", None).unwrap();
    let meat_addr = app.instantiate_contract(meat_id, Addr::unchecked("owner"), &MeatInstantiate { goat_contract: goat_addr.to_string() }, &[], "meat", None).unwrap();

    app.execute_contract(Addr::unchecked("owner"), goat_addr.clone(), &goat_msg::ExecuteMsg::SetMeatAddress { meat_address: meat_addr.to_string() }, &[]).unwrap();

    // send meat to user
    app.execute_contract(Addr::unchecked("owner"), meat_addr.clone(), &MeatExecute::Transfer { recipient: "user".into(), amount: Uint128::new(1000) }, &[]).unwrap();

    app.execute_contract(Addr::unchecked("user"), meat_addr.clone(), &MeatExecute::SwapMeatForGoat { meat_amount: Uint128::new(1000) }, &[]).unwrap();
    let goat_bal: goat_msg::BalanceResponse = app.wrap().query_wasm_smart(goat_addr.clone(), &goat_msg::QueryMsg::Balance { address: "user".into() }).unwrap();
    assert_eq!(goat_bal.balance, Uint128::new(1000 / 85));

    app.execute_contract(Addr::unchecked("user"), goat_addr.clone(), &goat_msg::ExecuteMsg::Approve { spender: meat_addr.to_string(), amount: goat_bal.balance }, &[]).unwrap();
    app.execute_contract(Addr::unchecked("user"), meat_addr.clone(), &MeatExecute::SwapGoatForMeat { goat_amount: goat_bal.balance }, &[]).unwrap();
    let meat_bal: BalanceResponse = app.wrap().query_wasm_smart(meat_addr, &MeatQuery::Balance { address: "user".into() }).unwrap();
    assert!(meat_bal.balance > Uint128::zero());
}
