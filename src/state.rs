use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

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

pub const STATE: Item<State> = Item::new("state");
