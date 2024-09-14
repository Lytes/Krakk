use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid hash length.")]
    InvalidHashLength,
    #[msg("Invalid extra info length.")]
    InvalidExtraInfoLength,
    #[msg("Hash mismatch.")]
    HashMismatch,
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Bounty cannot be removed yet.")]
    BountyNotRemovable,
}
