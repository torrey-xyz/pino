//! High-level entrypoint system built on Pinocchio.
//!
//! This module provides automatic instruction routing and account parsing
//! while maintaining Pinocchio's zero-copy efficiency.

use pinocchio::{
    account_info::AccountInfo,
    entrypoint::InstructionContext,
    pubkey::Pubkey,
    ProgramResult,
};
use crate::{
    context::{Context, BumpSeeds, Accounts},
    error::PinoError,
    instruction::InstructionData,
};

/// Trait for program instruction processors.
///
/// This trait should be implemented by your program's instruction enum.
pub trait ProgramInstruction: InstructionData {
    /// Processes the instruction with the given accounts and context.
    fn process<'info>(
        &self,
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult;
}

/// Processes a program instruction using Pino's high-level interface.
///
/// This function provides automatic instruction deserialization and routing
/// while maintaining Pinocchio's zero-copy efficiency.
pub fn process_instruction<T: ProgramInstruction>(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize the instruction
    let instruction = T::try_from_slice(instruction_data)
        .map_err(|e| pinocchio::program_error::ProgramError::from(e))?;

    // Process the instruction
    instruction.process(program_id, accounts, instruction_data)
}

/// Processes a program instruction using Pino's lazy interface.
///
/// This provides maximum CU efficiency by only parsing accounts as needed.
pub fn process_instruction_lazy<T: ProgramInstruction>(
    mut ctx: InstructionContext,
) -> ProgramResult {
    // Get program ID and instruction data
    let program_id = unsafe { ctx.program_id_unchecked() };
    let instruction_data = ctx.instruction_data()
        .map_err(|e| pinocchio::program_error::ProgramError::from(e))?;

    // Deserialize the instruction
    let instruction = T::try_from_slice(instruction_data)
        .map_err(|e| pinocchio::program_error::ProgramError::from(e))?;

    // Collect all accounts
    let mut accounts = heapless::Vec::<AccountInfo, 64>::new();
    let mut account_iter = ctx.accounts();
    while let Ok(maybe_account) = account_iter.next() {
        match maybe_account {
            pinocchio::entrypoint::MaybeAccount::Account(account) => {
                accounts.push(account).map_err(|_| {
                    pinocchio::program_error::ProgramError::Custom(0x1002) // Too many accounts
                })?;
            }
            pinocchio::entrypoint::MaybeAccount::Duplicated(index) => {
                if let Some(original) = accounts.get(index as usize) {
                    accounts.push(original.clone()).map_err(|_| {
                        pinocchio::program_error::ProgramError::Custom(0x1002)
                    })?;
                } else {
                    return Err(pinocchio::program_error::ProgramError::Custom(0x1003)); // Invalid duplicate index
                }
            }
        }
    }

    // Process the instruction
    instruction.process(program_id, &accounts, instruction_data)
}

/// Helper function to parse accounts into a structured context.
pub fn parse_accounts<'info, T: Accounts<'info>>(
    program_id: &'info Pubkey,
    accounts: &mut &'info [AccountInfo],
    instruction_data: &'info [u8],
) -> Result<Context<'info, T>, PinoError> {
    let mut bumps = BumpSeeds::new();
    
    // Parse the accounts using the Accounts trait
    let parsed_accounts = T::try_accounts(program_id, accounts, instruction_data, &mut bumps)?;
    
    // Create the context
    Ok(Context {
        program_id,
        accounts: parsed_accounts,
        remaining_accounts: accounts,
        instruction_data,
        bumps,
    })
}

/// Macro to generate a program entrypoint with automatic instruction routing.
#[macro_export]
macro_rules! pino_entrypoint {
    ($processor:ty) => {
        /// Program entrypoint using Pino's high-level interface
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            use $crate::entrypoint::process_instruction;
            
            // Use Pinocchio's standard entrypoint for parsing
            let (program_id, accounts, instruction_data) = 
                pinocchio::entrypoint::deserialize::<64>(input, &mut [
                    core::mem::MaybeUninit::uninit(); 64
                ]);

            match process_instruction::<$processor>(
                &program_id,
                core::slice::from_raw_parts(accounts.as_ptr() as _, accounts.len()),
                &instruction_data,
            ) {
                Ok(()) => $crate::SUCCESS,
                Err(error) => error.into(),
            }
        }
        
        // Set up allocator and panic handler
        pinocchio::default_allocator!();
        pinocchio::default_panic_handler!();
    };
}

/// Macro to generate a lazy program entrypoint for maximum CU efficiency.
#[macro_export]
macro_rules! pino_lazy_entrypoint {
    ($processor:ty) => {
        /// Program entrypoint using Pino's lazy interface
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            use $crate::entrypoint::process_instruction_lazy;
            
            let ctx = pinocchio::entrypoint::InstructionContext::new_unchecked(input);
            
            match process_instruction_lazy::<$processor>(ctx) {
                Ok(()) => $crate::SUCCESS,
                Err(error) => error.into(),
            }
        }
        
        // Set up allocator and panic handler
        pinocchio::default_allocator!();
        pinocchio::default_panic_handler!();
    };
}

/// Macro to generate a no-allocator program entrypoint.
#[macro_export]
macro_rules! pino_no_alloc_entrypoint {
    ($processor:ty) => {
        /// Program entrypoint with no allocator
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            use $crate::entrypoint::process_instruction_lazy;
            
            let ctx = pinocchio::entrypoint::InstructionContext::new_unchecked(input);
            
            match process_instruction_lazy::<$processor>(ctx) {
                Ok(()) => $crate::SUCCESS,
                Err(error) => error.into(),
            }
        }
        
        // Use no allocator for maximum efficiency
        pinocchio::no_allocator!();
        pinocchio::default_panic_handler!();
    };
}

// Re-export commonly used entrypoint macros
pub use {pino_entrypoint, pino_lazy_entrypoint, pino_no_alloc_entrypoint}; 