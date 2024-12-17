use anchor_lang::prelude::*;

pub mod dex;
pub mod error;
pub mod instructions;
pub mod state;

use state::*;

declare_id!("HJNqViVX59b1hoGXDxMLoaFtjCwvPp9QM2qeFuTnBJcH");

#[program]
pub mod solana_dex_aggregator {
    use super::*;

    pub fn raydium_swap(
        ctx: Context<RaydiumSwap>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        dex::raydium::raydium_swap(ctx, amount_in, minimum_amount_out)
    }

    pub fn orca_swap(
        ctx: Context<OrcaSwap>,
        amount_in: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool,
    ) -> Result<()> {
        dex::orca::orca_swap(
            ctx,
            amount_in,
            other_amount_threshold,
            sqrt_price_limit,
            amount_specified_is_input,
            a_to_b,
        )
    }
}
