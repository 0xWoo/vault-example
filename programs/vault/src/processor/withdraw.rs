use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

pub fn process_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // token program will check vault token account's balance and ownership
    // bc signer is vault authority here
    ctx.accounts.vault.authority_seeds(|signer_seeds| {
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault_token_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.vault_authority.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            amount,
        )
    })?;

    ctx.accounts.user.amount = ctx.accounts.user.amount.checked_sub(amount).unwrap();
    ctx.accounts.vault.total_amount = ctx.accounts.vault.total_amount.checked_sub(amount).unwrap();
    Ok(())
}

impl<'info> Withdraw<'info> {
    pub fn validate(ctx: &Context<Withdraw>, amount: u64) -> Result<()> {
        // skipped user_token_account.owner check bc vault_token_account.mint is checked
        assert_eq!(
            ctx.accounts.user_token_account.owner,
            ctx.accounts.user.owner
        );
        assert_eq!(
            ctx.accounts.vault_token_account.mint,
            ctx.accounts.vault.token_mint
        );
        // skipped vault_token_account.owner check bc vault authority will sign transfer transaction

        require!(
            ctx.accounts.user.amount < amount,
            CustomError::InsufficientBalance
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub owner: Signer<'info>,

    #[account(
        seeds = [User::PREFIX, &owner.key().to_bytes()],
        bump = user.bump,
        has_one = vault,
        has_one = owner,
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// CHECK: seeded by vault address
    #[account(
        seeds = [Vault::AUTHORITY_PREFIX, &vault.key().to_bytes()],
        bump = vault.authority_bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
