use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, CustomMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub retry_delay: u64,
    pub job_id: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    PutWithdraw {
        deposit_ids: Vec<u32>,
        profit_taking_or_stop_loss: Vec<bool>,
    },
}

/// Message struct for cross-chain calls.
#[cw_serde]
pub struct PalomaMsg {
    /// The ID of the paloma scheduled job to run.
    pub job_id: String,
    /// The payload, ABI encoded for the target chain.
    pub payload: Binary,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetJobIdResponse)]
    GetJobId {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetJobIdResponse {
    pub job_id: String,
}

impl CustomMsg for PalomaMsg {}
