use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

pub fn process_initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.vault.set_inner(Vault {
        admin: ctx.accounts.admin.key(),
        token_mint: ctx.accounts.token_mint.key(),
        token_decimals: ctx.accounts.token_mint.decimals,
        authority_bump: *ctx.bumps.get("vault_authority").unwrap(),
        total_amount: 0,
        total_deposits: 0,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub admin: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    #[account(zero)]
    pub vault: Account<'info, Vault>,

    /// CHECK: seeded by vault address
    #[account(
        seeds = [Vault::AUTHORITY_PREFIX, &vault.key().to_bytes()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,
}
