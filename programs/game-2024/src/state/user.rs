use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub owner: Pubkey, //32
    pub time: u64,     //8
    pub bump: u8,      //1
}

impl User {
    pub fn init(&mut self, owner: Pubkey, time: u64, bump: u8) -> Result<()> {
        self.owner = owner;
        self.time = time;
        self.bump = bump;

        Ok(())
    }
}
