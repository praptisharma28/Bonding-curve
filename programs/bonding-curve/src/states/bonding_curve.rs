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
        let (amount_out, fee) = self.calculate_amount_out(amount_in, 0, fee_percentage)?;
        require!(amount_out >= min_amount_out, ErrorCode::SlippageExceeded);

        // Transfer fee to the fee wallet
        sol_transfer_from_user(
            &user,
            fee_recipient,
            system_program,
            fee,
        )?;

        // Transfer adjust amount to the bonding curve
        sol_transfer_from_user(
            &user,
            curve_pda,
            system_program,
            (amount_in - fee),
        )?;

        // Transfer token from the PDA to the user
        token_transfer_from_signers(
            curve_ata,
            curve_pda,
            user_ata,
            token_program,
            &[&BondingCurve::get_signer(&token_mint.key(), &curve_bump)],
            amount_out,
        );

        // calculate new reserves
        let new_token_reserve = self.virtual_token_reserve
            .checked_sub(amount_out)
            .ok_or(ErrorCode::CalculationError)?;

        let new_sol_reserves = self.virtual_sol_reserve
            .checked_add(amount_in - fee)
            .ok_or(ErrorCode::CalculationError)?;

        // update reserves on the curve
        self.update_reserves(new_sol_reserves, new_token_reserve)?;

        emit!(TokenPurchased{
            token_mint: token_mint.key(),
            buyer: user.key(),
            sol_amount: amount_in,
            token_amount: amount_out,
            fee_amount: fee,
            price: new_sol_reserves / new_token_reserve,
        });

        // Return true if the curve reaches its limit
        if new_sol_reserves >= curve_limit {
            self.is_compeleted = true;
            emit!(CurveCompleted{
                token_mint: token_mint.key(),
                final_sol_reserve: new_sol_reserves,
                final_token_reserve: new_token_reserve,
            });
            return Ok(true);
        }

        // Return false if the curve not reach its limit
        Ok(false)
    }

    // sell

    // calculate amount out
    pub fn calculate_amount_out(&mut self, amount_in: u64, direction:u8, fee_percentage: f64) -> Result<u64> {
        // calculate fee
        let fee = (amount_in as f64 * fee_percentage / 100.0) as f64;
        let amount_in_after_fee = amount_in
        .checked_sub(fee)
        .ok_or(ErrorCode::CalculationError)? as f64;

        let virtual_sol = self.virtual_sol_reserve as f64;
        let virtual_token = self.virtual_token_reserve as f64;
        let amount_in_after_fee = amount_in_after_fee as f64;

        const CRR: f64 = 0.5;

        let amount_out = if direction == 0{
            require!(virtual_sol > 0.0, ErrorCode::CalculationError);
            let base = 1.0 + amount_in_after_fee / virtual_sol;
            virtual_token * (base.powf(CRR) - 1.0)
        } else
        {
            require!(virtual_token > 0.0, ErrorCode::CalculationError);
            let base = 1.0 - amount_in_after_fee / virtual_token;
            virtual_sol * (1.0 - base.powf(1.0 / CRR))
        };
    }
}
