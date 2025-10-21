use anchor_lang::prelude::*;

pub mod errors;

declare_id!("HCEwd2GpRPY7xgdGzKKJXKNJ8EUeENytA5dw9wJ4hEZC");

#[program]
pub mod bonding_curve {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
