use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid Price")]
    InvalidPrice,

    #[msg("Invalid Health Factor")]
    BelowMinHealthFactor,

}