//! Stack allocator implementation for Pino runtime.
//!
//! A stack allocator allocates memory in LIFO order, making it very fast
//! but requiring deallocations to happen in reverse order.

use super::{Allocator, AllocationStats};
use crate::{RuntimeError, RuntimeResult};
use core::alloc::Layout;
use core::cell::RefCell;
use core::ptr::NonNull;

/// A stack-based allocator
pub struct StackAllocator {
    inner: RefCell<StackInner>,
}

struct StackInner {
    size: usize,
    top: usize,
    stats: AllocationStats,
}

impl StackAllocator {
    /// Create a new stack allocator
    pub fn new(size: usize) -> RuntimeResult<Self> {
        if size == 0 {
            return Err(RuntimeError::InvalidSize);
        }

        Ok(Self {
            inner: RefCell::new(StackInner {
                size,
                top: 0,
                stats: AllocationStats::new(),
            }),
        })
    }

    /// Get the total size
    pub fn size(&self) -> usize {
        self.inner.borrow().size
    }

    /// Get the current top position
    pub fn top(&self) -> usize {
        self.inner.borrow().top
    }

    /// Get remaining space
    pub fn remaining(&self) -> usize {
        let inner = self.inner.borrow();
        inner.size.saturating_sub(inner.top)
    }

    /// Create a stack frame (checkpoint)
    pub fn push_frame(&self) -> StackFrame {
        StackFrame {
            position: self.inner.borrow().top,
        }
    }

    /// Restore to a previous frame
    pub fn pop_frame(&self, frame: StackFrame) -> RuntimeResult<()> {
        let mut inner = self.inner.borrow_mut();
        if frame.position > inner.top {
            return Err(RuntimeError::InvalidContext);
        }
        inner.top = frame.position;
        Ok(())
    }
}

/// A stack frame representing a checkpoint
#[derive(Debug, Clone, Copy)]
pub struct StackFrame {
    position: usize,
}

impl Allocator for StackAllocator {
    fn allocate(&self, layout: Layout) -> RuntimeResult<NonNull<u8>> {
        let mut inner = self.inner.borrow_mut();
        
        // Align the top
        let align = layout.align();
        let aligned_top = (inner.top + align - 1) & !(align - 1);
        
        // Check if we have enough space
        let end_top = aligned_top.checked_add(layout.size())
            .ok_or(RuntimeError::OutOfMemory)?;
        
        if end_top > inner.size {
            return Err(RuntimeError::OutOfMemory);
        }
        
        // Update top
        inner.top = end_top;
        
        // Record allocation
        inner.stats.record_allocation(layout.size());
        
        // Return a dangling pointer as placeholder
        Ok(NonNull::dangling())
    }

    fn deallocate(&self, _ptr: NonNull<u8>, layout: Layout) {
        // Stack allocator expects deallocations in reverse order
        // For now, just record the deallocation
        let mut inner = self.inner.borrow_mut();
        inner.stats.record_deallocation(layout.size());
        
        // In a real implementation, we would validate that this is the last allocation
        // and adjust the top pointer accordingly
    }

    fn stats(&self) -> AllocationStats {
        self.inner.borrow().stats
    }

    fn reset(&self) -> RuntimeResult<()> {
        let mut inner = self.inner.borrow_mut();
        inner.top = 0;
        inner.stats = AllocationStats::new();
        Ok(())
    }
}

unsafe impl Send for StackAllocator {}
unsafe impl Sync for StackAllocator {} 