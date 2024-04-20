use anchor_lang::prelude::*;

use crate::{
    error::GameErrors, AuthRole, AuthorityRole, Game, WithdrawEvent, ADMIN_ROLE, GAME_ACCOUNT,
};

use solana_program::system_instruction;

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(
        mut,
        seeds = [GAME_ACCOUNT],
        bump=game.bump,
        constraint = game.admin == admin_account.key() @ GameErrors::AdminAccountInvalid,
    )]
    pub game: Box<Account<'info, Game>>,

    #[account(
        // mut,
        seeds = [ADMIN_ROLE],
        bump=admin_account.bump,
        constraint = admin_account.is_authority(admin.key) == true @ GameErrors::OnlyAdmin,
        constraint = admin_account.role == AuthRole::Admin @ GameErrors::OnlyAdmin,
        constraint = admin_account.status == true @ GameErrors::OnlyAdmin,
    )]
    pub admin_account: Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_sol_handle(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let admin = &mut ctx.accounts.admin;

    let (index, check) = game.get_fee(Pubkey::default());
    require_neq!(check, false, GameErrors::CurrencyNotSupport);
    require_gt!(
        game.fees[index].total_collect - game.fees[index].total_withdraw,
        amount,
        GameErrors::AmountInsufficient
    );

    msg!("Tranfer {:} to admin", amount);

    let seeds: &[&[u8]] = &[GAME_ACCOUNT, &[game.bump]];
    let signer = &[&seeds[..]];

    msg!("Transfer SOL from user to game contract");
    let transfer_instruction = system_instruction::transfer(
        game.to_account_info().key,
        admin.to_account_info().key,
        amount,
    );

    // Invoke the transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            admin.to_account_info(),
            game.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        signer,
    )?;

    //update total withdraw
    game.fees[index].total_withdraw += amount;

    emit!(WithdrawEvent {
        admin: ctx.accounts.admin.key(),
        currency: Pubkey::default(),
        amount,
        time: Clock::get()?.unix_timestamp
    });

    Ok(())
}
