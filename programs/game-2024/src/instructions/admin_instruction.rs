use anchor_lang::prelude::*;

use crate::{ error::GameErrors, AuthRole, AuthorityRole, Game, SetAuthorityEvent, ADMIN_ROLE, GAME_ACCOUNT, OPERATOR_ROLE};

#[derive(Accounts)]
pub struct AdminInstruction<'info> {
    #[account(
        seeds = [GAME_ACCOUNT],
        bump=game.bump,
        constraint = game.admin == admin_account.key() @ GameErrors::AdminAccountInvalid,
        constraint = game.operator == operator_account.key() @ GameErrors::OperatorAccountInvalid,
    )]
    pub game: Box<Account<'info, Game>>,

    #[account(
        mut,
        seeds = [ADMIN_ROLE], 
        bump=admin_account.bump,
        constraint = admin_account.is_authority(admin.key) == true @ GameErrors::OnlyAdmin,
        constraint = admin_account.role == AuthRole::Admin @ GameErrors::OnlyAdmin,
        constraint = admin_account.status == true @ GameErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    #[account(
        mut,
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

pub fn set_authority_handler(ctx: Context<AdminInstruction>, role: AuthRole, operators: Vec<Pubkey>) -> Result<()> {
    match role {
        AuthRole::Operator => set_operator_handler(ctx, operators),
        AuthRole::Admin => set_admin_handler(ctx, operators),
    }
}

fn set_operator_handler(ctx: Context<AdminInstruction>, operators: Vec<Pubkey>) -> Result<()> {
    let operator_account = &mut ctx.accounts.operator_account;

    for operator in operators.iter(){
        msg!("{:},", *operator)
    }

    operator_account.set_authorities(&operators)?;

    emit!(SetAuthorityEvent{
        admin: ctx.accounts.admin.key(),
        role: AuthRole::Operator,
        operators,
        time: Clock::get()?.unix_timestamp
    });

    Ok(())
}

fn set_admin_handler(ctx: Context<AdminInstruction>, admins: Vec<Pubkey>) -> Result<()> {
    let admin_account = &mut ctx.accounts.admin_account;

    admin_account.set_authorities(&admins)?;

    emit!(SetAuthorityEvent{
        admin: ctx.accounts.admin.key(),
        role: AuthRole::Admin,
        operators: admins,
        time: Clock::get()?.unix_timestamp
    });

    Ok(())
}
