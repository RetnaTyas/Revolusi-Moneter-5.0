use cosmwasm_std::{Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub meat_contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    Transfer { recipient: String, amount: Uint128 },
    Approve { spender: String, amount: Uint128 },
    TransferFrom { owner: String, recipient: String, amount: Uint128 },
    MintTo { to: String, amount: Uint128 },
    BurnAndMint { token_id: u64 },
    Stake { amount: Uint128 },
    EmergencyUnstake {},
    Unstake {},
    ClaimReward {},
    CompoundReward {},
    SetMeatAddress { meat_address: String },
    SetNftAddress { nft_address: String },
    SetRewardConfig { new_rate: Uint128, new_interval: u64, new_min_claim: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    Balance { address: String },
    Allowance { owner: String, spender: String },
    TokenInfo {},
    StakingBalance { address: String },
    PendingReward { address: String },
    NextClaimTime { address: String },
    Owner {},
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
pub struct StakingInfoResponse {
    pub balance: Uint128,
    pub last_staked: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PendingRewardResponse {
    pub reward: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GoatValueResponse {
    pub value: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NextClaimResponse {
    pub timestamp: u64,
}

