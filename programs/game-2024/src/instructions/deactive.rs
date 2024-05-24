use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use anchor_spl::token::transfer;
use anchor_spl::token::Transfer;

use crate::{error::GameErrors, Game, User, UserDeactiveEvent, GAME_ACCOUNT, USER_ACCOUNT};

#[derive(Accounts)]
pub struct Deactive<'info> {
    #[account(
        seeds = [GAME_ACCOUNT],
        bump=game.bump,
    )]
    pub game: Box<Account<'info, Game>>,

    #[account(
        mut,
        // close = user,
        associated_token::mint = mint,
        associated_token::authority = game,
    )]
    pub nft_game: Account<'info, TokenAccount>,

    #[account(
        mut,
        close = user,
        seeds = [USER_ACCOUNT,  nft_user.key().as_ref()],
        bump=user_account.bump,
        constraint = user_account.owner == user.key() @ GameErrors::OnlyOperator,
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

pub fn deactive_handle(ctx: Context<Deactive>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let user_account = &mut ctx.accounts.user_account;
    // let mint = &ctx.accounts.mint;
    // let user = &mut ctx.accounts.user;

    let currenct = Clock::get()?.unix_timestamp as u64;
    require_gte!(
        currenct,
        game.duration_active + user_account.time as u64,
        GameErrors::StillLock
    );

    //transfer NFT  from user to game contract
    msg!(
        "Transfer NFT from {:} to {:}",
        ctx.accounts.nft_game.key(),
        ctx.accounts.nft_user.key()
    );

    let seeds: &[&[u8]] = &[GAME_ACCOUNT, &[game.bump]];
    let signer = &[&seeds[..]];
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.nft_game.to_account_info(),
                to: ctx.accounts.nft_user.to_account_info(),
                authority: game.to_account_info(),
            },
        )
        .with_signer(signer),
        1,
    )?;

    msg!("Transfer done");

    let clock = Clock::get().unwrap();

    emit!(UserDeactiveEvent {
        user: ctx.accounts.user.key(),
        mint: ctx.accounts.mint.key(),
        time: clock.unix_timestamp,
        slot: clock.slot,
    });

    Ok(())
}
