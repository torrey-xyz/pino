#![no_std]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

//! # Pino Framework
//!
//! A high-performance Solana program framework built on top of Pinocchio,
//! providing zero-copy abstractions with 60-90% CU reduction compared to Anchor.
//!
//! Pino combines Pinocchio's zero-copy efficiency with developer-friendly APIs,
//! automatic instruction routing, and advanced memory management.
//!
//! ## Quick Start
//!
//! ```ignore
//! use pino::prelude::*;
//!
//! #[pino_program]
//! pub mod my_program {
//!     use super::*;
//!
//!     #[instruction]
//!     pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
//!         msg!("Initializing with zero-copy efficiency!");
//!         Ok(())
//!     }
//! }
//!
//! #[derive(Accounts)]
//! pub struct Initialize<'info> {
//!     #[account(init, payer = user, space = 8 + 32)]
//!     pub data: Account<'info, MyData>,
//!     #[account(mut)]
//!     pub user: Signer<'info>,
//!     pub system_program: Program<'info, System>,
//! }
//! ```

extern crate self as pino_core;

// Re-export Pinocchio as the foundation
pub use pinocchio;

// Core framework modules
pub mod account;
pub mod context;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod program;

// Memory and performance optimizations
pub mod collections;
pub mod memory;

// Utilities
pub mod utils;

// Convenient prelude
pub mod prelude;

// Re-export macros when available
#[cfg(feature = "macros")]
pub use pino_macros::*;

// Core types from Pinocchio
pub use pinocchio::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    ProgramResult,
};

/// The main result type for Pino programs
pub type Result<T = ()> = core::result::Result<T, error::PinoError>;

/// Success constant
pub const SUCCESS: u64 = pinocchio::SUCCESS;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Minimum supported Agave version
pub const MIN_AGAVE_VERSION: &str = "2.2.16"; 