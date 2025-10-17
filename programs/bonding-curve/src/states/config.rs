use anchor_lang::prelude::*;

//NOTE :  the commented values are just assumption when i am minting one billion tokens

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ConfigSettings {
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub curve_limit: u64,
    pub initial_virtual_token_reserve: u64,
    pub initial_virtual_sol_reserve: u64,
    pub initial_real_token_reserve: u64,
    pub initial_token_supply: u64,
    pub buy_fee_percentage: f64,
    pub sell_fee_percentage: f64,
    pub migration_fee_percentage: f64,
    pub reserved: [u8; 128],
}

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub curve_limit: u64, // maximum SOL the pool can absorb before reaching its cap

    pub initial_virtual_token_reserve: u64, // 200_000_000 M for shaping the curve
    pub initial_virtual_sol_reserve: u64, // It makes the curve smoother and prevents the price from starting at zero or being too volatile initially.
    pub initial_real_token_reserve: u64,  // 800_000_000 M for bonding curve launch for users to buy
    pub initial_token_supply: u64,        // total tokens minted

    pub buy_fee_percentage: f64,
    pub sell_fee_percentage: f64,
    pub migration_fee_percentage: f64,

    pub reserved: [u8; 128], // space for future upgrades : [[u8; 8]; 16]
}

impl Config {
    pub const SEED_PREFIX: &'static str = "global_config";
    pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 128;
}
