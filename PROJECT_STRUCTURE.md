# Pino Project Structure

This document outlines the folder structure for the Pino framework project, designed to scale from MVP to a comprehensive Solana smart contract development framework.

## Root Directory Structure

```
pino/
├── Cargo.toml                 # Workspace configuration
├── PROJECT_STRUCTURE.md       # This file
├── README.md                  # Project overview
├── LICENSE                    # License file
├── .gitignore                 # Git ignore patterns
├── .github/                   # GitHub workflows and templates
│   ├── workflows/
│   │   ├── ci.yml
│   │   ├── release.yml
│   │   └── benchmarks.yml
│   └── ISSUE_TEMPLATE/
├── pinocchio/                 # Pinocchio submodule
├── crates/                    # All Pino framework crates
├── examples/                  # Example programs
├── docs/                      # Documentation
├── tools/                     # Development tools and scripts
├── tests/                     # Integration tests
└── benchmarks/                # Performance benchmarks
```

## Core Framework Crates (`crates/`)

### Phase 1: Core Foundation

#### `crates/pino-core/`
The main entry point and core abstractions.
```
pino-core/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Re-exports and main API
│   ├── prelude.rs          # Common imports for users
│   ├── entrypoint/         # Enhanced entrypoint system
│   │   ├── mod.rs
│   │   ├── router.rs       # Instruction routing
│   │   └── context.rs      # Execution context
│   ├── account/            # Account system
│   │   ├── mod.rs
│   │   ├── wrapper.rs      # Account<T> wrapper
│   │   ├── validation.rs   # Account validation
│   │   └── traits.rs       # Account traits
│   ├── instruction/        # Instruction handling
│   │   ├── mod.rs
│   │   ├── data.rs         # Instruction data parsing
│   │   └── processor.rs    # Instruction processor traits
│   ├── error/              # Error handling
│   │   ├── mod.rs
│   │   └── program.rs      # Program-specific errors
│   └── utils/              # Utility functions
│       ├── mod.rs
│       └── borsh.rs        # Borsh helpers
└── tests/
    ├── integration/
    └── unit/
```

#### `crates/pino-macros/`
Procedural macros for the framework.
```
pino-macros/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Macro exports
│   ├── entrypoint.rs       # pino_entrypoint! macro
│   ├── processor.rs        # #[pino_processor] macro
│   ├── accounts.rs         # #[derive(Accounts)] macro
│   ├── account.rs          # #[pino_account] macro
│   ├── data.rs             # #[derive(PinoData)] macro
│   ├── instruction.rs      # #[instruction] macro
│   ├── utils/              # Macro utilities
│   │   ├── mod.rs
│   │   ├── parsing.rs      # AST parsing helpers
│   │   └── codegen.rs      # Code generation helpers
│   └── tests/              # Macro expansion tests
└── tests/
    └── expand/             # Macro expansion tests
```

#### `crates/pino-runtime/`
Runtime components and allocators.
```
pino-runtime/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── allocator/          # Memory allocators
│   │   ├── mod.rs
│   │   ├── bump.rs         # Bump allocator
│   │   ├── region.rs       # Region allocator
│   │   ├── pool.rs         # Pool allocator
│   │   └── stack.rs        # Stack allocator
│   ├── context/            # Execution context
│   │   ├── mod.rs
│   │   └── program.rs      # Program context
│   └── profiler/           # CU profiling
│       ├── mod.rs
│       └── cu_tracker.rs   # CU consumption tracking
└── tests/
```

### Phase 2: Developer Tools

