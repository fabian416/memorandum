use anchor_lang::prelude::*;

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

#[derive(Accounts)]
#[instruction(text: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + calc_size(&text)
    )]
    pub memorandum: Account<'info, Memorandum>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

fn calc_size(text: &String) -> usize {
    let len = text.len();
    len // Puedes simplificar usando solo `len`, ya que `String` calcula el tamaño en bytes.
}

#[account]
pub struct Memorandum {
    pub text: String,
    pub owner: Pubkey,
}