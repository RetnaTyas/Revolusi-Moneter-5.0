use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128, WasmMsg,
};
use cw2::set_contract_version;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{MEAT, OWNER, SAPI_NFT};

pub mod msg;
pub mod state;

const CONTRACT_NAME: &str = "sapinftburnhook";
const CONTRACT_VERSION: &str = "0.1.0";

const BEEFMEAT_SUBTYPE: &str = "BEEFMEAT";

const SLAUGHTER_YIELD_BPS: u64 = 6500;
const WEIGHT_DECIMALS: u32 = 1;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;
    SAPI_NFT.save(deps.storage, &deps.api.addr_validate(&msg.nft_contract)?)?;
    MEAT.save(deps.storage, &deps.api.addr_validate(&msg.meat_contract)?)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::OnBurn { to, weight } => execute_on_burn(deps, info, to, weight),
    }
}

fn execute_on_burn(
    deps: DepsMut,
    info: MessageInfo,
    to: String,
    weight: u64,
) -> StdResult<Response> {
    let nft = SAPI_NFT.load(deps.storage)?;
    if info.sender != nft {
        return Err(StdError::generic_err("Unauthorized"));
    }
    if weight == 0 {
        return Ok(Response::default());
    }
    let amount = weight as u128 * 1_000_000_000_000_000_000u128 * SLAUGHTER_YIELD_BPS as u128
        / 10u128.pow(WEIGHT_DECIMALS)
        / 10_000u128;
    let meat = MEAT.load(deps.storage)?;
    let mint = WasmMsg::Execute {
        contract_addr: meat.to_string(),
        msg: to_json_binary(&meat::msg::ExecuteMsg::MintSubtype {
            to: to.clone(),
            subtype: BEEFMEAT_SUBTYPE.into(),
            amount: Uint128::new(amount),
        })?,
        funds: vec![],
    };
    Ok(Response::new()
        .add_message(mint)
        .add_attribute("action", "BeefMeatMinted")
        .add_attribute("to", to)
        .add_attribute("amount", amount.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner {} => {
            let owner = OWNER.load(deps.storage)?;
            Ok(to_json_binary(&owner.into_string())?)
        }
    }
}
