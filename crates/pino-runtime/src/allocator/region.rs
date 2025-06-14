//! Region allocator implementation for Pino runtime.
//!
//! A region allocator divides memory into regions and manages allocation within those regions.

use super::{Allocator, AllocationStats};
use crate::{RuntimeError, RuntimeResult};
use core::alloc::Layout;
use core::cell::RefCell;
use core::ptr::NonNull;

/// A region-based allocator
pub struct RegionAllocator {
    inner: RefCell<RegionInner>,
}

struct RegionInner {
    region_size: usize,
    region_count: usize,
    current_region: usize,
    region_offset: usize,
    stats: AllocationStats,
}

impl RegionAllocator {
    /// Create a new region allocator
    pub fn new(region_size: usize, region_count: usize) -> RuntimeResult<Self> {
        if region_size == 0 || region_count == 0 {
            return Err(RuntimeError::InvalidSize);
        }

        Ok(Self {
            inner: RefCell::new(RegionInner {
                region_size,
                region_count,
                current_region: 0,
                region_offset: 0,
                stats: AllocationStats::new(),
            }),
        })
    }

    /// Get the region size
    pub fn region_size(&self) -> usize {
        self.inner.borrow().region_size
    }

    /// Get the number of regions
    pub fn region_count(&self) -> usize {
        self.inner.borrow().region_count
    }

    /// Get the current region
    pub fn current_region(&self) -> usize {
        self.inner.borrow().current_region
    }
}

impl Allocator for RegionAllocator {
    fn allocate(&self, layout: Layout) -> RuntimeResult<NonNull<u8>> {
        let mut inner = self.inner.borrow_mut();
        
        // Check if allocation fits in a single region
        if layout.size() > inner.region_size {
            return Err(RuntimeError::InvalidSize);
        }
        
        // Check if allocation fits in current region
        let aligned_offset = (inner.region_offset + layout.align() - 1) & !(layout.align() - 1);
        let end_offset = aligned_offset.checked_add(layout.size())
            .ok_or(RuntimeError::OutOfMemory)?;
        
        if end_offset > inner.region_size {
            // Move to next region
            inner.current_region += 1;
            if inner.current_region >= inner.region_count {
                return Err(RuntimeError::OutOfMemory);
            }
            inner.region_offset = 0;
            
            // Recalculate for new region
            let aligned_offset = (inner.region_offset + layout.align() - 1) & !(layout.align() - 1);
            let end_offset = aligned_offset.checked_add(layout.size())
                .ok_or(RuntimeError::OutOfMemory)?;
            
            inner.region_offset = end_offset;
        } else {
            inner.region_offset = end_offset;
        }
        
        inner.stats.record_allocation(layout.size());
        
        // Return a dangling pointer as placeholder
        Ok(NonNull::dangling())
    }

    fn deallocate(&self, _ptr: NonNull<u8>, layout: Layout) {
        let mut inner = self.inner.borrow_mut();
        inner.stats.record_deallocation(layout.size());
    }

    fn stats(&self) -> AllocationStats {
        self.inner.borrow().stats
    }

    fn reset(&self) -> RuntimeResult<()> {
        let mut inner = self.inner.borrow_mut();
        inner.current_region = 0;
        inner.region_offset = 0;
        inner.stats = AllocationStats::new();
        Ok(())
    }
}

unsafe impl Send for RegionAllocator {}
unsafe impl Sync for RegionAllocator {} 