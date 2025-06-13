# Pino Framework Benchmarks

This directory contains performance benchmarks for the Pino framework, designed to validate the performance targets outlined in the framework design.

## Performance Targets

Based on the Pino framework design document, we aim for:

- **60%+ CU reduction** vs Anchor for common operations
- **50%+ binary size reduction** 
- **Zero runtime allocation overhead**

## Benchmark Categories

### Compute Unit (CU) Consumption (`cu-consumption/`)
Benchmarks that measure compute unit usage for common operations:

- Account deserialization
- Instruction routing  
- CPI preparation
- Basic transfers
- Complex state updates

### Memory Usage (`memory-usage/`)
Benchmarks that track memory allocation patterns:

- Stack frame sizes
- Account wrapper overhead
- Instruction data copying
- Allocator efficiency

### Binary Size (`binary-size/`)
Benchmarks that measure compiled program sizes:

- Hello World program size
- Token program size  
- DEX program size
- Framework overhead

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark category
cargo bench --bench cu-consumption
cargo bench --bench memory-usage
cargo bench --bench binary-size

# Compare with Anchor (requires Anchor examples)
cargo bench --bench anchor-comparison
```

## Benchmark Results Format

All benchmarks output results in a standardized format for tracking performance over time:

```
Operation: account_deserialization
Framework: Pino
CU Used: 150
Memory: 0 bytes allocated
Binary Size: N/A

Operation: account_deserialization  
Framework: Anchor
CU Used: 2100
Memory: 128 bytes allocated
Binary Size: N/A

Improvement: 93% CU reduction, 100% memory reduction
```

## Continuous Integration

These benchmarks run automatically in CI/CD to:

1. **Prevent regressions** - Fail builds if performance decreases
2. **Track improvements** - Report performance gains over time
3. **Compare frameworks** - Validate claims against Anchor

## Adding New Benchmarks

When adding new benchmarks:

1. Place them in the appropriate category directory
2. Follow the standardized output format
3. Include comparison with Anchor if applicable
4. Add documentation explaining what is being measured
5. Set appropriate performance thresholds

## Performance Tracking

Benchmark results are tracked over time to ensure we meet our targets:

- Compute unit usage trends
- Memory allocation patterns
- Binary size growth
- Framework overhead analysis 