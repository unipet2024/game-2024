use anchor_lang::prelude::*;

use crate::{ error::GameErrors, AuthRole, AuthorityRole, CloseEvent, Game, ADMIN_ROLE, GAME_ACCOUNT, OPERATOR_ROLE};


#[derive(Accounts)]
#[instruction(id: u8)]
pub struct Close<'info> {
    #[account(
        mut,
        close = admin,
        seeds = [GAME_ACCOUNT],
        bump=game.bump,
        constraint = game.admin == admin_account.key() @ GameErrors::AdminAccountInvalid,
        constraint = game.operator == operator_account.key() @ GameErrors::OperatorAccountInvalid,
    )]
    pub game: Box<Account<'info, Game>>,

    #[account(
        mut,
        close = admin,
        seeds = [ADMIN_ROLE], 
        bump=admin_account.bump,
        constraint = admin_account.is_authority(admin.key) == true @ GameErrors::OnlyAdmin,
        constraint = admin_account.role == AuthRole::Admin @ GameErrors::OnlyAdmin,
        constraint = admin_account.status == true @ GameErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    #[account(
        mut,
        close = admin,
        seeds = [OPERATOR_ROLE], 
        bump=operator_account.bump,
        constraint = operator_account.role == AuthRole::Operator @ GameErrors::OnlyOperator,
        constraint = operator_account.status == true @ GameErrors::OnlyOperator,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn close_handler(
    ctx: Context<Close>,
) -> Result<()> {
    emit!(CloseEvent {
        admin: ctx.accounts.admin.key(),
        time: Clock::get()?.unix_timestamp
    });

    Ok(())
}
