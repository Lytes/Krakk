use crate::state::HashType;
use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("6YuaHDGkiwMhSj8y5rkznTPohHJpeU96LnCq3U7dRkz5");

#[program]
pub mod hash_bounty {
    use super::*;

    pub fn place_bounty(
        ctx: Context<PlaceBounty>,
        amount: u64,
        hash: [u8; 32],
        extra_info: String,
    ) -> Result<()> {
        instructions::place_bounty(ctx, amount, hash, extra_info)
    }

    pub fn claim_bounty(
        ctx: Context<ClaimBounty>,
        clear_txt: String,
        hash_type: HashType,
    ) -> Result<()> {
        instructions::claim_bounty(ctx, clear_txt, hash_type)
    }

    pub fn remove_bounty(ctx: Context<RemoveBounty>) -> Result<()> {
        instructions::remove_bounty(ctx)
    }
}
