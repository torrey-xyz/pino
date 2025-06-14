//! Bump allocator implementation for Pino runtime.
//!
//! A simple, fast allocator that allocates memory by incrementing a pointer.
//! Memory is deallocated all at once when the allocator is reset.

use super::{Allocator, AllocationStats};
use crate::{RuntimeError, RuntimeResult};
use core::alloc::Layout;
use core::cell::RefCell;
use core::ptr::NonNull;

/// A bump allocator that allocates memory by incrementing a pointer
pub struct BumpAllocator {
    buffer: RefCell<BumpBuffer>,
}

struct BumpBuffer {
    data: NonNull<u8>,
    size: usize,
    offset: usize,
    stats: AllocationStats,
}

impl BumpAllocator {
    /// Create a new bump allocator with the given size
    pub fn new(size: usize) -> RuntimeResult<Self> {
        if size == 0 {
            return Err(RuntimeError::InvalidSize);
        }

        // For now, we'll use a static buffer approach
        // In a real implementation, this would allocate from heap or use BPF loader
        let layout = Layout::from_size_align(size, 8)
            .map_err(|_| RuntimeError::InvalidSize)?;
        
        // This is a placeholder - in actual implementation would need proper allocation
        let data = NonNull::dangling();
        
        Ok(Self {
            buffer: RefCell::new(BumpBuffer {
                data,
                size,
                offset: 0,
                stats: AllocationStats::new(),
            }),
        })
    }

    /// Get the total size of the allocator
    pub fn size(&self) -> usize {
        self.buffer.borrow().size
    }

    /// Get the current offset
    pub fn offset(&self) -> usize {
        self.buffer.borrow().offset
    }

    /// Get remaining space
    pub fn remaining(&self) -> usize {
        let buffer = self.buffer.borrow();
        buffer.size.saturating_sub(buffer.offset)
    }
}

impl Allocator for BumpAllocator {
    fn allocate(&self, layout: Layout) -> RuntimeResult<NonNull<u8>> {
        let mut buffer = self.buffer.borrow_mut();
        
        // Align the offset
        let align = layout.align();
        let aligned_offset = (buffer.offset + align - 1) & !(align - 1);
        
        // Check if we have enough space
        let end_offset = aligned_offset.checked_add(layout.size())
            .ok_or(RuntimeError::OutOfMemory)?;
        
        if end_offset > buffer.size {
            return Err(RuntimeError::OutOfMemory);
        }
        
        // Update offset
        buffer.offset = end_offset;
        
        // Record allocation
        buffer.stats.record_allocation(layout.size());
        
        // Calculate pointer (this is simplified for the example)
        // In real implementation, would return actual allocated memory
        Ok(NonNull::dangling())
    }

    fn deallocate(&self, _ptr: NonNull<u8>, layout: Layout) {
        // Bump allocator doesn't deallocate individual allocations
        // Just record the deallocation for stats
        let mut buffer = self.buffer.borrow_mut();
        buffer.stats.record_deallocation(layout.size());
    }

    fn stats(&self) -> AllocationStats {
        self.buffer.borrow().stats
    }

    fn reset(&self) -> RuntimeResult<()> {
        let mut buffer = self.buffer.borrow_mut();
        buffer.offset = 0;
        buffer.stats = AllocationStats::new();
        Ok(())
    }
}

unsafe impl Send for BumpAllocator {}
unsafe impl Sync for BumpAllocator {} 