//! Program execution context for Pino runtime.

use crate::{RuntimeError, RuntimeResult};
use pinocchio::account_info::AccountInfo;
use pinocchio::pubkey::Pubkey;

/// Program execution context
pub struct ProgramContext<'a> {
    /// Program ID
    pub program_id: &'a Pubkey,
    /// Accounts passed to the program
    pub accounts: &'a [AccountInfo<'a>],
    /// Instruction data
    pub instruction_data: &'a [u8],
}

impl<'a> ProgramContext<'a> {
    /// Create a new program context
    pub fn new(
        program_id: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
        instruction_data: &'a [u8],
    ) -> Self {
        Self {
            program_id,
            accounts,
            instruction_data,
        }
    }

    /// Get the program ID
    pub fn program_id(&self) -> &Pubkey {
        self.program_id
    }

    /// Get the accounts
    pub fn accounts(&self) -> &[AccountInfo<'a>] {
        self.accounts
    }

    /// Get the instruction data
    pub fn instruction_data(&self) -> &[u8] {
        self.instruction_data
    }

    /// Get an account by index
    pub fn get_account(&self, index: usize) -> RuntimeResult<&AccountInfo<'a>> {
        self.accounts
            .get(index)
            .ok_or(RuntimeError::InvalidContext)
    }

    /// Get the number of accounts
    pub fn account_count(&self) -> usize {
        self.accounts.len()
    }

    /// Check if the context is valid
    pub fn is_valid(&self) -> bool {
        !self.accounts.is_empty()
    }

    /// Validate the context
    pub fn validate(&self) -> RuntimeResult<()> {
        if !self.is_valid() {
            return Err(RuntimeError::InvalidContext);
        }
        Ok(())
    }
} 