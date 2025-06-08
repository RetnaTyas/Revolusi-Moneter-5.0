use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub goat_contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Transfer {
        recipient: String,
        amount: Uint128,
    },
    Approve {
        spender: String,
        amount: Uint128,
    },
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    MintWithNative {},
    WithdrawNative {
        to: Option<String>,
    },
    ChangeDepositRate {
        new_rate: Uint128,
    },
    SwapGoatForMeat {
        goat_amount: Uint128,
    },
    SwapMeatForGoat {
        meat_amount: Uint128,
    },
    RedeemForMeat {
        amount: Uint128,
    },
    SetSwapEnabled {
        enabled: bool,
    },
    SetGoatAddress {
        goat_address: String,
    },
    SetRateHandler {
        addr: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { address: String },
    Allowance { owner: String, spender: String },
    TokenInfo {},
    DepositRate {},
    SwapEnabled {},
    Owner {},
    EquivalentMeat { goat_amount: Uint128 },
    EquivalentGoat { meat_amount: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllowanceResponse {
    pub allowance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfoResponse {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RateResponse {
    pub rate: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EnabledResponse {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EquivalentResponse {
    pub amount: Uint128,
}
