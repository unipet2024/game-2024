use anchor_lang::prelude::*;

use crate::Fee;

// total 105
#[account]
pub struct Game {
    pub admin: Pubkey,        //32
    pub operator: Pubkey,     //32
    pub fees: Vec<Fee>,       // 4 + 56 * n
    pub duration_active: u64, //8
    pub bump: u8,             //1
}

impl Game {
    pub fn init(
        &mut self,
        admin: Pubkey,
        operator: Pubkey,
        duration_active: u64,
        fees: Vec<Fee>,
        // currency_amount: u64,
        bump: u8,
    ) -> Result<()> {
        self.admin = admin;
        self.operator = operator;

        self.set_duration(duration_active)?;

        self.set_fees(&fees)?;

        self.bump = bump;

        Ok(())
    }

    pub fn set_fees(&mut self, fees: &Vec<Fee>) -> Result<()> {
        self.fees = vec![];

        for fee in fees.iter() {
            self.add_fee(*fee)?;
        }
        Ok(())
    }

    pub fn set_fee(&mut self, fee: Fee) -> Result<()> {
        for f in self.fees.iter_mut() {
            if f.currency == fee.currency {
                f.total_collect = fee.total_collect;
                f.amount = fee.amount;
                f.total_withdraw = fee.total_withdraw;
            }
        }
        Ok(())
    }

    fn add_fee(&mut self, fee: Fee) -> Result<()> {
        self.fees.push(fee);
        Ok(())
    }

    pub fn get_fee(&self, currency: Pubkey) -> (usize, bool) {
        for (i, fee) in self.fees.iter().enumerate() {
            if fee.currency == currency {
                return (i, true);
            }
        }
        (0, false)
    }

    pub fn set_duration(&mut self, duration_active: u64) -> Result<()> {
        self.duration_active = duration_active;
        Ok(())
    }
}
