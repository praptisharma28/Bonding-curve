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
    pub fn buy(
        &mut self,
        token_mint: &Account<'info, Mint>,  // Token mint address
        curve_limit: u64,                   // Bonding Curve Limit
        user: &Signer<'info>,               // User address for buyer
        curve_pda: &mut AccountInfo<'info>, // Bonding Curve PDA
        fee_recipient: &mut AccountInfo<'info>, // Team wallet address to get fees
        user_ata: &mut AccountInfo<'info>,  // Associated token account for user
        curve_ata: &AccountInfo<'info>,     // Associated token account for bonding curve
        amount_in: u64,                     // Amount of SOL to pay
        min_amount_out: u64,                // Minimum amount of tokens to receive
        fee_percentage: f64,                // Fee percentage for buying on the bonding curve
        curve_bump: u8,                     // Bump for the bonding curve PDA
        system_program: &AccountInfo<'info>, // System program
        token_program: &AccountInfo<'info>,
    ) -> Result<Bool> {
        // check if the amount out is greater than the min amount out
        self.amout

        // Transfer fee to the fee wallet

        // Transfer adjust amount to the bonding curve

        // Transfer token from the PDA to the user

        // calculate new reserves

        // update reserves on the curve

        // Return true if the curve reaches its limit

        // Return false if the curve not reach its limit
    }

    // sell

    // calculate amount out
}
