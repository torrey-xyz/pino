//! Pool allocator implementation for Pino runtime.
//!
//! A pool allocator manages fixed-size blocks of memory for efficient allocation
//! and deallocation of objects of the same size.

use super::{Allocator, AllocationStats};
use crate::{RuntimeError, RuntimeResult};
use core::alloc::Layout;
use core::cell::RefCell;
use core::ptr::NonNull;

/// A pool allocator for fixed-size blocks
pub struct PoolAllocator {
    inner: RefCell<PoolInner>,
}

struct PoolInner {
    block_size: usize,
    block_count: usize,
    free_blocks: usize,
    stats: AllocationStats,
}

impl PoolAllocator {
    /// Create a new pool allocator
    pub fn new(block_size: usize, block_count: usize) -> RuntimeResult<Self> {
        if block_size == 0 || block_count == 0 {
            return Err(RuntimeError::InvalidSize);
        }

        Ok(Self {
            inner: RefCell::new(PoolInner {
                block_size,
                block_count,
                free_blocks: block_count,
                stats: AllocationStats::new(),
            }),
        })
    }

    /// Get the block size
    pub fn block_size(&self) -> usize {
        self.inner.borrow().block_size
    }

    /// Get the total number of blocks
    pub fn block_count(&self) -> usize {
        self.inner.borrow().block_count
    }

    /// Get the number of free blocks
    pub fn free_blocks(&self) -> usize {
        self.inner.borrow().free_blocks
    }
}

impl Allocator for PoolAllocator {
    fn allocate(&self, layout: Layout) -> RuntimeResult<NonNull<u8>> {
        let mut inner = self.inner.borrow_mut();
        
        // Check if the requested size fits in our block size
        if layout.size() > inner.block_size {
            return Err(RuntimeError::InvalidSize);
        }
        
        // Check if we have free blocks
        if inner.free_blocks == 0 {
            return Err(RuntimeError::OutOfMemory);
        }
        
        // Allocate a block
        inner.free_blocks -= 1;
        inner.stats.record_allocation(inner.block_size);
        
        // Return a dangling pointer as placeholder
        Ok(NonNull::dangling())
    }

    fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        let mut inner = self.inner.borrow_mut();
        
        // Return block to pool
        inner.free_blocks += 1;
        inner.stats.record_deallocation(inner.block_size);
    }

    fn stats(&self) -> AllocationStats {
        self.inner.borrow().stats
    }

    fn reset(&self) -> RuntimeResult<()> {
        let mut inner = self.inner.borrow_mut();
        inner.free_blocks = inner.block_count;
        inner.stats = AllocationStats::new();
        Ok(())
    }
}

unsafe impl Send for PoolAllocator {}
unsafe impl Sync for PoolAllocator {} 