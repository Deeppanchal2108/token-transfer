use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

declare_id!("EdZMKG2hbcijrkx68LGB5LSVZi6QqAea4MXe7SMYrkmp");
// need  to comment for this contract

// In this contract we just transfering tokens from one account to another
// we are using anchor_spl::token to use the transfer function
#[program]
pub mod token_transfer {
    use super::*;

    pub fn initialize(ctx: Context<TokenTransfer>,amount :u64) -> Result<()> {

        // Some checks to ensure that the sender is the owner of the token account and that the mint matches

        // the owner of the token account should be the sender
        require!(ctx.accounts.senders_token_account.owner== ctx.accounts.sender.key(),CustomError::SignerError);

        // The mint of the sender's token account should match the mint of the receiver's token account (means same token should be transferred and received)
        require!(ctx.accounts.senders_token_account.mint== ctx.accounts.mint.key(),CustomError::InvalidSenderMint);

        // The mint of the receiver's token account should match the mint of the sender's token account (means same token should be transferred and received)
        require!(ctx.accounts.receiver_token_account.mint== ctx.accounts.mint.key(),CustomError::InvalidReceiverMint);

        


        // All the accounts needed for the transfer function are set up 
        let cpi_accounts =token::Transfer{
            from: ctx.accounts.senders_token_account.to_account_info(),
            to:ctx.accounts.receiver_token_account.to_account_info(),
            authority : ctx.accounts.sender.to_account_info()
        };

        //the token program that we have passed in the struct 
        let cpi_program=ctx.accounts.token_program.to_account_info();

        // We create a CPI context to call the transfer function from the token program
        let cpi_context =CpiContext::new(cpi_program,cpi_accounts);


        // Finally we call the transfer function from the token program with the CPI context and the amount of tokens to be transferred
        token::transfer(cpi_context,amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TokenTransfer<'info>{

// This is the context for the transfer function
    // The signer is the account that is sending the tokens
    #[account(mut)]
    pub sender :Signer<'info>,

    // The sender's token account is the account that holds the tokens to be sent
    #[account(mut)]
    pub senders_token_account :Account<'info , TokenAccount>,

    // The receiver's token account is the account that will receive the tokens
    #[account(mut)]
    pub receiver_token_account :Account<'info , TokenAccount>,

    // The mint account is the account that holds the token's mint information (which token is being transferred or allowed to be transferred)
    pub mint : Account<'info, Mint>,

    // The token program is the program that handles the token transfers
    //we need this to call the transfer function from the token program
    pub token_program:Program<'info, Token>


}



#[error_code]
// Custom error codes for the program
//This is the way how u write custom errors in anchor its not struct its enum remeber that
pub enum CustomError{

    #[msg("Signer does not match the senders")]
    SignerError,

    #[msg("Mint account does not match with the sender mint")]
    InvalidSenderMint,

    #[msg("Mint account does not match with the sender mint")]
    InvalidReceiverMint

}