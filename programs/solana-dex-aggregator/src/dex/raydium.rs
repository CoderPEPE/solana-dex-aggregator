use crate::state::*;
use anchor_lang::prelude::*;
use raydium_cpmm_cpi::cpi;

pub fn swap_raydium(
    ctx: Context<RaydiumSwap>,
    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<u64> {
    // Build CPI accounts struct
    let cpi_accounts = cpi::accounts::Swap {
        payer: ctx.accounts.payer.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        amm_config: ctx.accounts.amm_config.to_account_info(),
        pool_state: ctx.accounts.pool_state.to_account_info(),
        input_token_account: ctx.accounts.input_token_account.to_account_info(),
        output_token_account: ctx.accounts.output_token_account.to_account_info(),
        input_vault: ctx.accounts.input_vault.to_account_info(),
        output_vault: ctx.accounts.output_vault.to_account_info(),
        input_token_program: ctx.accounts.input_token_program.to_account_info(),
        output_token_program: ctx.accounts.output_token_program.to_account_info(),
        input_token_mint: ctx.accounts.input_token_mint.to_account_info(),
        output_token_mint: ctx.accounts.output_token_mint.to_account_info(),
        observation_state: ctx.accounts.observation_state.to_account_info(),
    };

    // Create CPI context
    let cpi_ctx = CpiContext::new(ctx.accounts.cp_swap_program.to_account_info(), cpi_accounts);

    // Execute swap via CPI
    cpi::swap_base_input(cpi_ctx, amount_in, minimum_amount_out)?;

    // Return expected output amount
    Ok(minimum_amount_out)
}
