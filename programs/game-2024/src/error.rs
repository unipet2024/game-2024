use anchor_lang::prelude::*;

#[error_code]
pub enum GameErrors {
    // #[msg("Package invalid")]
    // PackageInvalid,

    #[msg("Amount insufficient")]
    AmountInsufficient,

    // #[msg("Game closed")]
    // GameClosed,
    #[msg("Admin account invalid")]
    AdminAccountInvalid,

    #[msg("Operator account invalid")]
    OperatorAccountInvalid,

    #[msg("Only admin")]
    OnlyAdmin,

    #[msg("Only Operator")]
    OnlyOperator,

    #[msg("Still lock")]
    StillLock,

    #[msg("Operator not change")]
    OperatorNotChange,

    #[msg("Currency not support")]
    CurrencyNotSupport,
}

impl From<GameErrors> for ProgramError {
    fn from(e: GameErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}
