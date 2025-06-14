//! Memory operations and utilities.
//!
//! This module provides zero-copy memory operations and utilities
//! built on Pinocchio's efficient memory syscalls.

use core::{mem, slice, ptr};
use bytemuck::{Pod, Zeroable};

// Re-export Pinocchio's memory operations
pub use pinocchio::memory::{
    sol_memcpy, sol_memmove, sol_memcmp, sol_memset, copy_val
};

/// Zero-copy memory utilities for efficient data handling.
pub struct ZeroCopy;

impl ZeroCopy {
    /// Casts a byte slice to a reference of type T.
    ///
    /// # Safety
    /// 
    /// The caller must ensure that:
    /// - The slice is properly aligned for type T
    /// - The slice is at least as large as T
    /// - The bytes represent a valid value of type T
    pub unsafe fn cast_ref<T: Pod>(bytes: &[u8]) -> &T {
        bytemuck::from_bytes(&bytes[..mem::size_of::<T>()])
    }

    /// Casts a mutable byte slice to a mutable reference of type T.
    ///
    /// # Safety
    /// 
    /// The caller must ensure that:
    /// - The slice is properly aligned for type T
    /// - The slice is at least as large as T
    /// - The bytes represent a valid value of type T
    pub unsafe fn cast_mut<T: Pod>(bytes: &mut [u8]) -> &mut T {
        bytemuck::from_bytes_mut(&mut bytes[..mem::size_of::<T>()])
    }

    /// Casts a byte slice to a slice of type T.
    ///
    /// # Safety
    /// 
    /// The caller must ensure that:
    /// - The slice is properly aligned for type T
    /// - The slice length is a multiple of size_of::<T>()
    /// - All bytes represent valid values of type T
    pub unsafe fn cast_slice<T: Pod>(bytes: &[u8]) -> &[T] {
        bytemuck::cast_slice(bytes)
    }

    /// Casts a mutable byte slice to a mutable slice of type T.
    ///
    /// # Safety
    /// 
    /// The caller must ensure that:
    /// - The slice is properly aligned for type T
    /// - The slice length is a multiple of size_of::<T>()
    /// - All bytes represent valid values of type T
    pub unsafe fn cast_slice_mut<T: Pod>(bytes: &mut [u8]) -> &mut [T] {
        bytemuck::cast_slice_mut(bytes)
    }

    /// Safely casts a byte slice to a reference of type T with validation.
    pub fn try_cast_ref<T: Pod>(bytes: &[u8]) -> Result<&T, MemoryError> {
        if bytes.len() < mem::size_of::<T>() {
            return Err(MemoryError::InsufficientSize);
        }

        if bytes.as_ptr() as usize % mem::align_of::<T>() != 0 {
            return Err(MemoryError::InvalidAlignment);
        }

        Ok(bytemuck::from_bytes(&bytes[..mem::size_of::<T>()]))
    }

    /// Safely casts a mutable byte slice to a mutable reference of type T with validation.
    pub fn try_cast_mut<T: Pod>(bytes: &mut [u8]) -> Result<&mut T, MemoryError> {
        if bytes.len() < mem::size_of::<T>() {
            return Err(MemoryError::InsufficientSize);
        }

        if bytes.as_ptr() as usize % mem::align_of::<T>() != 0 {
            return Err(MemoryError::InvalidAlignment);
        }

        Ok(bytemuck::from_bytes_mut(&mut bytes[..mem::size_of::<T>()]))
    }
}

/// Memory operation errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryError {
    /// Insufficient buffer size for the operation
    InsufficientSize,
    /// Invalid memory alignment for the type
    InvalidAlignment,
    /// Memory region overlap detected
    OverlapDetected,
}

/// Efficient memory comparison that compiles to minimal CU overhead.
#[inline(always)]
pub fn compare_bytes(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    if a.len() == 0 {
        return true;
    }

    unsafe {
        sol_memcmp(a, b, a.len()) == 0
    }
}

/// Efficient memory copy that uses Solana syscalls for optimal performance.
#[inline(always)]
pub fn copy_bytes(dst: &mut [u8], src: &[u8], len: usize) {
    debug_assert!(dst.len() >= len);
    debug_assert!(src.len() >= len);
    
    if len > 0 {
        unsafe {
            sol_memcpy(dst, src, len);
        }
    }
}

/// Efficient memory set that uses Solana syscalls.
#[inline(always)]
pub fn set_bytes(dst: &mut [u8], value: u8, len: usize) {
    debug_assert!(dst.len() >= len);
    
    if len > 0 {
        unsafe {
            sol_memset(dst, value, len);
        }
    }
}

/// Memory layout utilities for account data structures.
pub mod layout {
    use super::*;

    /// Calculates the total size needed for a struct with trailing data.
    pub const fn size_with_trailing_data<T>(trailing_len: usize) -> usize {
        mem::size_of::<T>() + trailing_len
    }

    /// Calculates the offset of trailing data after a struct.
    pub const fn trailing_data_offset<T>() -> usize {
        mem::size_of::<T>()
    }

    /// Splits a byte slice into a struct reference and trailing data.
    pub fn split_struct_and_data<T: Pod>(
        bytes: &[u8],
    ) -> Result<(&T, &[u8]), MemoryError> {
        if bytes.len() < mem::size_of::<T>() {
            return Err(MemoryError::InsufficientSize);
        }

        let (struct_bytes, data_bytes) = bytes.split_at(mem::size_of::<T>());
        let struct_ref = ZeroCopy::try_cast_ref::<T>(struct_bytes)?;
        
        Ok((struct_ref, data_bytes))
    }

    /// Splits a mutable byte slice into a mutable struct reference and trailing data.
    pub fn split_struct_and_data_mut<T: Pod>(
        bytes: &mut [u8],
    ) -> Result<(&mut T, &mut [u8]), MemoryError> {
        if bytes.len() < mem::size_of::<T>() {
            return Err(MemoryError::InsufficientSize);
        }

        let (struct_bytes, data_bytes) = bytes.split_at_mut(mem::size_of::<T>());
        let struct_ref = ZeroCopy::try_cast_mut::<T>(struct_bytes)?;
        
        Ok((struct_ref, data_bytes))
    }
}

/// Alignment utilities for memory operations.
pub mod align {
    use core::mem;

    /// Aligns a size up to the next multiple of the alignment.
    pub const fn align_up(size: usize, alignment: usize) -> usize {
        (size + alignment - 1) & !(alignment - 1)
    }

    /// Aligns a size down to the previous multiple of the alignment.
    pub const fn align_down(size: usize, alignment: usize) -> usize {
        size & !(alignment - 1)
    }

    /// Checks if a size is aligned to the given alignment.
    pub const fn is_aligned(size: usize, alignment: usize) -> bool {
        size & (alignment - 1) == 0
    }

    /// Returns the alignment requirement for type T.
    pub const fn alignment_of<T>() -> usize {
        mem::align_of::<T>()
    }
} 