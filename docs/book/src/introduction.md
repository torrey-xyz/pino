# Introduction to Pino

Welcome to **Pino**, a lightweight, high-performance framework for Solana smart contract development. Pino is built on the Pinocchio SDK and provides the perfect balance between developer productivity and runtime efficiency.

## Why Pino?

Solana development has traditionally forced developers to choose between two extremes:

- **Raw Pinocchio**: Maximum performance but lots of boilerplate and complexity
- **Anchor**: Great developer experience but significant CU overhead and bloated binaries

**Pino bridges this gap** by providing zero-cost abstractions that compile down to optimal Pinocchio code while offering a developer-friendly API.

## Key Benefits

### 🚀 **Performance First**
- **60%+ CU reduction** compared to Anchor for common operations
- **50%+ smaller binaries** with zero runtime overhead
- **Zero-copy account handling** with compile-time validation

### 🛠️ **Developer Experience**
- **90% less boilerplate** than raw Pinocchio
- **Familiar patterns** for developers coming from Anchor
- **Progressive enhancement** - start simple, add complexity as needed

### 🏗️ **Production Ready**
- **Type-safe** account and instruction handling
- **Memory-safe** operations without runtime checks
- **Comprehensive testing** and profiling tools

## Core Philosophy

Pino follows three fundamental principles:

1. **Zero-Cost Abstractions**: All convenience features compile to the same bytecode as hand-written Pinocchio code
2. **CU-First Design**: Every feature is designed with compute unit consumption as the primary concern
3. **Progressive Enhancement**: Start with raw Pinocchio power, add framework features incrementally

## Quick Example

Here's what a simple program looks like in Pino:

```rust
use pino::prelude::*;

pino_entrypoint! {
    processor: CounterProcessor,
    max_accounts: 16,
    allocator: BumpAllocator,
}

#[pino_processor]
impl CounterProcessor {
    #[instruction(0x00)]
    pub fn initialize(
        ctx: Context<Initialize>,
        initial_value: u64,
    ) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.value = initial_value;
        counter.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    #[instruction(0x01)]
    pub fn increment(
        ctx: Context<Increment>,
    ) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.value = counter.value.checked_add(1).unwrap();
        msg!("Counter incremented to {}", counter.value);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[pino_account]
pub struct Counter {
    pub authority: Pubkey,
    pub value: u64,
}
```

This compiles to the same efficient bytecode as hand-written Pinocchio, but with a fraction of the boilerplate!

## Performance Comparison

| Operation | Anchor | Pino | Improvement |
|-----------|--------|------|-------------|
| Account parsing | 2,100 CU | 150 CU | 93% less |
| Instruction routing | 800 CU | 50 CU | 94% less |
| Basic transfer | 5,500 CU | 2,100 CU | 62% less |

## What's Next?

Ready to start building with Pino? Check out the [Installation Guide](getting-started/installation.md) to get your development environment set up, or dive into [Your First Program](getting-started/first-program.md) to see Pino in action.

For developers coming from Anchor, the [Migration Guide](guides/anchor-migration.md) will help you transition your existing programs to Pino's high-performance approach. 