#### `crates/pino-cli/`
Command-line interface tool.
```
pino-cli/
├── Cargo.toml
├── src/
│   ├── main.rs             # CLI entry point
│   ├── lib.rs              # Library functions
│   ├── commands/           # CLI commands
│   │   ├── mod.rs
│   │   ├── new.rs          # pino new
│   │   ├── generate.rs     # pino generate
│   │   ├── build.rs        # pino build
│   │   ├── test.rs         # pino test
│   │   ├── dev.rs          # pino dev
│   │   ├── deploy.rs       # pino deploy
│   │   ├── analyze.rs      # pino analyze
│   │   └── migrate.rs      # pino migrate
│   ├── templates/          # Project templates
│   │   ├── basic/
│   │   ├── token/
│   │   ├── dex/
│   │   └── nft/
│   ├── config/             # Configuration handling
│   │   ├── mod.rs
│   │   └── pino_toml.rs    # Pino.toml parsing
│   └── utils/              # CLI utilities
│       ├── mod.rs
│       ├── fs.rs           # File system helpers
│       └── spinner.rs      # Progress indicators
├── templates/              # Template files
└── tests/
```

#### `crates/pino-codegen/`
Code generation utilities.
```
pino-codegen/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── instruction/        # Instruction generators
│   │   ├── mod.rs
│   │   └── handler.rs      # Generate instruction handlers
│   ├── account/            # Account generators
│   │   ├── mod.rs
│   │   └── state.rs        # Generate account structs
│   ├── program/            # Program generators
│   │   ├── mod.rs
│   │   └── scaffold.rs     # Program scaffolding
│   └── templates/          # Code templates
│       ├── mod.rs
│       ├── instruction.hbs
│       ├── account.hbs
│       └── program.hbs
└── tests/
```

#### `crates/pino-test/`
Testing framework.
```
pino-test/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── context/            # Test context
│   │   ├── mod.rs
│   │   ├── program.rs      # Program test context
│   │   └── account.rs      # Account mocking
│   ├── assertions/         # Test assertions
│   │   ├── mod.rs
│   │   ├── cu.rs           # CU assertions
│   │   └── account.rs      # Account state assertions
│   ├── macros/             # Test macros
│   │   ├── mod.rs
│   │   └── test_instruction.rs
│   └── utils/              # Test utilities
│       ├── mod.rs
│       └── fixtures.rs     # Test fixtures
└── tests/
```

### Phase 3: Framework Extensions

#### `crates/pino-std/`
Standard library extensions.
```
pino-std/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── collections/        # Efficient collections
│   │   ├── mod.rs
│   │   ├── stack_vec.rs    # StackVec<T, N>
│   │   ├── stack_map.rs    # StackMap<K, V, N>
│   │   └── stack_set.rs    # StackSet<T, N>
│   ├── serde/              # Serialization helpers
│   │   ├── mod.rs
│   │   ├── zero_copy.rs    # Zero-copy serialization
│   │   └── borsh.rs        # Borsh extensions
│   ├── math/               # Math utilities
│   │   ├── mod.rs
│   │   ├── decimal.rs      # Fixed-point decimal
│   │   └── checked.rs      # Checked arithmetic
│   └── string/             # String handling
│       ├── mod.rs
│       └── stack_string.rs # Stack-allocated strings
└── tests/
```

#### `crates/pino-memory/`
Advanced memory management.
```
pino-memory/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── allocator/          # Advanced allocators
│   │   ├── mod.rs
│   │   ├── region.rs       # Region-based allocation
│   │   ├── pool.rs         # Object pool allocation
│   │   └── slab.rs         # Slab allocation
│   ├── analysis/           # Memory analysis
│   │   ├── mod.rs
│   │   ├── usage.rs        # Usage tracking
│   │   └── fragmentation.rs
│   └── macros/             # Memory macros
│       ├── mod.rs
│       └── allocate_in.rs  # allocate_in! macro
└── tests/
```

#### `crates/pino-cpi/`
Cross-program invocation helpers.
```
pino-cpi/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── context/            # CPI context
│   │   ├── mod.rs
│   │   └── builder.rs      # Context builder
│   ├── programs/           # Program-specific CPI
│   │   ├── mod.rs
│   │   ├── system.rs       # System program CPI
│   │   └── token.rs        # Token program CPI
│   └── utils/              # CPI utilities
│       ├── mod.rs
│       └── accounts.rs     # Account preparation
└── tests/
```

