//! Instruction handling and CPI support.
//!
//! This module provides structured instruction handling and Cross-Program
//! Invocation (CPI) support built on Pinocchio's efficient syscalls.

extern crate alloc;

use alloc::vec::Vec;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    pubkey::Pubkey,
    ProgramResult,
};
use crate::{
    context::CpiContext,
    error::PinoError,
};

/// Re-export instruction data trait from context
pub use crate::context::InstructionData;

/// Trait for types that can be converted to instruction data.
pub trait ToInstructionData {
    /// Serializes the type to instruction data.
    fn to_instruction_data(&self) -> Result<Vec<u8>, PinoError>;
}

/// Implements ToInstructionData for types that implement borsh::BorshSerialize.
impl<T> ToInstructionData for T
where
    T: borsh::BorshSerialize,
{
    fn to_instruction_data(&self) -> Result<Vec<u8>, PinoError> {
        borsh::to_vec(self)
            .map_err(|_| PinoError::InvalidInstructionData)
    }
}

/// Performs a Cross-Program Invocation (CPI) using slice invoke.
///
/// This function provides a high-level interface for CPI calls while
/// maintaining Pinocchio's efficiency.
pub fn invoke(
    instruction: &Instruction,
    account_infos: &[&AccountInfo],
) -> ProgramResult {
    pinocchio::cpi::slice_invoke(instruction, account_infos)
}

/// Performs a Cross-Program Invocation (CPI) with signer seeds using slice invoke.
///
/// This allows PDAs to sign CPI calls.
pub fn invoke_signed(
    instruction: &Instruction,
    account_infos: &[&AccountInfo],
    signer_seeds: &[Signer],
) -> ProgramResult {
    pinocchio::cpi::slice_invoke_signed(instruction, account_infos, signer_seeds)
}

/// Helper function to create an instruction with owned data.
pub fn create_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountMeta<'a>],
    data: &'a [u8],
) -> Instruction<'a, 'a, 'a, 'a> {
    Instruction {
        program_id,
        accounts,
        data,
    }
}

/// Builder for creating instructions with a fluent API.
pub struct InstructionBuilder<'a> {
    program_id: &'a Pubkey,
    accounts: Vec<AccountMeta<'a>>,
    data: Vec<u8>,
}

impl<'a> InstructionBuilder<'a> {
    /// Creates a new instruction builder.
    pub fn new(program_id: &'a Pubkey) -> Self {
        Self {
            program_id,
            accounts: Vec::new(),
            data: Vec::new(),
        }
    }

    /// Adds an account to the instruction.
    pub fn account(mut self, account: AccountMeta<'a>) -> Self {
        self.accounts.push(account);
        self
    }

    /// Adds multiple accounts to the instruction.
    pub fn accounts(mut self, accounts: Vec<AccountMeta<'a>>) -> Self {
        self.accounts.extend(accounts);
        self
    }

    /// Sets the instruction data.
    pub fn data<T: ToInstructionData>(mut self, data: T) -> Result<Self, PinoError> {
        self.data = data.to_instruction_data()?;
        Ok(self)
    }

    /// Sets the instruction data from raw bytes.
    pub fn data_raw(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }

    /// Builds the instruction.
    pub fn build(&'a self) -> Instruction<'a, 'a, 'a, 'a> {
        Instruction {
            program_id: self.program_id,
            accounts: &self.accounts,
            data: &self.data,
        }
    }

    /// Builds and invokes the instruction.
    pub fn invoke(&'a self, account_infos: &[&AccountInfo]) -> ProgramResult {
        let instruction = self.build();
        invoke(&instruction, account_infos)
    }

    /// Builds and invokes the instruction with signer seeds.
    pub fn invoke_signed(
        &'a self,
        account_infos: &[&AccountInfo],
        signer_seeds: &[Signer],
    ) -> ProgramResult {
        let instruction = self.build();
        invoke_signed(&instruction, account_infos, signer_seeds)
    }
}

