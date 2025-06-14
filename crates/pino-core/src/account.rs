//! Zero-copy account system built on Pinocchio.
//!
//! This module provides type-safe account wrappers that maintain Pinocchio's
//! zero-copy efficiency while offering convenient APIs for account validation
//! and data access.

use core::marker::PhantomData;
use pinocchio::{
    account_info::{AccountInfo, Ref, RefMut},
    pubkey::Pubkey,
};
use bytemuck::Pod;
use crate::error::{PinoError, require_check};

/// A zero-copy account wrapper that provides type-safe access to account data.
///
/// This is the primary account type in Pino, providing safe access to account
/// data while maintaining Pinocchio's zero-copy efficiency.
pub struct Account<'info, T> {
    info: &'info AccountInfo,
    _phantom: PhantomData<T>,
}

impl<'info, T> Account<'info, T> {
    /// Creates a new Account wrapper around an AccountInfo.
    ///
    /// # Safety
    /// 
    /// The caller must ensure that the account data is properly initialized
    /// and matches the expected type T.
    pub unsafe fn new_unchecked(info: &'info AccountInfo) -> Self {
        Self {
            info,
            _phantom: PhantomData,
        }
    }

    /// Creates a new Account wrapper with validation.
    pub fn new(info: &'info AccountInfo) -> Result<Self, PinoError> {
        // Basic validation
        require_check(
            info.data_len() >= core::mem::size_of::<T>(),
            PinoError::AccountDataTooSmall
        )?;

        Ok(Self {
            info,
            _phantom: PhantomData,
        })
    }

    /// Returns the underlying AccountInfo.
    pub fn info(&self) -> &'info AccountInfo {
        self.info
    }

    /// Returns the account's public key.
    pub fn key(&self) -> &Pubkey {
        self.info.key()
    }

    /// Returns the account's owner.
    pub fn owner(&self) -> &Pubkey {
        unsafe { self.info.owner() }
    }

    /// Returns the account's lamport balance.
    pub fn lamports(&self) -> u64 {
        self.info.lamports()
    }

    /// Checks if the account is writable.
    pub fn is_writable(&self) -> bool {
        self.info.is_writable()
    }

    /// Checks if the account is a signer.
    pub fn is_signer(&self) -> bool {
        self.info.is_signer()
    }

    /// Checks if the account is executable.
    pub fn executable(&self) -> bool {
        self.info.executable()
    }
}

impl<'info, T: Pod> Account<'info, T> {
    /// Loads the account data as a reference to T.
    ///
    /// This provides zero-copy access to the account data.
    pub fn load(&self) -> Result<&T, PinoError> {
        let data = unsafe { self.info.borrow_data_unchecked() };
        
        require_check(
            data.len() >= core::mem::size_of::<T>(),
            PinoError::AccountDataTooSmall
        )?;

        Ok(bytemuck::from_bytes(&data[..core::mem::size_of::<T>()]))
    }

    /// Loads the account data as a mutable reference to T.
    pub fn load_mut(&self) -> Result<&mut T, PinoError> {
        require_check(self.is_writable(), PinoError::AccountNotMutable)?;

        let data = unsafe { self.info.borrow_mut_data_unchecked() };
        
        require_check(
            data.len() >= core::mem::size_of::<T>(),
            PinoError::AccountDataTooSmall
        )?;

        Ok(bytemuck::from_bytes_mut(&mut data[..core::mem::size_of::<T>()]))
    }

    /// Initializes the account data with the given value.
    pub fn init(&self, value: T) -> Result<(), PinoError> {
        require_check(self.is_writable(), PinoError::AccountNotMutable)?;
        
        let data = unsafe { self.info.borrow_mut_data_unchecked() };
        
        require_check(
            data.len() >= core::mem::size_of::<T>(),
            PinoError::AccountDataTooSmall
        )?;

        let account_data = bytemuck::from_bytes_mut(&mut data[..core::mem::size_of::<T>()]);
        *account_data = value;
        
        Ok(())
    }
}

/// A signer account wrapper.
pub struct Signer<'info> {
    info: &'info AccountInfo,
}

impl<'info> Signer<'info> {
    /// Creates a new Signer wrapper.
    pub fn new(info: &'info AccountInfo) -> Result<Self, PinoError> {
        require_check(info.is_signer(), PinoError::AccountNotSigner)?;
        
        Ok(Self { info })
    }