#### `crates/pino-anchor-compat/`
Anchor compatibility layer.
```
pino-anchor-compat/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── macros/             # Anchor-compatible macros
│   │   ├── mod.rs
│   │   ├── program.rs      # #[program] macro
│   │   └── account.rs      # #[account] macro
│   ├── context/            # Context compatibility
│   │   ├── mod.rs
│   │   └── anchor.rs       # Anchor Context wrapper
│   └── migration/          # Migration helpers
│       ├── mod.rs
│       └── analyzer.rs     # Code analysis
└── tests/
```

### Phase 4: Integration Helpers

#### `crates/pino-spl/`
SPL program integrations.
```
pino-spl/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── token/              # SPL Token
│   │   ├── mod.rs
│   │   ├── instructions.rs
│   │   └── state.rs
│   ├── associated_token/   # Associated Token
│   │   ├── mod.rs
│   │   └── instructions.rs
│   └── memo/               # Memo program
│       ├── mod.rs
│       └── instructions.rs
└── tests/
```

#### `crates/pino-metaplex/`
Metaplex integrations.
```
pino-metaplex/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── token_metadata/     # Token metadata
│   │   ├── mod.rs
│   │   ├── instructions.rs
│   │   └── state.rs
│   └── candy_machine/      # Candy machine
│       ├── mod.rs
│       └── instructions.rs
└── tests/
```

## Examples (`examples/`)

```
examples/
├── hello-world/            # Basic program
│   ├── Cargo.toml
│   ├── src/lib.rs
│   └── tests/
├── token-program/          # SPL Token-like program
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── instruction.rs
│   │   └── state.rs
│   └── tests/
├── dex-program/            # DEX implementation
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── orderbook.rs
│   │   └── matching.rs
│   └── tests/
└── nft-marketplace/        # NFT marketplace
    ├── Cargo.toml
    ├── src/
    └── tests/
```

## Documentation (`docs/`)

```
docs/
├── book/                   # mdBook documentation
│   ├── src/
│   │   ├── SUMMARY.md
│   │   ├── introduction.md
│   │   ├── getting-started/
│   │   ├── guides/
│   │   ├── reference/
│   │   └── examples/
│   └── book.toml
├── api/                    # Generated API docs
└── tutorials/              # Tutorial content
```

## Development Tools (`tools/`)

```
tools/
├── scripts/                # Development scripts
│   ├── setup.sh
│   ├── test-all.sh
│   └── release.sh
├── benchmarks/             # Benchmarking tools
│   ├── cu-profiler/
│   └── memory-analyzer/
└── migration/              # Migration tools
    └── anchor-to-pino/
```

## Testing (`tests/`)

```
tests/
├── integration/            # Integration tests
│   ├── cli/
│   ├── macros/
│   └── examples/
├── performance/            # Performance tests
│   ├── cu-benchmarks/
│   └── memory-benchmarks/
└── compatibility/          # Compatibility tests
    └── anchor-compat/
```

## Development Phases

### Phase 1 (Weeks 1-4)
Start with core crates:
- `pino-core`
- `pino-macros` 
- `pino-runtime`
- `pino-cli` (basic functionality)

### Phase 2 (Weeks 5-8)
Add developer experience:
- `pino-codegen`
- `pino-test`
- Enhanced `pino-cli`

### Phase 3 (Weeks 9-12)
Framework extensions:
- `pino-std`
- `pino-memory`
- `pino-cpi`

### Phase 4 (Weeks 13-16)
Ecosystem integration:
- `pino-spl`
- `pino-metaplex`
- `pino-anchor-compat`

## Benefits of This Structure

1. **Modular**: Each crate has a single responsibility
2. **Scalable**: Easy to add new crates and features
3. **Testable**: Clear separation allows focused testing
4. **Publishable**: Individual crates can be published independently
5. **Maintainable**: Clear ownership and boundaries
6. **Flexible**: Users can depend on only what they need

This structure supports the progressive enhancement philosophy of Pino while maintaining clear boundaries and allowing for independent development and testing of each component. 