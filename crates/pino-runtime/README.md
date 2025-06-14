# Pino Runtime

Runtime components and allocators for the Pino Solana framework.

## Features

- **Memory Allocators**: Multiple allocation strategies optimized for Solana programs
  - Bump Allocator: Fast linear allocation
  - Pool Allocator: Fixed-size block allocation
  - Region Allocator: Region-based allocation
  - Stack Allocator: LIFO stack allocation

- **Execution Context**: Program context management and validation

- **CU Profiling**: Compute Unit tracking and performance analysis

## Usage

```rust
use pino_runtime::prelude::*;

// Initialize runtime
pino_runtime::init()?;

// Use allocators
let allocator = BumpAllocator::new(8192)?;

// Track CU consumption
let tracker = CuTracker::new();
tracker.track_allocation(100);

// Get statistics
let stats = tracker.stats();
println!("Total CU consumed: {}", stats.total_consumed);
```

## Phase 1 Implementation

This Phase 1 implementation includes:

- ✅ Basic allocator traits and implementations
- ✅ Memory allocation statistics
- ✅ Program context management
- ✅ CU tracking utilities
- ✅ Runtime initialization

Future phases will add:
- Advanced memory management
- Real memory allocation (currently uses placeholders)
- Integration with Solana runtime
- Performance optimizations 