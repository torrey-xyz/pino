#!/bin/bash

# Pino Framework Development Setup Script
# This script sets up the development environment for the Pino framework

set -e

echo "🔧 Setting up Pino Framework development environment..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first: https://rustup.rs/"
    exit 1
fi

echo "✅ Rust found: $(rustc --version)"

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "📦 Installing Solana CLI..."
    sh -c "$(curl -sSfL https://release.solana.com/v1.18.8/install)"
    export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
else
    echo "✅ Solana CLI found: $(solana --version)"
fi

# Install required Rust components
echo "📦 Installing Rust components..."
rustup component add rustfmt clippy
rustup target add bpf-unknown-unknown

# Install development tools
echo "📦 Installing development tools..."

# Install mdBook for documentation
if ! command -v mdbook &> /dev/null; then
    cargo install mdbook
fi

# Install cargo-expand for macro debugging
if ! command -v cargo-expand &> /dev/null; then
    cargo install cargo-expand
fi

# Install cargo-udeps for unused dependency detection
if ! command -v cargo-udeps &> /dev/null; then
    cargo install cargo-udeps
fi

# Install cargo-audit for security auditing
if ! command -v cargo-audit &> /dev/null; then
    cargo install cargo-audit
fi

# Install cargo-deny for license and dependency checking
if ! command -v cargo-deny &> /dev/null; then
    cargo install cargo-deny
fi

# Create .env file for development
if [ ! -f ".env" ]; then
    echo "📝 Creating .env file..."
    cat > .env << EOF
# Pino Framework Development Environment
RUST_LOG=debug
RUST_BACKTRACE=1

# Solana configuration
SOLANA_URL=https://api.devnet.solana.com
SOLANA_KEYPAIR_PATH=~/.config/solana/id.json

# Development flags
PINO_DEV_MODE=true
PINO_PROFILING=true
EOF
fi

# Initialize git hooks if .git exists
if [ -d ".git" ]; then
    echo "🔧 Setting up git hooks..."
    mkdir -p .git/hooks
    
    # Pre-commit hook
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Run tests and formatting before commit
set -e

echo "Running pre-commit checks..."

# Format code
cargo fmt --all -- --check
if [ $? -ne 0 ]; then
    echo "❌ Code formatting check failed. Run 'cargo fmt' to fix."
    exit 1
fi

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Clippy check failed. Fix the warnings above."
    exit 1
fi

# Run tests
cargo test --all
if [ $? -ne 0 ]; then
    echo "❌ Tests failed."
    exit 1
fi

echo "✅ Pre-commit checks passed!"
EOF
    
    chmod +x .git/hooks/pre-commit
fi

# Update Pinocchio submodule if it exists
if [ -d "pinocchio" ]; then
    echo "📦 Updating Pinocchio submodule..."
    git submodule update --init --recursive
fi

echo ""
echo "🎉 Development environment setup complete!"
echo ""
echo "Next steps:"
echo "  1. Run 'cargo test' to verify everything works"
echo "  2. Run 'mdbook serve docs/book' to view documentation"
echo "  3. Start developing with 'cargo run --bin pino-cli'"
echo ""
echo "Happy coding! 🦀" 