use cosmwasm_std::{
    to_binary, Deps, entry_point, DepsMut, Env, MessageInfo, Response, Addr, StdResult, Binary
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, VotersResponse, CandidatesResponse, VotesResponse};
use crate::state::{State, Voter, STATE, VOTES, Votes};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        voters: vec![],
        candidates: vec![],
        voted: vec![]
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateVoter {address, name} => create_voter(deps, address, name),
        ExecuteMsg::CastVote {voter, candidate} => cast_vote(deps, voter, candidate),
        ExecuteMsg::Register { voter } => register(deps, voter),
    }
}

fn is_voter(voter: &Addr, state: &State) -> bool {
    state.voters.iter().any(|i| &i.address == voter)
}

pub fn create_voter(deps: DepsMut, voter_address: String, name: String) -> Result<Response, ContractError> {
    let voter = Voter {
        address: deps.api.addr_validate(&voter_address.as_str())?,
        name: name
    };
    let state = STATE.load(deps.storage)?;

    if is_voter(&voter.address, &state) {
        return Err(ContractError::VoterAlreadyExists {});
    }

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.voters.push(voter);
        Ok(state)
    })?;

    Ok(Response::default())
}

fn has_voted(voter: &Addr, state: &State) -> bool {
    state.voted.iter().any(|i| i == voter)
}

fn is_candidate(candidate: &Addr, state: &State) -> bool {
    state.candidates.iter().any(|i| i == candidate)
}

pub fn cast_vote(deps: DepsMut, voter: String, candidate: String) -> Result<Response, ContractError> {
    let voter_address = deps.api.addr_validate(&voter.as_str())?;
    let candidate_address = deps.api.addr_validate(&candidate.as_str())?;

    let state = STATE.load(deps.storage)?;

    if has_voted(&voter_address, &state) {
        return Err(ContractError::AlreadyVoted {});
    }

    if is_voter(&voter_address, &state) == false {
        return Err(ContractError::VoterNotFound {});
    }

    if is_candidate(&candidate_address, &state) == false {
        return Err(ContractError::NotACandidate {});
    }

    let mut votes= VOTES.load(deps.storage, &candidate)?;
    votes.voters.push(voter);
    votes.count += 1;

    VOTES.save(deps.storage, &candidate, &votes)?;

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.voted.push(voter_address);
        Ok(state)
    })?;

    Ok(Response::default())
}

pub fn register(deps: DepsMut, voter_address: String) -> Result<Response, ContractError> {
    let address = deps.api.addr_validate(&voter_address.as_str())?;
    let state = STATE.load(deps.storage)?;
    if is_voter(&address, &state) == false {
        return Err(ContractError::NotAVoter {});
    }
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.candidates.push(address);
        Ok(state)
    })?;
    let votes = Votes {
        voters: vec![],
        count: 0
    };
    VOTES.save(deps.storage, &voter_address, &votes)?;
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&STATE.load(deps.storage)?),
        QueryMsg::Voters {} => to_binary(&query_voters(deps)?),
        QueryMsg::Candidates {} => to_binary(&query_candidates(deps)?),
        QueryMsg::Votes {candidate} => to_binary(&query_votes(deps, candidate)?)
    }
}

fn query_voters(deps: Deps) -> StdResult<VotersResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(VotersResponse { voters: state.voters })
}

fn query_candidates(deps: Deps) -> StdResult<CandidatesResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CandidatesResponse { candidates: state.candidates })
}

fn query_votes(deps: Deps, candidate: String) -> StdResult<VotesResponse> {
    let votes = VOTES.load(deps.storage, &candidate)?;
    Ok(VotesResponse { voters: votes.voters, count: votes.count })
}