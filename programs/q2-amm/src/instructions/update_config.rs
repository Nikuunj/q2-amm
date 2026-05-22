use anchor_lang::prelude::*;

use crate::{error::AmmErrorCode, state::Config};

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
}

impl<'info> UpdateConfig<'info> {
    pub fn update_auther(&mut self, new_auther: Option<Pubkey>) -> Result<()> {
        require_eq!(
            self.authority.key(),
            self.config.authority.unwrap(),
            AmmErrorCode::CustomError
        );
        self.config.authority = new_auther;
        Ok(())
    }

    pub fn lock_pool(&mut self, is_lock: bool) -> Result<()> {
        require_eq!(
            self.authority.key(),
            self.config.authority.unwrap(),
            AmmErrorCode::CustomError
        );

        require_neq!(self.config.locked, is_lock, AmmErrorCode::CustomError);
        self.config.locked = is_lock;
        Ok(())
    }

    pub fn update_fee(&mut self, fee: u16) -> Result<()> {
        require_eq!(
            self.authority.key(),
            self.config.authority.unwrap(),
            AmmErrorCode::CustomError
        );

        self.config.fee = fee;
        Ok(())
    }
}
