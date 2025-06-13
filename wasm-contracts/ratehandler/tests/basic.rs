use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use ratehandler::{execute, instantiate, query, CommodityRepresentation, ExecuteMsg, InstantiateMsg, QueryMsg, LODResponse, RateResponse};

fn contract_rate() -> Box<dyn cw_multi_test::Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

#[test]
fn registry_and_lod_queries() {
    let mut app = App::default();
    let code_id = app.store_code(contract_rate());
    let addr = app
        .instantiate_contract(code_id, Addr::unchecked("owner"), &InstantiateMsg {}, &[], "rate", None)
        .unwrap();

    let data = CommodityRepresentation {
        nft_address: "0x0".into(),
        token_virtual_address: "0x0".into(),
        token_product_address: "0x0".into(),
        token_product_subtype: "WHEATMEAT".into(),
        is_nft_active: true,
        is_token_virtual_active: true,
        is_token_product_active: true,
        lod_per_day_nft: Uint128::new(2),
        lod_per_day_virtual: Uint128::new(3),
        lod_per_day_product: Uint128::new(4),
        protein_g_per_kg: Uint128::new(1),
        fat_g_per_kg: Uint128::new(1),
        micronutrient_index_x1000: Uint128::new(1),
        yield_per_cycle_kg: Uint128::new(1),
        cycle_time_days: Uint128::new(1),
    };

    app.execute_contract(
        Addr::unchecked("owner"),
        addr.clone(),
        &ExecuteMsg::SetCommodityRepresentation {
            commodity_id: "WHEAT".into(),
            data: data.clone(),
        },
        &[],
    )
    .unwrap();

    let res: LODResponse = app
        .wrap()
        .query_wasm_smart(
            addr.clone(),
            &QueryMsg::GetLODPerDay {
                commodity_id: "WHEAT".into(),
                layer: "NFT".into(),
            },
        )
        .unwrap();
    assert_eq!(res.lod_per_day, Uint128::new(2));
}

#[test]
fn compute_rate_product_product() {
    let mut app = App::default();
    let code_id = app.store_code(contract_rate());
    let addr = app
        .instantiate_contract(code_id, Addr::unchecked("owner"), &InstantiateMsg {}, &[], "rate", None)
        .unwrap();

    let wheat = CommodityRepresentation {
        nft_address: "0x0".into(),
        token_virtual_address: "0x0".into(),
        token_product_address: "0x0".into(),
        token_product_subtype: "WHEATMEAT".into(),
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

    let rice = CommodityRepresentation {
        nft_address: "0x0".into(),
        token_virtual_address: "0x0".into(),
        token_product_address: "0x0".into(),
        token_product_subtype: "RICEMEAT".into(),
        is_nft_active: true,
        is_token_virtual_active: true,
        is_token_product_active: true,
        lod_per_day_nft: Uint128::new(1),
        lod_per_day_virtual: Uint128::new(1),
        lod_per_day_product: Uint128::new(10),
        protein_g_per_kg: Uint128::new(1),
        fat_g_per_kg: Uint128::new(1),
        micronutrient_index_x1000: Uint128::new(1),
        yield_per_cycle_kg: Uint128::new(1),
        cycle_time_days: Uint128::new(1),
    };

    app.execute_contract(
        Addr::unchecked("owner"),
        addr.clone(),
        &ExecuteMsg::SetCommodityRepresentation {
            commodity_id: "WHEAT".into(),
            data: wheat,
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        addr.clone(),
        &ExecuteMsg::SetCommodityRepresentation {
            commodity_id: "RICE".into(),
            data: rice,
        },
        &[],
    )
    .unwrap();

    let rate: RateResponse = app
        .wrap()
        .query_wasm_smart(
            addr,
            &QueryMsg::GetRate {
                from_commodity: "WHEAT".into(),
                from_layer: "PRODUCT".into(),
                to_commodity: "RICE".into(),
                to_layer: "PRODUCT".into(),
            },
        )
        .unwrap();
    assert_eq!(rate.rate, Uint128::from((4u128 * 1_000_000_000_000_000_000u128) / 10u128));
}