/// Helper functions for creating account metas.
pub mod account_meta {
    use super::*;

    /// Creates a writable, signer account meta.
    pub fn writable_signer(pubkey: &Pubkey) -> AccountMeta {
        AccountMeta::writable_signer(pubkey)
    }

    /// Creates a read-only, signer account meta.
    pub fn readonly_signer(pubkey: &Pubkey) -> AccountMeta {
        AccountMeta::readonly_signer(pubkey)
    }

    /// Creates a writable, non-signer account meta.
    pub fn writable(pubkey: &Pubkey) -> AccountMeta {
        AccountMeta::writable(pubkey)
    }

    /// Creates a read-only, non-signer account meta.
    pub fn readonly(pubkey: &Pubkey) -> AccountMeta {
        AccountMeta::readonly(pubkey)
    }
}

/// Common system program instructions.
pub mod system {
    use super::*;
    use crate::account::program_ids::SYSTEM_PROGRAM_ID;

    /// Creates a system program transfer instruction with owned data.
    pub fn transfer<'a>(from: &'a Pubkey, to: &'a Pubkey, lamports: u64) -> (Vec<AccountMeta<'a>>, Vec<u8>) {
        let accounts = alloc::vec![
            account_meta::writable_signer(from),
            account_meta::writable(to),
        ];

        // System program transfer instruction data
        let mut data = Vec::with_capacity(12);
        data.extend_from_slice(&2u32.to_le_bytes()); // Transfer instruction discriminator
        data.extend_from_slice(&lamports.to_le_bytes());

        (accounts, data)
    }

    /// Creates a system program create account instruction with owned data.
    pub fn create_account<'a>(
        from: &'a Pubkey,
        to: &'a Pubkey,
        lamports: u64,
        space: u64,
        owner: &'a Pubkey,
    ) -> (Vec<AccountMeta<'a>>, Vec<u8>) {
        let accounts = alloc::vec![
            account_meta::writable_signer(from),
            account_meta::writable_signer(to),
        ];

        // System program create account instruction data
        let mut data = Vec::with_capacity(52);
        data.extend_from_slice(&0u32.to_le_bytes()); // CreateAccount instruction discriminator
        data.extend_from_slice(&lamports.to_le_bytes());
        data.extend_from_slice(&space.to_le_bytes());
        data.extend_from_slice(owner.as_ref());

        (accounts, data)
    }

    /// Creates a system program transfer instruction.
    pub fn transfer_ix<'a>(
        from: &'a Pubkey,
        to: &'a Pubkey,
        lamports: u64,
        accounts: &'a [AccountMeta<'a>],
        data: &'a [u8],
    ) -> Instruction<'a, 'a, 'a, 'a> {
        Instruction {
            program_id: &SYSTEM_PROGRAM_ID,
            accounts,
            data,
        }
    }

    /// Creates a system program create account instruction.
    pub fn create_account_ix<'a>(
        from: &'a Pubkey,
        to: &'a Pubkey,
        lamports: u64,
        space: u64,
        owner: &'a Pubkey,
        accounts: &'a [AccountMeta<'a>],
        data: &'a [u8],
    ) -> Instruction<'a, 'a, 'a, 'a> {
        Instruction {
            program_id: &SYSTEM_PROGRAM_ID,
            accounts,
            data,
        }
    }
}

/// Macro to simplify CPI calls.
#[macro_export]
macro_rules! cpi {
    ($ctx:expr, $instruction:expr) => {
        $crate::instruction::invoke(&$instruction, &$ctx.to_account_infos())
    };
    ($ctx:expr, $instruction:expr, $signer_seeds:expr) => {
        $crate::instruction::invoke_signed(
            &$instruction,
            &$ctx.to_account_infos(),
            $signer_seeds,
        )
    };
}

// Re-export the CPI macro
pub use cpi; 