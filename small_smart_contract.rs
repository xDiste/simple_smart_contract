use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod messengerapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.current_message = "".to_string();
        base_account.previous_message = "".to_string();
        base_account.counter_messages = 0;
        Ok(())
    }

    pub fn send_message(ctx: Context<Update>, message: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let copy = base_account.current_message.clone();
        base_account.previous_message = copy;
        base_account.current_message = message;
        base_account.counter_messages += 1;
        Ok(())
    }

    pub fn get_current_message(ctx: Context<Update>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        msg!("Last message that you sent: {}", base_account.current_message);
        Ok(())
    }
    
    pub fn get_previous_message(ctx: Context<Update>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        msg!("Previous message that you sent: {}", base_account.previous_message);
        Ok(())
    }

    pub fn get_counter_messages(ctx: Context<Update>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        msg!("You sent {} messages in our system!", base_account.counter_messages);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub current_message: String,
    pub previous_message: String,
    pub counter_messages: u64,
}
