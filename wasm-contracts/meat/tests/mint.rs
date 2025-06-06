use cosmwasm_std::{Addr, Empty, Uint128, coin};
use cw_multi_test::{App, ContractWrapper, Executor};

use starter::{execute as goat_execute, instantiate as goat_instantiate, query as goat_query};
use meat::{execute, instantiate, query};
use meat::msg::{InstantiateMsg as MeatInstantiate, ExecuteMsg as MeatExecute, QueryMsg as MeatQuery, BalanceResponse};

fn contract_goat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(goat_execute, goat_instantiate, goat_query))
}

fn contract_meat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

#[test]
fn direct_send_does_not_mint() {
    let mut app = App::new(|router, _, storage| {
        router.bank.init_balance(storage, &Addr::unchecked("owner"), vec![coin(1, "ucosm")]).unwrap();
        router.bank.init_balance(storage, &Addr::unchecked("user"), vec![coin(2000, "ucosm")]).unwrap();
    });
    let goat_id = app.store_code(contract_goat());
    let meat_id = app.store_code(contract_meat());

    let goat_addr = app.instantiate_contract(goat_id, Addr::unchecked("owner"), &starter::msg::InstantiateMsg { meat_contract: "meat".into() }, &[], "goat", None).unwrap();
    let meat_addr = app.instantiate_contract(meat_id, Addr::unchecked("owner"), &MeatInstantiate { goat_contract: goat_addr.to_string() }, &[], "meat", None).unwrap();

    app.send_tokens(Addr::unchecked("user"), meat_addr.clone(), &[coin(1000, "ucosm")]).unwrap();

    let bal: BalanceResponse = app.wrap().query_wasm_smart(meat_addr, &MeatQuery::Balance { address: "user".into() }).unwrap();
    assert_eq!(bal.balance, Uint128::zero());
}

#[test]
fn mint_with_native_mints_meat() {
    let mut app = App::new(|router, _, storage| {
        router.bank.init_balance(storage, &Addr::unchecked("owner"), vec![coin(1, "ucosm")]).unwrap();
        router.bank.init_balance(storage, &Addr::unchecked("user"), vec![coin(2000, "ucosm")]).unwrap();
    });
    let goat_id = app.store_code(contract_goat());
    let meat_id = app.store_code(contract_meat());

    let goat_addr = app.instantiate_contract(goat_id, Addr::unchecked("owner"), &starter::msg::InstantiateMsg { meat_contract: "meat".into() }, &[], "goat", None).unwrap();
    let meat_addr = app.instantiate_contract(meat_id, Addr::unchecked("owner"), &MeatInstantiate { goat_contract: goat_addr.to_string() }, &[], "meat", None).unwrap();

    let resp = app.execute_contract(Addr::unchecked("user"), meat_addr.clone(), &MeatExecute::MintWithNative {}, &[coin(1000, "ucosm")]).unwrap();
    assert!(resp.events.iter().any(|e| e.ty == "wasm" && e.attributes.iter().any(|a| a.key == "action" && a.value == "MintedWithNative")));
    let bal: BalanceResponse = app.wrap().query_wasm_smart(meat_addr, &MeatQuery::Balance { address: "user".into() }).unwrap();
    assert_eq!(bal.balance, Uint128::new(100));
}
