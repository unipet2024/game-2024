use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, Fee, Game, ADMIN_ROLE, GAME_ACCOUNT, OPERATOR_ROLE};


#[derive(Accounts)]
#[instruction(duration_active: u64, fees: Vec<Fee>)]
pub struct InitGame<'info> {
    #[account(
        init,  
        payer = authority, 
        space = 8 + 77 + fees.len() * 56,
        // space = 1000,
        seeds = [GAME_ACCOUNT],
        bump
    )]
    pub game: Box<Account<'info, Game>>,
    #[account(
        init,
        space = 8 + 40, // 1 admin
        payer = authority,
        seeds = [ADMIN_ROLE], 
        bump,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,
    #[account(
        init,
        space = 8+170, // max 5 operator
        payer = authority,
        seeds = [OPERATOR_ROLE], 
        bump,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn init_handle(ctx: Context<InitGame>,duration_active: u64, fees: Vec<Fee>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let admin_account = &mut ctx.accounts.admin_account;
    let operator_account = &mut ctx.accounts.operator_account;


    game.init(
        admin_account.key(),
        operator_account.key(),
        duration_active,
        fees,
        ctx.bumps.game,
    )?;

    //SET ADMIN
   //SET ADMIN
   let authorities = vec![ctx.accounts.authority.key()];
   admin_account.initialize(
       &authorities,
       ctx.bumps.admin_account,
       AuthRole::Admin,
   )?;
   operator_account.initialize(
       &authorities,
       ctx.bumps.operator_account,
       AuthRole::Operator,
   )?;

    Ok(())
}
