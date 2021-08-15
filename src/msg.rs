use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{Voter};
use cosmwasm_std::{Addr};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateVoter { address: String, name: String},
    CastVote {voter: String, candidate: String},
    Register { voter: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Voters {},
    Candidates {},
    Votes { candidate: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VotersResponse {
    pub voters: Vec<Voter>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CandidatesResponse {
    pub candidates: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VotesResponse {
    pub voters: Vec<String>,
    pub count: u64
}
