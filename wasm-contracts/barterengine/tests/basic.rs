use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use barterengine::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, RateResponse};
use barterengine::{execute, instantiate, query};
use ratehandler::{
    execute as rate_execute, instantiate as rate_inst, query as rate_query, CommodityRepresentation,
    ExecuteMsg as RHExecute, InstantiateMsg as RHInstantiate, QueryMsg as RHQuery,
};

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

    let data_a = CommodityRepresentation {
        nft_address: "0x0".into(),
        token_virtual_address: "0x0".into(),
        token_product_address: "0x0".into(),
        token_product_subtype: "AMEAT".into(),
        is_nft_active: true,
        is_token_virtual_active: true,
        is_token_product_active: true,
        lod_per_day_nft: Uint128::new(2),
        lod_per_day_virtual: Uint128::new(2),
        lod_per_day_product: Uint128::new(4),
        protein_g_per_kg: Uint128::new(1),
        fat_g_per_kg: Uint128::new(1),
        micronutrient_index_x1000: Uint128::new(1),
        yield_per_cycle_kg: Uint128::new(1),
        cycle_time_days: Uint128::new(1),
    };

    let data_b = CommodityRepresentation {
        nft_address: "0x0".into(),
        token_virtual_address: "0x0".into(),
        token_product_address: "0x0".into(),
        token_product_subtype: "BMEAT".into(),
        is_nft_active: true,
        is_token_virtual_active: true,
        is_token_product_active: true,
        lod_per_day_nft: Uint128::new(1),
        lod_per_day_virtual: Uint128::new(1),
        lod_per_day_product: Uint128::new(8),
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
            commodity_id: "A".into(),
            data: data_a,
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        rate_addr.clone(),
        &RHExecute::SetCommodityRepresentation {
            commodity_id: "B".into(),
            data: data_b,
        },
        &[],
    )
    .unwrap();

    let rate: RateResponse = app
        .wrap()
        .query_wasm_smart(
            eng_addr,
            &QueryMsg::GetRate { from_subtype: "A".into(), to_subtype: "B".into() },
        )
        .unwrap();
    assert_eq!(rate.rate, ((4u128 * 1_000_000_000_000_000_000u128) / 8u128));
}
