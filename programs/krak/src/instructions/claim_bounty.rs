use crate::errors::ErrorCode;
use crate::state::{HashBounty, HashType};
use anchor_lang::prelude::*;
use hex::encode;
use solana_program::{blake3::hashv as blake3_hashv, hash::hashv, keccak::hashv as keccak_hashv};

#[derive(Accounts)]
pub struct ClaimBounty<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"bounty", &bounty_acct.hash], bump)]
    pub bounty_acct: Account<'info, HashBounty>,
    #[account(mut)]
    pub creator: SystemAccount<'info>,
}

#[event]
pub struct BountyClaimed {
    pub claimer: Pubkey,
    pub amount: u64,
    pub hash: [u8; 32],
    pub cleartxt: String,
}

pub fn claim_bounty(
    ctx: Context<ClaimBounty>,
    clear_txt: String,
    hash_type: HashType,
) -> Result<()> {
    let computed_hash: [u8; 32] = match hash_type {
        HashType::SHA256 => hashv(&[clear_txt.as_bytes()]).to_bytes(),
        HashType::SHA3 => keccak_hashv(&[clear_txt.as_bytes()]).to_bytes(),
        HashType::BLAKE3 => blake3_hashv(&[clear_txt.as_bytes()]).to_bytes(),
    };
    let computed_hash_hex = encode(computed_hash);
    msg!(&computed_hash_hex);

    require!(
        computed_hash == ctx.accounts.bounty_acct.hash,
        ErrorCode::HashMismatch
    );

    let bounty_acct = &ctx.accounts.bounty_acct;

    // Transfer bounty amount to the claimant
    ctx.accounts.bounty_acct.sub_lamports(bounty_acct.amt)?;
    ctx.accounts.signer.add_lamports(bounty_acct.amt)?;

    // Emit the BountyClaimed event
    emit!(BountyClaimed {
        claimer: *ctx.accounts.signer.key,
        amount: bounty_acct.amt,
        hash: bounty_acct.hash,
        cleartxt: clear_txt
    });

    Ok(())
}
