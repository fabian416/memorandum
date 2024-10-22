use anchor_lang::prelude::*;

declare_id!("AKyYdWGzNdMZ5cwYBbxbS7mJbQApZRji6wgoidGLqRGK");

#[program]
pub mod memorandum {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
