use crate::state::*;
use anchor_lang::prelude::*;

/// Compound interests
/// NOTE: last compounded index should be kept tracked in production to avoid compounding it more than once
/// interest_rate_bps: 10,000bps = 100.00%, 100bps = 1.00%
pub fn process_compound(ctx: Context<Compound>, interest_rate_bps: u16) -> Result<()> {
    let interest_amount = u64::try_from(
        (ctx.accounts.user.amount as u128)
            .checked_mul(interest_rate_bps as u128)
            .unwrap()
            .checked_div(1_0000)
            .unwrap(),
    )
    .unwrap();
    ctx.accounts.user.amount = ctx
        .accounts
        .user
        .amount
        .checked_add(interest_amount)
        .unwrap();
    ctx.accounts.vault.total_amount = ctx
        .accounts
        .vault
        .total_amount
        .checked_add(interest_amount)
        .unwrap();
    Ok(())
}

impl<'info> Compound<'info> {
    pub fn validate(_ctx: &Context<Compound>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Compound<'info> {
    #[account(mut, has_one = vault)]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub vault: Account<'info, Vault>,
}
