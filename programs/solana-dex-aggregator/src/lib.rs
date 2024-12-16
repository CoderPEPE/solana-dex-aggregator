use anchor_lang::prelude::*;

pub mod dex;
pub mod error;
pub mod instructions;
pub mod state;

use dex::*;
use instructions::*;
use state::*;

declare_id!("HJNqViVX59b1hoGXDxMLoaFtjCwvPp9QM2qeFuTnBJcH");

#[program]
pub mod solana_dex_aggregator {
    use super::*;

    pub fn swap(
        ctx: Context<RaydiumSwap>,
        amount_in: u64,
        minimum_amount_out: u64,
        route: DexRoute,
    ) -> Result<()> {
        instructions::swap::handler(ctx, amount_in, minimum_amount_out, route)
    }
}
