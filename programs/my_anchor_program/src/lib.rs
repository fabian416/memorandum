use anchor_lang::prelude::*;

declare_id!("AKyYdWGzNdMZ5cwYBbxbS7mJbQApZRji6wgoidGLqRGK");

#[program]
pub mod my_anchor_program {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        if counter.is_initialized {
            return Err(ErrorCode::AlreadyInitialized.into());
        }
        counter.count = 0;
        counter.is_initialized = true;
        Ok(())
    }

    pub fn increment_counter(ctx: Context<Increment>, value: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        if value == 0 {
            return Err(ErrorCode::InvalidIncrementValue.into());
        }
        counter.count = counter
            .count
            .checked_add(value)
            .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }

    pub fn initialize_memorandum(ctx: Context<InitializeMemorandum>, text: String) -> Result<()> {
        let memorandum = &mut ctx.accounts.memorandum;
        memorandum.text = text;
        memorandum.owner = ctx.accounts.signer.key();
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub count: u64,
    pub is_initialized: bool,
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(init, payer = user, space = 8 + 16)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
#[instruction(text: String)]
pub struct InitializeMemorandum<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + text.len() // Tamaño calculado para el texto
    )]
    pub memorandum: Account<'info, Memorandum>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Memorandum {
    pub text: String,
    pub owner: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The counter has already been initialized.")]
    AlreadyInitialized,
    #[msg("Invalid increment value")]
    InvalidIncrementValue,
    #[msg("Overflow error while incrementing the value")]
    Overflow,
}
