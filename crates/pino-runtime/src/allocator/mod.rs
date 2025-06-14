//! Memory allocators for Pino runtime.
//!
//! This module provides various memory allocation strategies optimized for
//! Solana programs, including bump allocation, region allocation, pool allocation,
//! and stack allocation.

use crate::RuntimeResult;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

pub mod bump;
pub mod pool;
pub mod region;
pub mod stack;

/// Core allocator trait
pub trait Allocator {
    /// Allocate memory with the given layout
    fn allocate(&self, layout: Layout) -> RuntimeResult<NonNull<u8>>;
    
    /// Deallocate memory
    fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
    
    /// Get allocation statistics
    fn stats(&self) -> AllocationStats;
    
    /// Reset the allocator (if supported)
    fn reset(&self) -> RuntimeResult<()>;
}

/// Allocation statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct AllocationStats {
    /// Total bytes allocated
    pub total_allocated: usize,
    /// Total bytes deallocated
    pub total_deallocated: usize,
    /// Current bytes in use
    pub current_usage: usize,
    /// Peak usage
    pub peak_usage: usize,
    /// Number of allocations
    pub allocation_count: usize,
    /// Number of deallocations
    pub deallocation_count: usize,
}

impl AllocationStats {
    /// Create new empty stats
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record an allocation
    pub fn record_allocation(&mut self, size: usize) {
        self.total_allocated = self.total_allocated.saturating_add(size);
        self.current_usage = self.current_usage.saturating_add(size);
        self.allocation_count = self.allocation_count.saturating_add(1);
        
        if self.current_usage > self.peak_usage {
            self.peak_usage = self.current_usage;
        }
    }
    
    /// Record a deallocation
    pub fn record_deallocation(&mut self, size: usize) {
        self.total_deallocated = self.total_deallocated.saturating_add(size);
        self.current_usage = self.current_usage.saturating_sub(size);
        self.deallocation_count = self.deallocation_count.saturating_add(1);
    }
}

/// Initialize the default allocator
pub fn init_default() -> RuntimeResult<()> {
    // Use bump allocator as default
    let allocator = bump::BumpAllocator::new(8192)?; // 8KB default
    set_global(allocator)
}

/// Set a global allocator
pub fn set_global<A: Allocator>(_allocator: A) -> RuntimeResult<()> {
    // For now, this is a placeholder since setting global allocators
    // requires more complex integration with the Rust allocator system
    Ok(())
}

/// Get current allocation statistics
pub fn get_stats() -> AllocationStats {
    // Placeholder - would integrate with global allocator
    AllocationStats::new()
} 