# Makefile for CommentsCleaner

.PHONY: help build test clean release install format clippy audit

# Default target
help:
	@echo "Available targets:"
	@echo "  build     - Build the project in release mode"
	@echo "  test      - Run all tests"
	@echo "  clean     - Clean build artifacts"
	@echo "  format    - Format the code"
	@echo "  clippy    - Run clippy lints"
	@echo "  audit     - Run security audit"
	@echo "  release   - Create a new release (requires VERSION)"
	@echo "  install   - Install the binary locally"
	@echo ""
	@echo "Examples:"
	@echo "  make release VERSION=0.1.0"
	@echo "  make build"

# Build the project
build:
	cargo build --release

# Run tests
test:
	cargo test --verbose

# Clean build artifacts
clean:
	cargo clean

# Format code
format:
	cargo fmt --all

# Run clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Security audit
audit:
	cargo audit

# Install locally
install:
	cargo install --path .

# Create release
release:
	@if [ -z "$(VERSION)" ]; then \
		echo "Error: VERSION is required. Usage: make release VERSION=0.1.0"; \
		exit 1; \
	fi
	./release.sh -v $(VERSION)

# Dry run release
release-dry:
	@if [ -z "$(VERSION)" ]; then \
		echo "Error: VERSION is required. Usage: make release-dry VERSION=0.1.0"; \
		exit 1; \
	fi
	./release.sh -v $(VERSION) --dry-run

# Build for all targets
build-all:
	@echo "Building for all supported targets..."
	rustup target add x86_64-unknown-linux-gnu
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-apple-darwin
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-apple-darwin

# Run CI checks
ci: format clippy test
	@echo "All CI checks passed!"

# Show project info
info:
	@echo "Project: CommentsCleaner"
	@echo "Version: $$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')"
	@echo "Rust toolchain: $$(rustc --version)"
	@echo "Targets: x86_64-unknown-linux-gnu, x86_64-pc-windows-gnu, x86_64-apple-darwin"
