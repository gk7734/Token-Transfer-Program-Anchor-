use anchor_lang::prelude::*;

declare_id!("2JcXXExQa4jYjbfTbZH4LS3QRWvGXtnmsoNHV4hTdtov");

#[program]
pub mod token_transfer_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
