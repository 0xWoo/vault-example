use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

pub fn process_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    if ctx.accounts.user.owner == ctx.accounts.system_program.key() {
        ctx.accounts.user.set_inner(User {
            vault: ctx.accounts.vault.key(),
            owner: ctx.accounts.owner.key(),
            bump: *ctx.bumps.get("user").unwrap(),
            amount: 0,
        });
        // assume there will be stakers less than 2^32 for now
        ctx.accounts.vault.total_users += 1;
    } else {
        assert_eq!(ctx.accounts.user.vault, ctx.accounts.vault.key());
        assert_eq!(ctx.accounts.user.owner, ctx.accounts.owner.key());
    }

    // token program will check user token account's balance and ownership
    // bc signer is user here
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    ctx.accounts.user.amount = ctx.accounts.user.amount.checked_add(amount).unwrap();
    ctx.accounts.vault.total_amount = ctx.accounts.vault.total_amount.checked_add(amount).unwrap();
    Ok(())
}

impl<'info> Deposit<'info> {
    pub fn validate(ctx: &Context<Deposit>) -> Result<()> {
        assert_eq!(
            ctx.accounts.user_token_account.mint,
            ctx.accounts.vault.token_mint
        );
        // skipped user_token_account.owner check bc owner will sign transfer transaction
        // skipped vault_token_account.mint check bc user_token_account.mint is checked
        assert_eq!(
            ctx.accounts.vault_token_account.owner,
            ctx.accounts.vault_authority.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [User::PREFIX, &owner.key().to_bytes()],
        bump,
        space = User::LEN,
        payer = owner,
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

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
