use anchor_lang::prelude::*;

use crate::{error::GameErrors, AuthRole, AuthorityRole, Fee, Game, GAME_ACCOUNT, OPERATOR_ROLE};


#[derive(Accounts)]
#[instruction(fees: Vec<Fee>)]
pub struct SetFee<'info> {
    #[account(
        mut, 
        realloc = 8 + 77 * fees.len() * 56,
        realloc::payer = operator,
        realloc::zero=true,
        seeds = [GAME_ACCOUNT],
        bump=game.bump,
        constraint = game.operator == operator_account.key() @ GameErrors::OperatorAccountInvalid,
    )]
    pub game: Box<Account<'info, Game>>,
    #[account(
        seeds = [OPERATOR_ROLE], 
        bump=operator_account.bump,
        constraint = operator_account.is_authority(operator.key) == true @ GameErrors::OnlyOperator,
        constraint = operator_account.role == AuthRole::Operator @ GameErrors::OnlyOperator,
        constraint = operator_account.status == true @ GameErrors::OnlyOperator,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn set_fee_handle(ctx: Context<SetFee>,fees: Vec<Fee>) -> Result<()> {
    let game = &mut ctx.accounts.game;

    game.set_fees(&fees)?;

    Ok(())
}
