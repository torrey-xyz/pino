//! Compute Unit tracking for Pino runtime.

use crate::{RuntimeError, RuntimeResult};
use core::cell::RefCell;

/// CU tracking statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct CuStats {
    /// Total CU consumed
    pub total_consumed: u64,
    /// CU consumed by allocations
    pub allocation_cu: u64,
    /// CU consumed by instructions  
    pub instruction_cu: u64,
    /// CU consumed by system calls
    pub syscall_cu: u64,
    /// Peak CU usage
    pub peak_usage: u64,
}

impl CuStats {
    /// Create new empty stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Add CU consumption for allocation
    pub fn add_allocation_cu(&mut self, cu: u64) {
        self.allocation_cu = self.allocation_cu.saturating_add(cu);
        self.total_consumed = self.total_consumed.saturating_add(cu);
        if self.total_consumed > self.peak_usage {
            self.peak_usage = self.total_consumed;
        }
    }

    /// Add CU consumption for instruction
    pub fn add_instruction_cu(&mut self, cu: u64) {
        self.instruction_cu = self.instruction_cu.saturating_add(cu);
        self.total_consumed = self.total_consumed.saturating_add(cu);
        if self.total_consumed > self.peak_usage {
            self.peak_usage = self.total_consumed;
        }
    }

    /// Add CU consumption for syscall
    pub fn add_syscall_cu(&mut self, cu: u64) {
        self.syscall_cu = self.syscall_cu.saturating_add(cu);
        self.total_consumed = self.total_consumed.saturating_add(cu);
        if self.total_consumed > self.peak_usage {
            self.peak_usage = self.total_consumed;
        }
    }

    /// Reset all stats
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// CU tracker for profiling performance
pub struct CuTracker {
    stats: RefCell<CuStats>,
    enabled: bool,
}

impl CuTracker {
    /// Create a new CU tracker
    pub fn new() -> Self {
        Self {
            stats: RefCell::new(CuStats::new()),
            enabled: true,
        }
    }

    /// Create a disabled CU tracker (for production)
    pub fn disabled() -> Self {
        Self {
            stats: RefCell::new(CuStats::new()),
            enabled: false,
        }
    }

    /// Enable tracking
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable tracking
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if tracking is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Track allocation CU
    pub fn track_allocation(&self, cu: u64) {
        if self.enabled {
            self.stats.borrow_mut().add_allocation_cu(cu);
        }
    }

    /// Track instruction CU
    pub fn track_instruction(&self, cu: u64) {
        if self.enabled {
            self.stats.borrow_mut().add_instruction_cu(cu);
        }
    }

    /// Track syscall CU
    pub fn track_syscall(&self, cu: u64) {
        if self.enabled {
            self.stats.borrow_mut().add_syscall_cu(cu);
        }
    }

    /// Get current stats
    pub fn stats(&self) -> CuStats {
        self.stats.borrow().clone()
    }

    /// Reset stats
    pub fn reset(&self) {
        if self.enabled {
            self.stats.borrow_mut().reset();
        }
    }

    /// Get a profiling checkpoint
    pub fn checkpoint(&self) -> CuCheckpoint {
        CuCheckpoint {
            total_consumed: self.stats.borrow().total_consumed,
        }
    }

    /// Calculate CU consumed since checkpoint
    pub fn since_checkpoint(&self, checkpoint: &CuCheckpoint) -> u64 {
        let current = self.stats.borrow().total_consumed;
        current.saturating_sub(checkpoint.total_consumed)
    }
}

impl Default for CuTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// A checkpoint for measuring CU consumption
#[derive(Debug, Clone, Copy)]
pub struct CuCheckpoint {
    total_consumed: u64,
}

/// Macro for tracking CU consumption of a block
#[cfg(feature = "profiling")]
#[macro_export]
macro_rules! track_cu {
    ($tracker:expr, $category:ident, $block:block) => {{
        let checkpoint = $tracker.checkpoint();
        let result = $block;
        let cu_consumed = $tracker.since_checkpoint(&checkpoint);
        match stringify!($category) {
            "allocation" => $tracker.track_allocation(cu_consumed),
            "instruction" => $tracker.track_instruction(cu_consumed),
            "syscall" => $tracker.track_syscall(cu_consumed),
            _ => {},
        }
        result
    }};
}

#[cfg(not(feature = "profiling"))]
#[macro_export]
macro_rules! track_cu {
    ($tracker:expr, $category:ident, $block:block) => {
        $block
    };
} 