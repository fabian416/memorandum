use anchor_lang::prelude::*;

declare_id!("AKyYdWGzNdMZ5cwYBbxbS7mJbQApZRji6wgoidGLqRGK");

#[program]
pub mod memorandum {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, text: String) -> Result<()> {
        let memorandum = &mut ctx.accounts.memorandum;
        memorandum.text = text;
        memorandum.owner = ctx.accounts.signer.key();
        Ok(())
    }
}

#[derive(Accounts)] // primitives and instruction 
#[instruction(text: String)] // params or args
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + calc_size(&text)
    )]
    pub memorandum: Account<'info, Memorandum >,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

fn calc_size(str: String) {
    let len = str.len();
    let mut size = 0;
    for c in str.chars() {
        size += c.len_utf8();
    }
    size
}

#[account] // Serialization, Deserialization
struct Memorandum {
    pub text: <String> // 200 characters
    pub owner: Pubkey // commonly pubkeys are 32 bytes
}