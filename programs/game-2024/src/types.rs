use anchor_lang::prelude::*;

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum AuthRole {
    Admin,
    Operator,
}

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
pub struct Fee {
    pub currency: Pubkey,    //32
    pub amount: u64,         //8
    pub total_collect: u64,  //8
    pub total_withdraw: u64, //8
}
