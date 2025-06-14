#![no_std]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

//! # Pino Runtime
//!
//! Runtime components and allocators for the Pino Solana framework.
//! Provides memory allocators, execution context, and CU profiling utilities.

extern crate alloc;

use pinocchio::account_info::AccountInfo;
use pinocchio::pubkey::Pubkey;

// Runtime modules
pub mod allocator;
pub mod context;
pub mod profiler;

// Re-exports from allocator module
pub use allocator::{
    bump::BumpAllocator,
    pool::PoolAllocator,
    region::RegionAllocator,
    stack::StackAllocator,
};

// Re-exports from context module
pub use context::{
    program::ProgramContext,
};

// Re-exports from profiler module
pub use profiler::{
    cu_tracker::CuTracker,
};

/// Runtime result type
pub type RuntimeResult<T = ()> = core::result::Result<T, RuntimeError>;

/// Runtime errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeError {
    /// Out of memory
    OutOfMemory,
    /// Invalid allocation size
    InvalidSize,
    /// Invalid program context
    InvalidContext,
    /// Profiling error
    ProfilingError,
}

impl core::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RuntimeError::OutOfMemory => write!(f, "Out of memory"),
            RuntimeError::InvalidSize => write!(f, "Invalid allocation size"),
            RuntimeError::InvalidContext => write!(f, "Invalid program context"),
            RuntimeError::ProfilingError => write!(f, "Profiling error"),
        }
    }
}

/// Initialize the runtime with default settings
pub fn init() -> RuntimeResult<()> {
    // Initialize default allocator
    allocator::init_default()?;
    Ok(())
}

/// Initialize the runtime with custom allocator
pub fn init_with_allocator<A: allocator::Allocator>(allocator: A) -> RuntimeResult<()> {
    allocator::set_global(allocator)?;
    Ok(())
} 