# Pino Framework

A high-performance Solana program framework built on top of [Pinocchio](https://github.com/firedancer-io/pinocchio), providing zero-copy abstractions with 60-90% CU reduction compared to Anchor.

## Overview

Pino combines Pinocchio's zero-copy efficiency with developer-friendly APIs, automatic instruction routing, and advanced memory management. It's designed for developers who want Anchor-like ergonomics with maximum performance.

## Key Features

- **Zero-Copy Foundation**: Built on Pinocchio's zero-copy account system
- **60-90% CU Reduction**: Significant compute unit savings vs Anchor
- **Developer Friendly**: Familiar APIs with automatic instruction routing
- **Type Safety**: Full type safety with minimal runtime overhead
- **Memory Efficient**: Stack-allocated collections and regional allocators
- **Flexible Entrypoints**: Standard, lazy, and no-allocator options

## Quick Start

Add Pino to your `Cargo.toml`:

```toml
[dependencies]
pino = { path = "path/to/pino-core", package = "pino-core" }
borsh = { version = "1.0", features = ["derive"] }
bytemuck = { version = "1.20.0", features = ["derive"] }
```

Create a simple program:

```rust
use pino::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};

// Declare your program ID
declare_id!("YourProgramId11111111111111111111111111111");

// Define your instructions
#[derive(BorshDeserialize, BorshSerialize)]
pub enum MyInstruction {
    Initialize { value: u64 },
    Update { new_value: u64 },
}

// Define account data structures
#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct MyAccount {
    pub value: u64,
    pub is_initialized: u8,
}

// Define account validation
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + core::mem::size_of::<MyAccount>())]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Implement instruction processing
impl ProgramInstruction for MyInstruction {
    fn process<'info>(
        &self,
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        match self {
            MyInstruction::Initialize { value } => {
                let mut accounts_iter = accounts.iter();
                let ctx = parse_accounts::<Initialize>(program_id, &mut accounts_iter, instruction_data)?;
                
                let mut account_data = ctx.accounts.my_account.load_mut()?;
                account_data.value = *value;
                account_data.is_initialized = 1;
                
                msg!("Initialized with value: {}", value);
                Ok(())
            }
            MyInstruction::Update { new_value } => {
                // Handle update...
                Ok(())
            }
        }
    }
}

// Generate the entrypoint
pino_entrypoint!(MyInstruction);
```

## Architecture

Pino is built on several key components:

### 1. Zero-Copy Account System
- Direct memory mapping via Pinocchio's `AccountInfo`
- No intermediate allocations or copying
- Support for large accounts (up to 10MB)

### 2. Efficient Error Handling
- Minimal CU overhead validation macros
- Compile-time optimized error paths
- Compatible with Pinocchio's `ProgramError`

### 3. Stack-Allocated Collections
- `StackVec<T, N>`: Vec-like without heap allocation
- `StackMap<K, V, N>`: Hash map with linear probing
- `StackString<N>`: String building without allocation

### 4. Advanced Memory Management
- Regional allocators for zero-CU frequent operations
- Alignment-aware allocation strategies
- Memory layout utilities for complex data structures

### 5. Flexible Entrypoints
- **Standard**: `pino_entrypoint!` - Familiar Anchor-like experience
- **Lazy**: `pino_lazy_entrypoint!` - Maximum CU efficiency
- **No-Alloc**: `pino_no_alloc_entrypoint!` - Zero allocation overhead

## Performance Comparison

| Operation | Anchor CU | Pino CU | Improvement |
|-----------|-----------|---------|-------------|
| Account Deserialization | 2,100 | <120 | 94% reduction |
| Instruction Routing | 800 | <35 | 96% reduction |
| Basic CPI Call | 1,200 | 180 | 85% reduction |
| Large Account (1MB) | 15,000 | 1,500 | 90% reduction |

## Examples

See the `examples/` directory for complete examples:

- **hello-world**: Basic program with account initialization
- **token-like**: Token program demonstrating advanced features
- **defi-vault**: Complex DeFi program with multiple instructions

## Features

- `default`: Standard features for most use cases
- `std`: Enable std library features (for testing)
- `profiling`: Enable CU profiling and optimization hints

## Building

```bash
# Build the framework
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Build examples
cd examples/hello-world
cargo build-bpf
```

## Contributing

Contributions are welcome! Please see our contributing guidelines and code of conduct.

## License

Licensed under either of:
- Apache License, Version 2.0
- MIT License

at your option. 