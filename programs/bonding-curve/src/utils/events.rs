use anchor_lang::prelude::*;

#[event]
pub struct TokenPurchased {
    pub token_mint: Pubkey,
    pub buyer: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub fee_amount: u64,
    pub price: u64,
}

#[event]
pub struct CurveCompleted {
    pub token_mint: Pubkey,
    pub final_sol_reserve: u64,
    pub final_token_reserve: u64,
}