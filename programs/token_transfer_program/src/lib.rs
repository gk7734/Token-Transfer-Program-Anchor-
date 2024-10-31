use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};

declare_id!("2JcXXExQa4jYjbfTbZH4LS3QRWvGXtnmsoNHV4hTdtov");

#[program]
pub mod token_transfer_program {
    use super::*;

    pub fn transfer_tokens(ctx: Context<TransferAccounts>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferAccounts<'info> {
    #[account(mut)]
    /// CHECK: 이 계정은 전송 출발지 계정입니다. 별도의 검증이 필요하지 않습니다.
    pub from: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: 이 계정은 전송 목적지 계정입니다. 별도의 검증이 필요하지 않습니다.
    pub to: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>
}
