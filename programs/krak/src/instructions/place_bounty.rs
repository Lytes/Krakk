use crate::errors::ErrorCode;
use crate::state::HashBounty;
use anchor_lang::{prelude::*, system_program};

#[derive(Accounts)]
#[instruction(amount: u64, hash: [u8;32])]
pub struct PlaceBounty<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer = signer, space = 8 + 177, seeds = [b"bounty", hash.as_ref()], bump)]
    pub bounty_acct: Account<'info, HashBounty>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct BountyPlaced {
    pub amount: u64,
    pub hash: [u8; 32],
}

pub fn place_bounty(
    ctx: Context<PlaceBounty>,
    amount: u64,
    hash: [u8; 32],
    extra_info: String,
) -> Result<()> {
    require!(hash.len() == 32, ErrorCode::InvalidHashLength);
    require!(extra_info.len() <= 96, ErrorCode::InvalidExtraInfoLength);

    let bounty_acct = &mut ctx.accounts.bounty_acct;

    let bounty_amount = bounty_acct.amt;
    let bounty_hash = bounty_acct.hash;

    bounty_acct.amt = amount;
    bounty_acct.creator = *ctx.accounts.signer.key;
    bounty_acct.hash = hash;
    bounty_acct.time_created = Clock::get()?.unix_timestamp as u64;
    bounty_acct.extra_info = extra_info;

    // Transfer lamports to the bounty account
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.signer.to_account_info().clone(),
            to: ctx.accounts.bounty_acct.to_account_info().clone(),
        },
    );
    system_program::transfer(cpi_context, amount)?;

    // Emit the BountyPlaced event
    emit!(BountyPlaced {
        amount: bounty_amount,
        hash: bounty_hash,
    });

    Ok(())
}
