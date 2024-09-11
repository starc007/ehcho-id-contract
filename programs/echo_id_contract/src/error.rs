use anchor_lang::prelude::*;

#[error_code]
pub enum EchoIDError {
    #[msg("Invalid instruction")]
    InvalidInstruction,

    #[msg("Invalid signer")]
    InvalidSigner,

    #[msg("Invalid derived account")]
    InvalidDerivedAccount,

    #[msg("Alias already exists")]
    AliasAlreadyExists,

    #[msg("Alias does not exist")]
    AliasDoesNotExist,

    #[msg("Invalid owner")]
    InvalidOwner,

    #[msg("Alias is too long")]
    AliasTooLong,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Chain ID already exists for this alias")]
    ChainIDAlreadyExists,

    #[msg("Maximum number of chain mappings reached")]
    MaxChainMappingsReached,

    #[msg("Invalid reputation change")]
    InvalidReputationChange,

    #[msg("Reputation account not initialized")]
    ReputationAccountNotInitialized,

    #[msg("Cross-chain account not initialized")]
    CrossChainAccountNotInitialized,
}