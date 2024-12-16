use anchor_lang::prelude::*;

declare_id!("HJNqViVX59b1hoGXDxMLoaFtjCwvPp9QM2qeFuTnBJcH");

#[program]
pub mod solana_dex_aggregator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
