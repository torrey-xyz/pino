//! Prelude for convenient importing of common Pino types and traits.

// Re-export Pinocchio's common types and macros
pub use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars,
    default_allocator,
    default_panic_handler,
    no_allocator,
    msg,
    program_entrypoint,
    lazy_program_entrypoint,
    ProgramResult,
    SUCCESS,
};

// Re-export core Pino modules
pub use crate::{
    account::*,
    context::*,
    error::*,
    instruction::*,
    program::*,
    Result,
};

// Re-export useful collections and memory utilities
pub use crate::{
    collections::*,
    memory::*,
    utils::*,
};

// Re-export macros when available
#[cfg(feature = "macros")]
pub use pino_macros::*;

// Zero-copy utilities
pub use bytemuck::{Pod, Zeroable, cast_ref, cast_mut, cast_slice, cast_slice_mut};

// Common traits and functions
pub use core::{
    mem::{size_of, align_of},
    slice,
    ptr,
};

/// Commonly used result type
pub type PinoResult<T = ()> = crate::Result<T>; 