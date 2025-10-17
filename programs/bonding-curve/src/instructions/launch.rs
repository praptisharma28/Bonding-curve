use anchor_lang::prelude::*;

#[account]
pub struct BondingCurve {
    pub virtual_token_reserve: u64,
    pub virtual_sol_reserve: u64,

    pub real_token_reserve: u64,
    pub sol_reserve: u64,

    pub token_total_supply: u64,

    pub is_compeleted: bool,

    pub is_migrated: bool,

    pub reserved: [u8; 128],
}

impl<'info> BondingCurve {
    pub const SEED_PREFIX: &'static str = "bonding_curve";
    pub const LEN: usize = 8 + 8 + 8 + 8 + 8 + 1 + 1 + 128;

    // Get signer for the bonding curve PDA
    pub fn get_signer<'a>(mint: &'a Pubkey, bump: &'a u8) -> [&'a [u8]; 3] {
        [
            Self::SEED_PREFIX.as_bytes(),
            mint.as_ref(),
            std::slice::from_ref(bump),
        ]
    }

    // update reserves
    pub fn update_reserves(&mut self, reserve_lamports: u64, reserve_tokens: u64) -> Result<bool> {
        self.virutal_sol_reserve = reserve_lamports;
        self.virtual_token_reserve = reserve_tokens;

        Ok(true)
    }

    // buy

    // sell

    // calculate amount out
}
