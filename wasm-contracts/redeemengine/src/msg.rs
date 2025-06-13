use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub meat_contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetRedeemConfig {
        subtype: String,
        grams_per_token_unit: u128,
        active: bool,
    },
    Redeem {
        subtype: String,
        amount: u128,
    },
    EmergencyWithdraw {
        subtype: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    RedeemConfig { subtype: String },
    Owner {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RedeemConfigResponse {
    pub grams_per_token_unit: u128,
    pub is_active: bool,
}
