//! Context system for structured account access.
//!
//! This module provides the Context type that organizes accounts and instruction
//! data in a type-safe manner while maintaining Pinocchio's zero-copy efficiency.

extern crate alloc;

use core::marker::PhantomData;
use alloc::vec::Vec;
use pinocchio::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    ProgramResult,
};
use crate::error::PinoError;

/// Context provides structured access to accounts and instruction data.
///
/// This is the primary way to access accounts in Pino instruction handlers.
/// It provides type-safe access to validated accounts while maintaining
/// zero-copy efficiency.
pub struct Context<'info, T> {
    /// The program ID that owns this instruction
    pub program_id: &'info Pubkey,
    /// The structured accounts
    pub accounts: T,
    /// The remaining accounts not captured in the accounts struct
    pub remaining_accounts: &'info [AccountInfo],
    /// The instruction data
    pub instruction_data: &'info [u8],
    /// Bump seeds for PDAs (if any)
    pub bumps: BumpSeeds,
}

impl<'info, T> Context<'info, T> {
    /// Creates a new Context.
    pub fn new(
        program_id: &'info Pubkey,
        accounts: T,
        remaining_accounts: &'info [AccountInfo],
        instruction_data: &'info [u8],
    ) -> Self {
        Self {
            program_id,
            accounts,
            remaining_accounts,
            instruction_data,
            bumps: BumpSeeds::new(),
        }
    }

    /// Returns the program ID.
    pub fn program_id(&self) -> &Pubkey {
        self.program_id
    }

    /// Returns the instruction data.
    pub fn instruction_data(&self) -> &[u8] {
        self.instruction_data
    }

    /// Returns the remaining accounts.
    pub fn remaining_accounts(&self) -> &[AccountInfo] {
        self.remaining_accounts
    }
}

/// Context for Cross-Program Invocations (CPI).
pub struct CpiContext<'info, T> {
    /// The structured accounts for the CPI
    pub accounts: T,
    /// The remaining accounts for the CPI
    pub remaining_accounts: &'info [AccountInfo],
    /// Signer seeds for PDA signing
    pub signer_seeds: &'info [&'info [&'info [u8]]],
}

impl<'info, T> CpiContext<'info, T> {
    /// Creates a new CpiContext.
    pub fn new(accounts: T, remaining_accounts: &'info [AccountInfo]) -> Self {
        Self {
            accounts,
            remaining_accounts,
            signer_seeds: &[],
        }
    }

    /// Creates a new CpiContext with signer seeds.
    pub fn new_with_signer(
        accounts: T,
        remaining_accounts: &'info [AccountInfo],
        signer_seeds: &'info [&'info [&'info [u8]]],
    ) -> Self {
        Self {
            accounts,
            remaining_accounts,
            signer_seeds,
        }
    }

    /// Returns the signer seeds.
    pub fn signer_seeds(&self) -> &[&[&[u8]]] {
        self.signer_seeds
    }
}

/// Storage for PDA bump seeds.
#[derive(Default)]
pub struct BumpSeeds {
    seeds: [Option<u8>; 16], // Support up to 16 PDAs per instruction
    count: usize,
}

impl BumpSeeds {
    /// Creates a new BumpSeeds storage.
    pub fn new() -> Self {
        Self {
            seeds: [None; 16],
            count: 0,
        }
    }

    /// Adds a bump seed.
    pub fn add(&mut self, bump: u8) -> Result<usize, PinoError> {
        if self.count >= 16 {
            return Err(PinoError::Custom(0x1001)); // Too many PDAs
        }
        
        let index = self.count;
        self.seeds[index] = Some(bump);
        self.count += 1;
        Ok(index)
    }

    /// Gets a bump seed by index.
    pub fn get(&self, index: usize) -> Option<u8> {
        if index < self.count {
            self.seeds[index]
        } else {
            None
        }
    }

    /// Returns the number of stored bump seeds.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns true if no bump seeds are stored.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

/// Trait for types that can be used as account contexts.
///
/// This trait is implemented by the derive macro for account structs.
pub trait Accounts<'info>: Sized {
    /// Tries to deserialize accounts from the given account infos.
    fn try_accounts(
        program_id: &Pubkey,
        accounts: &mut &[AccountInfo],
        instruction_data: &[u8],
        bumps: &mut BumpSeeds,
    ) -> Result<Self, PinoError>;

    /// Performs any necessary cleanup or validation after instruction execution.
    fn exit(&self, program_id: &Pubkey) -> ProgramResult {
        Ok(())
    }
}

/// Trait for types that can be used as CPI account contexts.
pub trait CpiAccounts<'info> {
    /// Converts to account metas for CPI.
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<pinocchio::instruction::AccountMeta>;

    /// Returns the account infos for CPI.
    fn to_account_infos(&self) -> Vec<AccountInfo>;
}

/// Helper trait for instruction data deserialization.
pub trait InstructionData: Sized {
    /// Deserializes instruction data.
    fn try_from_slice(data: &[u8]) -> Result<Self, PinoError>;
}

/// Implements InstructionData for types that implement borsh::BorshDeserialize.
impl<T> InstructionData for T
where
    T: borsh::BorshDeserialize,
{
    fn try_from_slice(data: &[u8]) -> Result<Self, PinoError> {
        borsh::BorshDeserialize::try_from_slice(data)
            .map_err(|_| PinoError::InvalidInstructionData)
    }
} 