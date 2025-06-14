//! Error handling for Pino programs.
//!
//! This module provides efficient error types and validation macros that
//! compile to minimal CU overhead while maintaining type safety.

use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

/// Main error type for Pino programs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinoError {
    /// Account data is too small for the expected type
    AccountDataTooSmall,
    /// Account is not mutable when mutation is required
    AccountNotMutable,
    /// Account is not a signer when signature is required
    AccountNotSigner,
    /// Account is not executable when program account is expected
    AccountNotExecutable,
    /// Invalid program ID
    InvalidProgramId,
    /// Account borrow failed (already borrowed)
    AccountBorrowFailed,
    /// Instruction data is invalid or malformed
    InvalidInstructionData,
    /// Insufficient funds for operation
    InsufficientFunds,
    /// Account already initialized
    AccountAlreadyInitialized,
    /// Account not initialized
    AccountNotInitialized,
    /// Invalid account owner
    InvalidAccountOwner,
    /// Arithmetic overflow
    ArithmeticOverflow,
    /// Custom error with code
    Custom(u32),
}

impl From<PinoError> for ProgramError {
    fn from(error: PinoError) -> Self {
        match error {
            PinoError::AccountDataTooSmall => ProgramError::AccountDataTooSmall,
            PinoError::AccountNotMutable => ProgramError::InvalidAccountData,
            PinoError::AccountNotSigner => ProgramError::MissingRequiredSignature,
            PinoError::AccountNotExecutable => ProgramError::InvalidAccountData,
            PinoError::InvalidProgramId => ProgramError::IncorrectProgramId,
            PinoError::AccountBorrowFailed => ProgramError::AccountBorrowFailed,
            PinoError::InvalidInstructionData => ProgramError::InvalidInstructionData,
            PinoError::InsufficientFunds => ProgramError::InsufficientFunds,
            PinoError::AccountAlreadyInitialized => ProgramError::AccountAlreadyInitialized,
            PinoError::AccountNotInitialized => ProgramError::UninitializedAccount,
            PinoError::InvalidAccountOwner => ProgramError::InvalidAccountOwner,
            PinoError::ArithmeticOverflow => ProgramError::ArithmeticOverflow,
            PinoError::Custom(code) => ProgramError::Custom(code),
        }
    }
}

impl From<PinoError> for u64 {
    fn from(error: PinoError) -> Self {
        let program_error: ProgramError = error.into();
        program_error.into()
    }
}

/// Validates a condition and returns an error if false.
///
/// This macro compiles to minimal CU overhead - just a conditional jump.
#[macro_export]
macro_rules! require {
    ($condition:expr, $error:expr) => {
        if !($condition) {
            return Err($error);
        }
    };
}

/// Helper function to validate conditions and return Result
pub fn require_check(condition: bool, error: PinoError) -> Result<(), PinoError> {
    if condition {
        Ok(())
    } else {
        Err(error)
    }
}

/// Validates that two values are equal.
#[macro_export]
macro_rules! require_eq {
    ($left:expr, $right:expr, $error:expr) => {
        if $left != $right {
            return Err($error);
        }
    };
}

/// Validates that two values are not equal.
#[macro_export]
macro_rules! require_neq {
    ($left:expr, $right:expr, $error:expr) => {
        if $left == $right {
            return Err($error);
        }
    };
}

/// Validates that two public keys are equal.
#[macro_export]
macro_rules! require_keys_eq {
    ($left:expr, $right:expr, $error:expr) => {
        if $left != $right {
            return Err($error);
        }
    };
}

/// Validates that two public keys are not equal.
#[macro_export]
macro_rules! require_keys_neq {
    ($left:expr, $right:expr, $error:expr) => {
        if $left == $right {
            return Err($error);
        }
    };
}

/// Validates that an account is owned by the expected program.
#[macro_export]
macro_rules! require_owned_by {
    ($account:expr, $owner:expr, $error:expr) => {
        if !$account.is_owned_by($owner) {
            return Err($error);
        }
    };
}

/// Validates that an account is a signer.
#[macro_export]
macro_rules! require_signer {
    ($account:expr, $error:expr) => {
        if !$account.is_signer() {
            return Err($error);
        }
    };
}

/// Validates that an account is writable.
#[macro_export]
macro_rules! require_writable {
    ($account:expr, $error:expr) => {
        if !$account.is_writable() {
            return Err($error);
        }
    };
}

/// Validates arithmetic operations to prevent overflow.
#[macro_export]
macro_rules! require_safe_add {
    ($left:expr, $right:expr, $error:expr) => {
        $left.checked_add($right).ok_or($error)?
    };
}

/// Validates arithmetic operations to prevent underflow.
#[macro_export]
macro_rules! require_safe_sub {
    ($left:expr, $right:expr, $error:expr) => {
        $left.checked_sub($right).ok_or($error)?
    };
}

/// Validates arithmetic operations to prevent overflow.
#[macro_export]
macro_rules! require_safe_mul {
    ($left:expr, $right:expr, $error:expr) => {
        $left.checked_mul($right).ok_or($error)?
    };
}

/// Validates arithmetic operations to prevent division by zero.
#[macro_export]
macro_rules! require_safe_div {
    ($left:expr, $right:expr, $error:expr) => {
        $left.checked_div($right).ok_or($error)?
    };
}

// Re-export the macros for easier use
pub use {
    require, require_eq, require_neq, require_keys_eq, require_keys_neq,
    require_owned_by, require_signer, require_writable,
    require_safe_add, require_safe_sub, require_safe_mul, require_safe_div,
}; 