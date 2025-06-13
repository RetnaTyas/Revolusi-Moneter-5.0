use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint {
        to: String,
        nfc_id: String,
        breed: String,
        birth_year: u64,
        weight: u64,
    },
    Burn {
        token_id: String,
    },
    Approve {
        spender: String,
        token_id: String,
    },
    Transfer {
        to: String,
        token_id: String,
    },
    TransferFrom {
        owner: String,
        to: String,
        token_id: String,
    },
    UpdateWeight {
        token_id: String,
        new_weight: u64,
    },
    SetBurnHook {
        hook: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    SapiValue { token_id: u64 },
    SapiData { token_id: u64 },
    Owner { token_id: u64 },
    OwnerAddress {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SapiValueResponse {
    pub value: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SapiDataResponse {
    pub nfc_id: String,
    pub breed: String,
    pub birth_year: u64,
    pub weight: u64,
    pub minted_at: u64,
}
