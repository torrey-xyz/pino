//! Program-level utilities and helpers.
//!
//! This module provides utilities for program development including
//! program ID management, PDA derivation, and common program patterns.

use pinocchio::{
    pubkey::{self, Pubkey},
};
use crate::error::PinoError;

/// Trait for program types that have an associated program ID.
pub trait Program {
    /// Returns the program ID.
    fn id() -> Pubkey;
}

/// Derives a Program Derived Address (PDA) from seeds.
///
/// This is a zero-copy wrapper around Pinocchio's PDA derivation.
pub fn find_program_address(seeds: &[&[u8]], program_id: &Pubkey) -> (Pubkey, u8) {
    pubkey::find_program_address(seeds, program_id)
}

/// Creates a Program Derived Address (PDA) from seeds and bump.
///
/// Returns an error if the derived address is not a valid PDA.
pub fn create_program_address(seeds: &[&[u8]], program_id: &Pubkey) -> Result<Pubkey, PinoError> {
    pubkey::create_program_address(seeds, program_id)
        .map_err(|_| PinoError::InvalidProgramId)
}

/// Validates that a given address is a valid PDA for the given seeds.
pub fn validate_pda(
    address: &Pubkey,
    seeds: &[&[u8]],
    program_id: &Pubkey,
) -> Result<u8, PinoError> {
    let (expected_address, bump) = find_program_address(seeds, program_id);
    
    if address != &expected_address {
        return Err(PinoError::InvalidProgramId);
    }
    
    Ok(bump)
}

/// Common program patterns and utilities.
pub mod patterns {
    use super::*;
    use crate::{
        account::{Account, Signer},
        context::Context,
        error::{require, require_keys_eq, PinoError},
    };
    use pinocchio::account_info::AccountInfo;

    /// Validates that an account is owned by the current program.
    pub fn validate_program_ownership<'info>(
        account: &Account<'info, impl bytemuck::Pod>,
        program_id: &Pubkey,
    ) -> Result<(), PinoError> {
        require_keys_eq!(
            account.owner(),
            program_id,
            PinoError::InvalidAccountOwner
        );
        Ok(())
    }

    /// Validates that a signer matches the expected pubkey.
    pub fn validate_signer_key<'info>(
        signer: &Signer<'info>,
        expected_key: &Pubkey,
    ) -> Result<(), PinoError> {
        require_keys_eq!(
            signer.key(),
            expected_key,
            PinoError::AccountNotSigner
        );
        Ok(())
    }

    /// Validates that an account has sufficient lamports.
    pub fn validate_sufficient_lamports<'info>(
        account: &AccountInfo,
        required_lamports: u64,
    ) -> Result<(), PinoError> {
        require!(
            account.lamports() >= required_lamports,
            PinoError::InsufficientFunds
        );
        Ok(())
    }

    /// Transfers lamports between accounts.
    pub fn transfer_lamports<'info>(
        from: &AccountInfo,
        to: &AccountInfo,
        amount: u64,
    ) -> Result<(), PinoError> {
        require!(from.is_writable(), PinoError::AccountNotMutable);
        require!(to.is_writable(), PinoError::AccountNotMutable);
        
        validate_sufficient_lamports(from, amount)?;

        // Perform the transfer
        *from.try_borrow_mut_lamports()
            .map_err(|_| PinoError::AccountBorrowFailed)? -= amount;
        *to.try_borrow_mut_lamports()
            .map_err(|_| PinoError::AccountBorrowFailed)? += amount;

        Ok(())
    }
}

/// Macro to declare a program ID.
#[macro_export]
macro_rules! declare_id {
    ($id:expr) => {
        /// The program ID
        pub static ID: pinocchio::pubkey::Pubkey = pinocchio::pubkey!($id);

        /// Returns the program ID
        pub fn id() -> pinocchio::pubkey::Pubkey {
            ID
        }
    };
}

/// Macro to generate a program module with automatic ID management.
#[macro_export]
macro_rules! pino_program {
    (
        $(#[$attr:meta])*
        pub mod $name:ident {
            $($item:item)*
        }
    ) => {
        $(#[$attr])*
        pub mod $name {
            use $crate::prelude::*;
            
            $($item)*
        }
    };
}

// Re-export macros
pub use {declare_id, pino_program}; 