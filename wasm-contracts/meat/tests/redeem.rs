use cosmwasm_std::{coin, Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use starter::{execute as goat_execute, instantiate as goat_instantiate, query as goat_query};
use meat::{execute, instantiate, query};
use meat::msg::{InstantiateMsg as MeatInstantiate, ExecuteMsg as MeatExecute, QueryMsg as MeatQuery, BalanceResponse, TokenInfoResponse};

fn contract_goat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(goat_execute, goat_instantiate, goat_query))
}

fn contract_meat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

#[test]
fn redeem_burns_supply() {
    let mut app = App::new(|router, _, storage| {
        router.bank.init_balance(storage, &Addr::unchecked("owner"), vec![coin(1, "ucosm")]).unwrap();
        router.bank.init_balance(storage, &Addr::unchecked("user"), vec![coin(2000, "ucosm")]).unwrap();
    });
    let goat_id = app.store_code(contract_goat());
    let meat_id = app.store_code(contract_meat());

    let goat_addr = app.instantiate_contract(goat_id, Addr::unchecked("owner"), &starter::msg::InstantiateMsg { meat_contract: "meat".into() }, &[], "goat", None).unwrap();
    let meat_addr = app.instantiate_contract(meat_id, Addr::unchecked("owner"), &MeatInstantiate { goat_contract: goat_addr.to_string() }, &[], "meat", None).unwrap();

    app.execute_contract(Addr::unchecked("user"), meat_addr.clone(), &MeatExecute::MintWithNative {}, &[coin(1000, "ucosm")]).unwrap();

    let info_before: TokenInfoResponse = app.wrap().query_wasm_smart(meat_addr.clone(), &MeatQuery::TokenInfo {}).unwrap();

    let resp = app.execute_contract(Addr::unchecked("user"), meat_addr.clone(), &MeatExecute::RedeemForMeat { amount: Uint128::new(50) }, &[]).unwrap();
    assert!(resp.events.iter().any(|e| e.ty == "wasm" && e.attributes.iter().any(|a| a.key == "action" && a.value == "MeatRedeemed")));

    let bal: BalanceResponse = app.wrap().query_wasm_smart(meat_addr.clone(), &MeatQuery::Balance { address: "user".into() }).unwrap();
    assert_eq!(bal.balance, Uint128::new(50));
    let info_after: TokenInfoResponse = app.wrap().query_wasm_smart(meat_addr, &MeatQuery::TokenInfo {}).unwrap();
    assert_eq!(info_after.total_supply, info_before.total_supply - Uint128::new(50));
}
