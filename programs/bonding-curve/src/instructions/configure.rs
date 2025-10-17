use anchor_lang::prelude::*;

use crate::{
    errors::BondingCurveError,
    states::config::{Config, ConfigSettings},
};

#[derive(Accounts)]
pub struct Configure<'info> {
    #[account(mut)]
    admin: Signer<'info>,

    #[account(init_if_needed,
        payer = admin,
        seeds = [Config::SEED_PREFIX.as_bytes()],
        bump,
        space = 8 + Config::LEN
    )]
    global_config: Account<'info, Config>,

    system_program: Program<'info, System>,
}

impl<'info> Configure<'info> {
    pub fn process(&mut self, new_config: ConfigSettings) -> Result<()> {
        let config_account = &mut self.global_config;

        // setting the admin as authority if not set already
        if self.global_config.authority.eq(&Pubkey::default()) {
            self.global_config.authority = self.admin.key();
        } else {
            require! {
                self.global_config.authority.eq(&self.admin.key()),
                BondingCurveError::UnauthorizedAddress
            }

            require! {
                new_config.authority.eq(&self.global_config.authority),
                BondingCurveError::UnauthorizedAddress
            }
        }

        require! {
            !new_config.authority.eq(&Pubkey::default()),
            BondingCurveError::UnauthorizedAddress
        }

        // we will copy over all the vaules from new_config to global_config

        self.global_config.authority = new_config.authority;
        self.global_config.fee_recipient = new_config.fee_recipient;
        self.global_config.curve_limit = new_config.curve_limit;
        self.global_config.initial_virtual_token_reserve = new_config.initial_virtual_token_reserve;
        self.global_config.initial_virtual_sol_reserve = new_config.initial_virtual_sol_reserve;
        self.global_config.initial_real_token_reserve = new_config.initial_real_token_reserve;
        self.global_config.initial_token_supply = new_config.initial_token_supply;
        self.global_config.buy_fee_percentage = new_config.buy_fee_percentage;
        self.global_config.sell_fee_percentage = new_config.sell_fee_percentage;
        self.global_config.migration_fee_percentage = new_config.migration_fee_percentage;
        self.global_config.reserved = new_config.reserved;

        Ok(())
    }
}
