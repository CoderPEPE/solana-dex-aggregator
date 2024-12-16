use crate::{dex::DexRoute, error::ErrorCode, state::*};
use anchor_lang::prelude::*;

pub fn handler<'info>(
    ctx: Context<RaydiumSwap<'info>>,
    amount_in: u64,
    minimum_amount_out: u64,
    route: DexRoute,
) -> Result<()> {
    require!(amount_in > 0, ErrorCode::InvalidAmount);

    let mut current_amount = amount_in;

    // Execute swaps through the route
    current_amount = route.swap(ctx, current_amount, minimum_amount_out)?;

    // Verify final amount meets minimum requirement
    require!(
        current_amount >= minimum_amount_out,
        ErrorCode::SlippageExceeded
    );

    Ok(())
}
