use crate::errors::ErrorCode;
use crate::state::HashBounty;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RemoveBounty<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, close = signer, seeds = [b"bounty", &bounty_acct.hash], bump)]
    pub bounty_acct: Account<'info, HashBounty>,
}

pub fn remove_bounty(ctx: Context<RemoveBounty>) -> Result<()> {
    let bounty_acct = &ctx.accounts.bounty_acct;

    require!(
        bounty_acct.creator == *ctx.accounts.signer.key,
        ErrorCode::Unauthorized
    );
    require!(
        Clock::get()?.unix_timestamp as u64 >= bounty_acct.time_created + 14 * 24 * 60 * 60,
        ErrorCode::BountyNotRemovable
    );

    // Transfer the bounty amount back to the creator
    ctx.accounts.bounty_acct.sub_lamports(bounty_acct.amt)?;
    ctx.accounts.signer.add_lamports(bounty_acct.amt)?;

    Ok(())
}
