use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Token, TokenAccount},
    token_interface::{Mint, TokenAccount as InterfaceTokenAccount, TokenInterface},
};
use raydium_cpmm_cpi::{
    program::RaydiumCpmm,
    states::{AmmConfig, ObservationState, PoolState},
};
use whirlpool_cpi::{self, program::Whirlpool as WhirlpoolProgram, state::*};

#[derive(Accounts)]
pub struct RaydiumSwap<'info> {
    pub cp_swap_program: Program<'info, RaydiumCpmm>,
    /// The user performing the swap
    pub payer: Signer<'info>,

    /// CHECK: pool vault and lp mint authority
    #[account(
      seeds = [
        raydium_cpmm_cpi::AUTH_SEED.as_bytes(),
      ],
      seeds::program = cp_swap_program.key(),
      bump,
  )]
    pub authority: UncheckedAccount<'info>,

    /// The factory state to read protocol fees
    #[account(address = pool_state.load()?.amm_config)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// The user token account for input token
    #[account(mut)]
    pub input_token_account: Box<InterfaceAccount<'info, InterfaceTokenAccount>>,

    /// The user token account for output token
    #[account(mut)]
    pub output_token_account: Box<InterfaceAccount<'info, InterfaceTokenAccount>>,

    /// The vault token account for input token
    #[account(
      mut,
      constraint = input_vault.key() == pool_state.load()?.token_0_vault || input_vault.key() == pool_state.load()?.token_1_vault
  )]
    pub input_vault: Box<InterfaceAccount<'info, InterfaceTokenAccount>>,

    /// The vault token account for output token
    #[account(
      mut,
      constraint = output_vault.key() == pool_state.load()?.token_0_vault || output_vault.key() == pool_state.load()?.token_1_vault
  )]
    pub output_vault: Box<InterfaceAccount<'info, InterfaceTokenAccount>>,

    /// SPL program for input token transfers
    pub input_token_program: Interface<'info, TokenInterface>,

    /// SPL program for output token transfers
    pub output_token_program: Interface<'info, TokenInterface>,

    /// The mint of input token
    #[account(
      address = input_vault.mint
  )]
    pub input_token_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of output token
    #[account(
      address = output_vault.mint
  )]
    pub output_token_mint: Box<InterfaceAccount<'info, Mint>>,
    /// The program account for the most recent oracle observation
    #[account(mut, address = pool_state.load()?.observation_key)]
    pub observation_state: AccountLoader<'info, ObservationState>,
}

#[derive(Accounts)]
pub struct OrcaSwap<'info> {
    pub whirlpool_program: Program<'info, WhirlpoolProgram>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    pub token_authority: Signer<'info>,

    #[account(mut)]
    pub whirlpool: Box<Account<'info, Whirlpool>>,

    #[account(mut, constraint = token_owner_account_a.mint == whirlpool.token_mint_a)]
    pub token_owner_account_a: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = whirlpool.token_vault_a)]
    pub token_vault_a: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = token_owner_account_b.mint == whirlpool.token_mint_b)]
    pub token_owner_account_b: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = whirlpool.token_vault_b)]
    pub token_vault_b: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    /// CHECK: checked by whirlpool_program
    pub tick_array_0: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: checked by whirlpool_program
    pub tick_array_1: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: checked by whirlpool_program
    pub tick_array_2: UncheckedAccount<'info>,

    #[account(mut, seeds = [b"oracle", whirlpool.key().as_ref()], bump, seeds::program = whirlpool_program.key())]
    /// CHECK: checked by whirlpool_program
    pub oracle: UncheckedAccount<'info>,
}
