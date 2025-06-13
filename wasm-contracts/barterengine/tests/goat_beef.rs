use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use barterengine::msg::{ExecuteMsg, InstantiateMsg};
use barterengine::{execute, instantiate, query};
use meat::{execute as meat_execute, instantiate as meat_instantiate, query as meat_query};
use ratehandler::{
    execute as rate_execute, instantiate as rate_inst, query as rate_query,
    CommodityRepresentation, ExecuteMsg as RHExecute, InstantiateMsg as RHInstantiate,
    QueryMsg as RHQuery,
};

fn contract_engine() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

fn contract_rate() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(rate_execute, rate_inst, rate_query))
}

fn contract_meat() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(
        meat_execute,
        meat_instantiate,
        meat_query,
    ))
}

#[test]
fn goat_to_beef_barter() {
    let mut app = App::default();
    let rate_id = app.store_code(contract_rate());
    let meat_id = app.store_code(contract_meat());
    let eng_id = app.store_code(contract_engine());

    let rate_addr = app
        .instantiate_contract(
            rate_id,
            Addr::unchecked("owner"),
            &RHInstantiate {},
            &[],
            "rate",
            None,
        )
        .unwrap();
    let meat_addr = app
        .instantiate_contract(
            meat_id,
            Addr::unchecked("owner"),
            &meat::msg::InstantiateMsg {},
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
                rate_handler: rate_addr.to_string(),
                meat_contract: meat_addr.to_string(),
            },
            &[],
            "engine",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &meat::msg::ExecuteMsg::SetMinter {
            account: eng_addr.to_string(),
            status: true,
        },
        &[],
    )
    .unwrap();
    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &meat::msg::ExecuteMsg::SetBurner {
            account: eng_addr.to_string(),
            status: true,
        },
        &[],
    )
    .unwrap();

    let rep_goat = CommodityRepresentation {
        nft_address: "0x0".into(),
        token_virtual_address: "0x0".into(),
        token_product_address: meat_addr.to_string(),
        token_product_subtype: "GOATMEAT".into(),
        is_nft_active: true,
        is_token_virtual_active: true,
        is_token_product_active: true,
        lod_per_day_nft: Uint128::from(44_380000000000000000u128),
        lod_per_day_virtual: Uint128::from(44_380000000000000000u128),
        lod_per_day_product: Uint128::from(44_380000000000000000u128),
        protein_g_per_kg: Uint128::new(1),
        fat_g_per_kg: Uint128::new(1),
        micronutrient_index_x1000: Uint128::new(1),
        yield_per_cycle_kg: Uint128::new(1),
        cycle_time_days: Uint128::new(1),
    };
    let rep_beef = CommodityRepresentation {
        nft_address: "0x0".into(),
        token_virtual_address: "0x0".into(),
        token_product_address: meat_addr.to_string(),
        token_product_subtype: "BEEFMEAT".into(),
        is_nft_active: true,
        is_token_virtual_active: true,
        is_token_product_active: true,
        lod_per_day_nft: Uint128::from(281_290000000000000000u128),
        lod_per_day_virtual: Uint128::from(281_290000000000000000u128),
        lod_per_day_product: Uint128::from(281_290000000000000000u128),
        protein_g_per_kg: Uint128::new(1),
        fat_g_per_kg: Uint128::new(1),
        micronutrient_index_x1000: Uint128::new(1),
        yield_per_cycle_kg: Uint128::new(1),
        cycle_time_days: Uint128::new(1),
    };

    app.execute_contract(
        Addr::unchecked("owner"),
        rate_addr.clone(),
        &RHExecute::SetCommodityRepresentation {
            commodity_id: "GOATMEAT".into(),
            data: rep_goat.clone(),
        },
        &[],
    )
    .unwrap();
    app.execute_contract(
        Addr::unchecked("owner"),
        rate_addr.clone(),
        &RHExecute::SetCommodityRepresentation {
            commodity_id: "BEEFMEAT".into(),
            data: rep_beef.clone(),
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &meat::msg::ExecuteMsg::MintSubtype {
            to: "user".into(),
            subtype: "GOATMEAT".into(),
            amount: Uint128::new(10_000000000000000000u128),
        },
        &[],
    )
    .unwrap();
    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &meat::msg::ExecuteMsg::SetSubtypeLineage {
            user: "user".into(),
            subtype: "GOATMEAT".into(),
            lineage_id: 1,
        },
        &[],
    )
    .unwrap();

    let res = app
        .execute_contract(
            Addr::unchecked("user"),
            eng_addr.clone(),
            &ExecuteMsg::BarterProductToProduct {
                from_subtype: "GOATMEAT".into(),
                to_subtype: "BEEFMEAT".into(),
                from_amount: 2_000000000000000000u128,
            },
            &[],
        )
        .unwrap();
    assert!(res.events.iter().any(|e| e
        .attributes
        .iter()
        .any(|a| a.key == "action" && a.value == "barter")));

    let bal: meat::msg::BalanceSubtypeWithLineageResponse = app
        .wrap()
        .query_wasm_smart(
            meat_addr.clone(),
            &meat::msg::QueryMsg::BalanceOfSubtypeWithLineage {
                user: "user".into(),
                subtype: "BEEFMEAT".into(),
            },
        )
        .unwrap();
    assert_eq!(bal.lineage_id, 1);
}

#[test]
fn goat_beef_lineage_missing() {
    let mut app = App::default();
    let rate_id = app.store_code(contract_rate());
    let meat_id = app.store_code(contract_meat());
    let eng_id = app.store_code(contract_engine());

    let rate_addr = app
        .instantiate_contract(
            rate_id,
            Addr::unchecked("owner"),
            &RHInstantiate {},
            &[],
            "rate",
            None,
        )
        .unwrap();
    let meat_addr = app
        .instantiate_contract(
            meat_id,
            Addr::unchecked("owner"),
            &meat::msg::InstantiateMsg {},
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
                rate_handler: rate_addr.to_string(),
                meat_contract: meat_addr.to_string(),
            },
            &[],
            "engine",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &meat::msg::ExecuteMsg::SetMinter {
            account: eng_addr.to_string(),
            status: true,
        },
        &[],
    )
    .unwrap();
    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &meat::msg::ExecuteMsg::SetBurner {
            account: eng_addr.to_string(),
            status: true,
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        meat_addr.clone(),
        &meat::msg::ExecuteMsg::MintSubtype {
            to: "user".into(),
            subtype: "GOATMEAT".into(),
            amount: Uint128::new(1_000000000000000000u128),
        },
        &[],
    )
    .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            eng_addr,
            &ExecuteMsg::BarterProductToProduct {
                from_subtype: "GOATMEAT".into(),
                to_subtype: "BEEFMEAT".into(),
                from_amount: 1_000000000000000000u128,
            },
            &[],
        )
        .unwrap_err();
    assert!(err.to_string().contains("Invalid lineage"));
}
