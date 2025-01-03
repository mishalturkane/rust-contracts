use anchor_lang::prelude::*;

declare_id!("YourProgramIDHere");

#[program]
pub mod message_board {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let message_account = &mut ctx.accounts.message_account;
        message_account.message = String::from("Welcome to the message board!");
        Ok(())
    }

    pub fn update_message(ctx: Context<UpdateMessage>, new_message: String) -> Result<()> {
        let message_account = &mut ctx.accounts.message_account;
        message_account.message = new_message;
        Ok(())
    }

    pub fn get_message(ctx: Context<GetMessage>) -> Result<String> {
        let message_account = &ctx.accounts.message_account;
        Ok(message_account.message.clone())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 280)] // 8 for discriminator, 280 for message
    pub message_account: Account<'info, MessageAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMessage<'info> {
    #[account(mut)]
    pub message_account: Account<'info, MessageAccount>,
}

#[derive(Accounts)]
pub struct GetMessage<'info> {
    pub message_account: Account<'info, MessageAccount>,
}

#[account]
pub struct MessageAccount {
    pub message: String,
}
