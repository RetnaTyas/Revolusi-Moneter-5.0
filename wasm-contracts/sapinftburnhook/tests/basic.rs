use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, ContractWrapper, Executor};

use sapinftburnhook::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use sapinftburnhook::{execute, instantiate, query};

fn contract_hook() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

#[test]
fn owner_query_works() {
    let mut app = App::default();
    let hook_id = app.store_code(contract_hook());
    let hook_addr = app
        .instantiate_contract(
            hook_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                nft_contract: "nft".into(),
                meat_contract: "meat".into(),
            },
            &[],
            "hook",
            None,
        )
        .unwrap();
    let owner: String = app
        .wrap()
        .query_wasm_smart(hook_addr, &QueryMsg::Owner {})
        .unwrap();
    assert_eq!(owner, "owner");
}
