//! Common utilities and helper functions.
//!
//! This module provides various utilities that are commonly needed
//! in Solana programs built with Pino.

use pinocchio::{pubkey::Pubkey, msg};
use crate::error::PinoError;

/// Logging utilities with minimal CU overhead.
pub mod logging {
    use super::*;

    /// Logs a message with minimal CU overhead.
    #[inline(always)]
    pub fn log_message(message: &str) {
        msg!(message);
    }

    /// Logs a formatted message (only available with std feature).
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn log_formatted(message: &str, args: core::fmt::Arguments) {
        msg!(message, args);
    }

    /// Logs a pubkey for debugging.
    #[inline(always)]
    pub fn log_pubkey(label: &str, pubkey: &Pubkey) {
        msg!(label);
        pinocchio::pubkey::log(pubkey);
    }

    /// Logs a number for debugging.
    #[inline(always)]
    pub fn log_number(label: &str, number: u64) {
        msg!(label);
        pinocchio::log::sol_log_64(number, 0, 0, 0, 0);
    }
}

/// Math utilities with overflow protection.
pub mod math {
    use super::*;

    /// Safely adds two numbers, returning an error on overflow.
    pub fn safe_add(a: u64, b: u64) -> Result<u64, PinoError> {
        a.checked_add(b).ok_or(PinoError::ArithmeticOverflow)
    }

    /// Safely subtracts two numbers, returning an error on underflow.
    pub fn safe_sub(a: u64, b: u64) -> Result<u64, PinoError> {
        a.checked_sub(b).ok_or(PinoError::ArithmeticOverflow)
    }

    /// Safely multiplies two numbers, returning an error on overflow.
    pub fn safe_mul(a: u64, b: u64) -> Result<u64, PinoError> {
        a.checked_mul(b).ok_or(PinoError::ArithmeticOverflow)
    }

    /// Safely divides two numbers, returning an error on division by zero.
    pub fn safe_div(a: u64, b: u64) -> Result<u64, PinoError> {
        a.checked_div(b).ok_or(PinoError::ArithmeticOverflow)
    }

    /// Calculates percentage with precision.
    pub fn percentage(amount: u64, percentage: u64, precision: u64) -> Result<u64, PinoError> {
        let result = safe_mul(amount, percentage)?;
        safe_div(result, precision)
    }

    /// Calculates proportional amount.
    pub fn proportional(amount: u64, numerator: u64, denominator: u64) -> Result<u64, PinoError> {
        let result = safe_mul(amount, numerator)?;
        safe_div(result, denominator)
    }
}

/// Time and clock utilities.
pub mod time {
    use super::*;

    /// Gets the current Unix timestamp from the Clock sysvar.
    pub fn current_timestamp() -> Result<i64, PinoError> {
        // This would require accessing the Clock sysvar
        // For now, return an error indicating it's not implemented
        Err(PinoError::Custom(0x2001))
    }

    /// Checks if a timestamp is in the past.
    pub fn is_past(timestamp: i64) -> Result<bool, PinoError> {
        let current = current_timestamp()?;
        Ok(timestamp < current)
    }

    /// Checks if a timestamp is in the future.
    pub fn is_future(timestamp: i64) -> Result<bool, PinoError> {
        let current = current_timestamp()?;
        Ok(timestamp > current)
    }
}

/// String utilities for no_std environment.
pub mod strings {
    use crate::collections::StackString;

    /// Creates a StackString from a string literal.
    pub fn stack_string<const N: usize>(s: &str) -> StackString<N> {
        let mut stack_str = StackString::new();
        let _ = stack_str.push_str(s);
        stack_str
    }

    /// Compares two byte slices as strings.
    pub fn bytes_equal(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        for (byte_a, byte_b) in a.iter().zip(b.iter()) {
            if byte_a != byte_b {
                return false;
            }
        }
        
        true
    }

    /// Finds a byte in a slice.
    pub fn find_byte(haystack: &[u8], needle: u8) -> Option<usize> {
        for (i, &byte) in haystack.iter().enumerate() {
            if byte == needle {
                return Some(i);
            }
        }
        None
    }
}

/// Validation utilities.
pub mod validation {
    use super::*;

    /// Validates that a pubkey is not the default (all zeros).
    pub fn validate_not_default(pubkey: &Pubkey) -> Result<(), PinoError> {
        if pubkey == &Pubkey::default() {
            return Err(PinoError::InvalidProgramId);
        }
        Ok(())
    }

    /// Validates that a number is within a range.
    pub fn validate_range(value: u64, min: u64, max: u64) -> Result<(), PinoError> {
        if value < min || value > max {
            return Err(PinoError::Custom(0x2002));
        }
        Ok(())
    }

    /// Validates that a slice is not empty.
    pub fn validate_not_empty<T>(slice: &[T]) -> Result<(), PinoError> {
        if slice.is_empty() {
            return Err(PinoError::Custom(0x2003));
        }
        Ok(())
    }

    /// Validates that a string contains only ASCII characters.
    pub fn validate_ascii(bytes: &[u8]) -> Result<(), PinoError> {
        for &byte in bytes {
            if !byte.is_ascii() {
                return Err(PinoError::Custom(0x2004));
            }
        }
        Ok(())
    }
}

/// Conversion utilities.
pub mod convert {
    use super::*;

    /// Converts a u64 to bytes in little-endian format.
    pub fn u64_to_bytes(value: u64) -> [u8; 8] {
        value.to_le_bytes()
    }

    /// Converts bytes to u64 in little-endian format.
    pub fn bytes_to_u64(bytes: &[u8]) -> Result<u64, PinoError> {
        if bytes.len() < 8 {
            return Err(PinoError::InvalidInstructionData);
        }
        
        let mut array = [0u8; 8];
        array.copy_from_slice(&bytes[..8]);
        Ok(u64::from_le_bytes(array))
    }

    /// Converts a u32 to bytes in little-endian format.
    pub fn u32_to_bytes(value: u32) -> [u8; 4] {
        value.to_le_bytes()
    }

    /// Converts bytes to u32 in little-endian format.
    pub fn bytes_to_u32(bytes: &[u8]) -> Result<u32, PinoError> {
        if bytes.len() < 4 {
            return Err(PinoError::InvalidInstructionData);
        }
        
        let mut array = [0u8; 4];
        array.copy_from_slice(&bytes[..4]);
        Ok(u32::from_le_bytes(array))
    }
}

/// Macro utilities for common patterns.
#[macro_export]
macro_rules! log_compute_units {
    ($message:expr) => {
        #[cfg(feature = "profiling")]
        {
            let remaining = pinocchio::syscalls::sol_remaining_compute_units();
            $crate::utils::logging::log_message(&format!("{}: {} CU remaining", $message, remaining));
        }
    };
}

/// Macro for conditional compilation based on target.
#[macro_export]
macro_rules! if_solana {
    ($($item:item)*) => {
        #[cfg(target_os = "solana")]
        $($item)*
    };
}

/// Macro for non-Solana targets (testing, etc.).
#[macro_export]
macro_rules! if_not_solana {
    ($($item:item)*) => {
        #[cfg(not(target_os = "solana"))]
        $($item)*
    };
}

// Re-export utility macros
pub use {log_compute_units, if_solana, if_not_solana}; 