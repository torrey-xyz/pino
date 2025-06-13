# Pino Framework Tests

This directory contains comprehensive tests for the Pino framework, organized by test type and scope.

## Directory Structure

### Integration Tests (`integration/`)
- **`cli/`** - Tests for the Pino CLI tool commands
- **`macros/`** - Tests for procedural macros and code generation
- **`examples/`** - Tests that validate example programs work correctly

### Performance Tests (`performance/`)
- **`cu-benchmarks/`** - Compute unit consumption benchmarks
- **`memory-benchmarks/`** - Memory usage and allocation benchmarks

### Compatibility Tests (`compatibility/`)
- **`anchor-compat/`** - Tests for Anchor compatibility layer

## Running Tests

```bash
# Run all tests
cargo test

# Run integration tests only
cargo test --test integration

# Run performance benchmarks
cargo test --test performance --release

# Run compatibility tests
cargo test --test compatibility
```

## Test Guidelines

1. **Integration tests** should test real-world usage scenarios
2. **Performance tests** should validate CU and memory targets from the roadmap
3. **Compatibility tests** should ensure migration paths work correctly
4. All tests should be deterministic and run in CI/CD

## Performance Targets

Based on the framework design document:
- 60%+ CU reduction vs Anchor for common operations
- 50%+ binary size reduction
- Zero runtime allocation overhead 