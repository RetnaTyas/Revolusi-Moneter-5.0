use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use barterengine::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, RateResponse};
use barterengine::{execute, instantiate, query};
use ratehandler::{execute as rate_execute, instantiate as rate_inst, query as rate_query, InstantiateMsg as RHInstantiate, ExecuteMsg as RHExecute};

fn contract_engine() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

fn contract_rate() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(rate_execute, rate_inst, rate_query))
}

#[test]
fn query_rate_forward() {
    let mut app = App::default();
    let rate_id = app.store_code(contract_rate());
    let eng_id = app.store_code(contract_engine());

    let rate_addr = app
        .instantiate_contract(rate_id, Addr::unchecked("owner"), &RHInstantiate {}, &[], "rate", None)
        .unwrap();
    let eng_addr = app
        .instantiate_contract(
            eng_id,
            Addr::unchecked("owner"),
            &InstantiateMsg { rate_handler: rate_addr.to_string(), meat_contract: "meat".into() },
            &[],
            "engine",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        rate_addr.clone(),
        &RHExecute::UpdateRate { new_rate: Uint128::new(200) },
        &[],
    )
    .unwrap();

    let rate: RateResponse = app
        .wrap()
        .query_wasm_smart(
            eng_addr,
            &QueryMsg::GetRate { from_subtype: "a".into(), to_subtype: "b".into() },
        )
        .unwrap();
    assert_eq!(rate.rate, 200u128);
}
