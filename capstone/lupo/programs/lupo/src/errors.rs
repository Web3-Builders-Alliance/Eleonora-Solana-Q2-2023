use anchor_lang::error_code;

#[error_code]
pub enum PredictionError {
    #[msg("Overflow")]
    Overflow,

    #[msg("Result is invalid")]
    InvalidResult,

    #[msg("Title is invalid")]
    InvalidTitle,

    #[msg("Result is invalid or/and control is invalid")]
    InvalidResultOrControl,

    #[msg("Prediction player not equal")]
    PredictionPlayerNotEqual,

    #[msg("You are not the creator")]
    NotCreator,

    #[msg("Transfer Failed")]
    TransferFailed
}
