use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use anchor_spl::token::{transfer, Transfer};

use crate::{
    error::GameErrors, AuthRole, AuthorityRole, Game, WithdrawEvent, ADMIN_ROLE, GAME_ACCOUNT,
};

#[derive(Accounts)]
pub struct WithdrawSpl<'info> {
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

    #[account(
        mut,
        associated_token::mint = currency_mint,
        associated_token::authority = game
    )]
    pub currency_game: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = currency_mint,
        associated_token::authority = admin
    )]
    pub currency_admin: Box<Account<'info, TokenAccount>>,

    #[account(mut, signer)]
    pub admin: Signer<'info>,

    pub currency_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_spl_handle(ctx: Context<WithdrawSpl>, amount: u64) -> Result<()> {
    let game = &mut ctx.accounts.game;
    // let admin = &mut ctx.accounts.admin;

    let (index, check) = game.get_fee(Pubkey::default());
    require_neq!(check, false, GameErrors::CurrencyNotSupport);
    require_gt!(
        game.fees[index].total_collect - game.fees[index].total_withdraw,
        amount,
        GameErrors::AmountInsufficient
    );

    msg!(
        "Tranfer {:} {:} to admin",
        amount,
        ctx.accounts.currency_mint.key()
    );

    let seeds: &[&[u8]] = &[GAME_ACCOUNT, &[game.bump]];
    let signer = &[&seeds[..]];

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                authority: game.to_account_info(),
                from: ctx.accounts.currency_game.to_account_info(),
                to: ctx.accounts.currency_admin.to_account_info(),
            },
        )
        .with_signer(signer),
        amount,
    )?;

    //update total withdraw
    game.fees[index].total_withdraw += amount;

    emit!(WithdrawEvent {
        admin: ctx.accounts.admin.key(),
        currency: ctx.accounts.currency_mint.key(),
        amount,
        time: Clock::get()?.unix_timestamp
    });

    Ok(())
}
