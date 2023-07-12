use anchor_lang::prelude::*;

use crate::located::Located;

#[account]
pub struct Vault {
    /// vault administrator
    pub admin: Pubkey,

    /// SPL token mint
    pub token_mint: Pubkey,

    /// save decimals to save rpc calls in web3 client
    pub token_decimals: u8,

    /// PDA bump for vault authority
    pub authority_bump: u8,

    /// total staked amount
    pub total_amount: u64,

    /// total users staked
    pub total_users: u32,
}

impl Vault {
    pub const AUTHORITY_PREFIX: &'static [u8] = b"Vault Authority";
}

pub trait VaultAuthoritySeeds {
    fn authority_seeds<R, F: FnOnce(&[&[&[u8]]]) -> R>(&self, f: F) -> R;
}

impl<T> VaultAuthoritySeeds for T
where
    T: Located<Vault>,
{
    fn authority_seeds<R, F: FnOnce(&[&[&[u8]]]) -> R>(&self, f: F) -> R {
        f(&[&[
            Vault::AUTHORITY_PREFIX,
            &self.key().to_bytes(),
            &[self.as_ref().authority_bump],
        ]])
    }
}

/// PDA
#[account]
pub struct User {
    /// address of the staked vault
    pub vault: Pubkey,

    /// address of the staked user
    pub owner: Pubkey,

    /// PDA bump for user
    pub bump: u8,

    /// accumulated deposited amount
    pub amount: u64,
}

impl User {
    pub const PREFIX: &'static [u8] = b"User";
    pub const LEN: usize = 81; //8 + 32 + 32 + 1 + 8;
}