    /// Returns the underlying AccountInfo.
    pub fn info(&self) -> &'info AccountInfo {
        self.info
    }

    /// Returns the signer's public key.
    pub fn key(&self) -> &Pubkey {
        self.info.key()
    }
}

/// A program account wrapper for CPI calls.
pub struct Program<'info, T> {
    info: &'info AccountInfo,
    _phantom: PhantomData<T>,
}

impl<'info, T> Program<'info, T> {
    /// Creates a new Program wrapper.
    pub fn new(info: &'info AccountInfo, expected_id: &Pubkey) -> Result<Self, PinoError> {
        require_check(info.executable(), PinoError::AccountNotExecutable)?;
        require_check(info.key() == expected_id, PinoError::InvalidProgramId)?;
        
        Ok(Self {
            info,
            _phantom: PhantomData,
        })
    }

    /// Returns the underlying AccountInfo.
    pub fn info(&self) -> &'info AccountInfo {
        self.info
    }

    /// Returns the program's public key.
    pub fn key(&self) -> &Pubkey {
        self.info.key()
    }
}

/// An unchecked account wrapper for maximum flexibility.
///
/// Use this when you need direct access to AccountInfo without validation.
/// Be careful - this bypasses all safety checks.
pub struct UncheckedAccount<'info> {
    info: &'info AccountInfo,
}

impl<'info> UncheckedAccount<'info> {
    /// Creates a new UncheckedAccount wrapper.
    pub fn new(info: &'info AccountInfo) -> Self {
        Self { info }
    }

    /// Returns the underlying AccountInfo.
    pub fn info(&self) -> &'info AccountInfo {
        self.info
    }

    /// Converts to a typed Account with validation.
    pub fn to_account<T>(&self) -> Result<Account<'info, T>, PinoError> {
        Account::new(self.info)
    }

    /// Converts to a Signer with validation.
    pub fn to_signer(&self) -> Result<Signer<'info>, PinoError> {
        Signer::new(self.info)
    }

    /// Converts to a Program with validation.
    pub fn to_program<T>(&self, expected_id: &Pubkey) -> Result<Program<'info, T>, PinoError> {
        Program::new(self.info, expected_id)
    }
}

/// An account loader for zero-copy large data access.
///
/// This is useful for accounts with large data that should be accessed without
/// copying the entire data into memory.
pub struct AccountLoader<'info, T> {
    info: &'info AccountInfo,
    _phantom: PhantomData<T>,
}

impl<'info, T: Pod> AccountLoader<'info, T> {
    /// Creates a new AccountLoader.
    pub fn new(info: &'info AccountInfo) -> Result<Self, PinoError> {
        require_check(
            info.data_len() >= core::mem::size_of::<T>(),
            PinoError::AccountDataTooSmall
        )?;

        Ok(Self {
            info,
            _phantom: PhantomData,
        })
    }

    /// Loads the account data as an immutable reference.
    pub fn load(&self) -> Result<Ref<T>, PinoError> {
        let data = self.info.try_borrow_data()
            .map_err(|_| PinoError::AccountBorrowFailed)?;
        
        let account_ref = Ref::map(data, |data| {
            bytemuck::from_bytes(&data[..core::mem::size_of::<T>()])
        });
        
        Ok(account_ref)
    }

    /// Loads the account data as a mutable reference.
    pub fn load_mut(&self) -> Result<RefMut<T>, PinoError> {
        require_check(self.info.is_writable(), PinoError::AccountNotMutable)?;

        let data = self.info.try_borrow_mut_data()
            .map_err(|_| PinoError::AccountBorrowFailed)?;
        
        let account_ref = RefMut::map(data, |data| {
            bytemuck::from_bytes_mut(&mut data[..core::mem::size_of::<T>()])
        });
        
        Ok(account_ref)
    }

    /// Returns the underlying AccountInfo.
    pub fn info(&self) -> &'info AccountInfo {
        self.info
    }
}

/// System program marker type.
pub struct System;

/// Token program marker type.
pub struct Token;

/// Associated token program marker type.
pub struct AssociatedToken;

/// Common program IDs
pub mod program_ids {
    use super::Pubkey;

    /// System program ID: 11111111111111111111111111111111
    pub const SYSTEM_PROGRAM_ID: Pubkey = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];

    /// SPL Token program ID: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
    pub const TOKEN_PROGRAM_ID: Pubkey = [
        6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53,
        218, 196, 57, 220, 26, 235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
    ];

    /// Associated Token program ID: ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
    pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey = [
        140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131,
        11, 90, 19, 153, 218, 255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
    ];
} 