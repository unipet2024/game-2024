use anchor_lang::prelude::*;

use crate::AuthRole;

#[event]
pub struct SetAuthorityEvent {
    pub admin: Pubkey,
    pub role: AuthRole,
    pub operators: Vec<Pubkey>,
    pub time: i64,
}

#[event]
pub struct UserActiveEvent {
    pub user: Pubkey,
    pub nft: Pubkey,
    pub mint: Pubkey,
    pub currency: Pubkey,
    pub amount: u64,
    pub time: u64,
}

#[event]
pub struct UserDeactiveEvent {
    pub user: Pubkey,
    pub nft: Pubkey,
    pub mint: Pubkey,
    pub time: u64,
}

#[event]
pub struct CloseEvent {
    pub admin: Pubkey,
    pub time: i64,
}


#[event]
pub struct WithdrawEvent {
    pub admin: Pubkey,
    pub currency: Pubkey,
    pub amount: u64,
    pub time: i64,
}