use crate::state::*;
use anchor_lang::prelude::*;

pub mod orca;
pub mod raydium;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum DexRoute {
    Raydium,
    Orca,
}

impl DexRoute {
    pub fn swap(
        &self,
        ctx: Context<RaydiumSwap>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<u64> {
        match self {
            DexRoute::Raydium => raydium::swap_raydium(ctx, amount_in, minimum_amount_out),
            DexRoute::Orca => {
                // Implement Orca swap logic
                Ok(minimum_amount_out)
            }
        }
    }
}
