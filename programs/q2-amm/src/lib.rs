pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("D4M1GT6qiWEMHfcPmorEHFqx5sV1vshcaoNXLuKnAs6c");

#[program]
pub mod q2_amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.initialize(seed, fee, authority, ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, min_x, min_y)
    }

    pub fn swap(ctx: Context<Swap>, is_x: bool, amount: u64, min: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount, min)
    }

    pub fn update_auther(ctx: Context<UpdateConfig>, new_auther: Option<Pubkey>) -> Result<()> {
        ctx.accounts.update_auther(new_auther)
    }

    pub fn update_lock(ctx: Context<UpdateConfig>, is_lock: bool) -> Result<()> {
        ctx.accounts.lock_pool(is_lock)
    }

    pub fn update_fee(ctx: Context<UpdateConfig>, fee: u16) -> Result<()> {
        ctx.accounts.update_fee(fee)
    }
}
