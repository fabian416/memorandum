use anchor_lang::prelude::*;

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> { 
        let counter = &mut ctx.accounts.counter;
        if counter.is_initialized { 
            return Err(ErrorCode::AlreadyInitialized.into());
        }
        counter.count = 0;
        counter.is_initialized = true;
        Ok(())
    }
}

#[account]
pub struct Counter { 
    pub count: u64,
    pub is_initialized: bool,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 16)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode { 
    #[msg("The counter has already been initialized.")]
    AlreadyInitialized,
}