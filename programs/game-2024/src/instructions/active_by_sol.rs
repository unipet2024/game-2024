use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use anchor_spl::token::Transfer;
use solana_program::system_instruction;

use crate::{error::GameErrors, Game, User, UserActiveEvent, GAME_ACCOUNT, USER_ACCOUNT};

#[derive(Accounts)]
pub struct ActiveBySol<'info> {
    #[account(
        mut,
        seeds = [GAME_ACCOUNT],
        bump=game.bump,
    )]
    pub game: Box<Account<'info, Game>>,

    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint = mint,
        associated_token::authority = game,
    )]
    pub nft_game: Account<'info, TokenAccount>,

    #[account(
        init,
        space = 8 + 41,
        payer=user,
        seeds = [USER_ACCOUNT, nft_user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, User>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub nft_user: Account<'info, TokenAccount>,

    #[account(mut, signer)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn active_by_sol_handle(ctx: Context<ActiveBySol>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let user_account = &mut ctx.accounts.user_account;
    // let mint = &ctx.accounts.mint;
    let user = &mut ctx.accounts.user;

    //check fee support
    let (index, check) = game.get_fee(Pubkey::default());
    require_neq!(check, false, GameErrors::CurrencyNotSupport);

    //transfer NFT  from user to game contract
    msg!("Transfer NFT from user to game contract");
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.nft_user.to_account_info(),
                to: ctx.accounts.nft_game.to_account_info(),
                authority: user.to_account_info(),
            },
        ),
        1,
    )?;

    //chage fee
    msg!("Transfer SOL from user to game contract");
    let transfer_instruction = system_instruction::transfer(
        user.key,
        game.to_account_info().key,
        game.fees[index].amount,
    );

    // Invoke the transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            user.to_account_info(),
            game.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    //update total collect
    game.fees[index].total_collect += game.fees[index].amount;


    let clock = Clock::get().unwrap();
    //update user account
    user_account.init(user.key(), clock.unix_timestamp, ctx.bumps.user_account)?;


 
    emit!(UserActiveEvent {
        user: user.key(),
        mint: ctx.accounts.mint.key(),
        currency: Pubkey::default(),
        amount: game.fees[index].amount,
        time: clock.unix_timestamp,
        slot: clock.slot,
    });

    Ok(())
}
