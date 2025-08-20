# Makefile for common project tasks

# Enforce that Make considers targets based on their order, not on which one is first alphabetically
.PHONY: all fmt fmt-check clippy test-native test-wasm test-all doc check-all coverage clean

# Default target
all: build

# Build the project
build:
	@echo "Building project..."
	@cargo build

# Build the project for release
build-release:
	@echo "Building project for release..."
	@trunk build --release

# Format the code
fmt:
	@echo "Formatting code..."
	@cargo fmt --all

# Check code formatting
fmt-check:
	@echo "Checking code formatting..."
	@cargo fmt --all -- --check

# Lint the code with Clippy
clippy:
	@echo "Linting with Clippy..."
	@cargo clippy --all-targets --all-features --workspace -- -D warnings

# Run native Rust tests
test-native:
	@echo "Running native Rust tests..."
	@cargo test --all-features --workspace

# Run Wasm browser tests
test-wasm:
	@echo "Running Wasm browser tests..."
	@wasm-pack test --headless --firefox

# Run all tests
test-all: test-native test-wasm
	@echo "All tests completed."

# Generate documentation
doc:
	@echo "Generating documentation..."
	@RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items --all-features --workspace --examples

# Run all required checks (as per AGENTS.md)
check-all: fmt-check clippy test-native doc
	@echo "All checks passed."

# Generate code coverage report
coverage:
	@echo "Generating code coverage report..."
	@cargo tarpaulin --ignore-tests --out Html

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
