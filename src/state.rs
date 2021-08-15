use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub voters: Vec<Voter>,
    pub candidates: Vec<Addr>,
    pub voted: Vec<Addr>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Voter {
    pub address: Addr,
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Votes {
    pub voters: Vec<String>,
    pub count: u64
}

pub const STATE: Item<State> = Item::new("state");
pub const VOTES: Map<&str, Votes> = Map::new("votes");