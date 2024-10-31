use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("2JcXXExQa4jYjbfTbZH4LS3QRWvGXtnmsoNHV4hTdtov");

#[program]
pub mod token_transfer_program {
    use super::*;

    pub fn transfer_tokens(ctx: Context<TransferAccounts>) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
    }
}

#[derive(Accounts)]
pub struct TransferAccounts<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>
}
