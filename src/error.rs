use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Your details has already exists in voters list, you cannot create again!!")]
    VoterAlreadyExists {},

    #[error("Your details are not present in voters list")]
    VoterNotFound {},

    #[error("Unknown voter cannot become candidate, he has to register as voter first")]
    NotAVoter {},

    #[error("Voter has already been voted")]
    AlreadyVoted {},

    #[error("This candidate is not present in the candidates list, he has to register first")]
    NotACandidate {},
}
