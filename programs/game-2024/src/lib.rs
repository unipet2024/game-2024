pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod types;

use anchor_lang::prelude::*;

pub use constants::*;
pub use events::*;
pub use instructions::*;
pub use state::*;
pub use types::*;

declare_id!("GRxJzSgBKJkgYRtjCzciCf1Pf68iGkygcH5s2RPvxj6Z");

#[program]
pub mod game_2024 {
    use super::*;

    pub fn init(ctx: Context<InitGame>, duration_active: u64, fees: Vec<Fee>) -> Result<()> {
        init::init_handle(ctx, duration_active, fees)
    }

    pub fn set_authority(
        ctx: Context<AdminInstruction>,
        role: AuthRole,
        operators: Vec<Pubkey>,
    ) -> Result<()> {
        admin_instruction::set_authority_handler(ctx, role, operators)
    }

    pub fn set_fee(ctx: Context<SetFee>, fees: Vec<Fee>) -> Result<()> {
        set_fee::set_fee_handle(ctx, fees)
    }

    pub fn set_duration_active(ctx: Context<SetDuration>, duration_active: u64) -> Result<()> {
        set_duration::set_duration_handle(ctx, duration_active)
    }

    pub fn active_by_sol(ctx: Context<ActiveBySol>) -> Result<()> {
        active_by_sol::active_by_sol_handle(ctx)
    }

    pub fn active_by_spl(ctx: Context<ActiveBySpl>) -> Result<()> {
        active_by_spl::active_by_spl_handle(ctx)
    }

    pub fn withdraw_spl(ctx: Context<WithdrawSpl>, amount: u64) -> Result<()> {
        withdraw_spl::withdraw_spl_handle(ctx, amount)
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
        withdraw_sol::withdraw_sol_handle(ctx, amount)
    }

    pub fn deactive(ctx: Context<Deactive>) -> Result<()> {
        deactive::deactive_handle(ctx)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        close::close_handler(ctx)
    }
}
