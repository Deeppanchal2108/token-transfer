use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

declare_id!("CFHC4sic3QX8GWrqWg7wBcaoryNvzDzqANGHsjr5PAiU");
// need  to comment for this contract
#[program]
pub mod token_transfer {
    use super::*;

    pub fn initialize(ctx: Context<TokenTransfer>,amount :u64) -> Result<()> {
        require!(ctx.accounts.senders_token_account.owner== ctx.accounts.sender.key(),CustomError::SignerError);
        require!(ctx.accounts.senders_token_account.mint== ctx.accounts.mint.key(),CustomError::InvalidSenderMint);

        require!(ctx.accounts.receiver_token_account.mint== ctx.accounts.mint.key(),CustomError::InvalidReceiverMint);

        

        let cpi_accounts =token::Transfer{
            from: ctx.accounts.senders_token_account.to_account_info(),
            to:ctx.accounts.receiver_token_account.to_account_info(),
            authority : ctx.accounts.sender.to_account_info()
        };

        let cpi_program=ctx.accounts.token_program.to_account_info();

        let cpi_context =CpiContext::new(cpi_program,cpi_accounts);
        token::transfer(cpi_context,amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TokenTransfer<'info>{

    #[account(mut)]
    pub sender :Signer<'info>,

    #[account(mut)]
    pub senders_token_account :Account<'info , TokenAccount>,


    #[account(mut)]
    pub receiver_token_account :Account<'info , TokenAccount>,

    pub mint : Account<'info, Mint>,

    pub token_program:Program<'info, Token>


}



#[error_code]
pub enum CustomError{

    #[msg("Signer does not match the senders")]
    SignerError,

    #[msg("Mint account does not match with the sender mint")]
    InvalidSenderMint,

    #[msg("Mint account does not match with the sender min")]
    InvalidReceiverMint

}