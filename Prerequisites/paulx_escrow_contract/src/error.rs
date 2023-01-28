use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    // Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,

    // NotRentExempt
    #[error("Invalid Instruction")]
    NotRentExempt,

    // ExpectedAmountMismatch
    #[error("Invalid Instruction")]
    ExpectedAmountMismatch,

    // AmountOverflow
    #[error("Invalid Instruction")]
    AmountOverflow,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}