.DEFAULT_GOAL := all

# Paths and settings
SCHEMA_PATH := ./deriv-api-docs/config/v3
SCHEMA_OUTPUT_PATH := ./schema/src
API_DOCS_REPO := https://github.com/deriv-com/deriv-api-docs.git

# Commands
CARGO := cargo
CARGO_BUILD := $(CARGO) build
CARGO_TEST := $(CARGO) test
CARGO_CHECK := $(CARGO) check
CARGO_CLIPPY := $(CARGO) clippy
CARGO_FMT := $(CARGO) fmt
CARGO_DOC := $(CARGO) doc

# Environment settings
RUST_LOG_LEVEL := debug
RUST_ENV := RUST_LOG=$(RUST_LOG_LEVEL) RUST_BACKTRACE=1 DERIV_API_GENERATE=1

# Generator commands
CARGO_GEN := $(RUST_ENV) $(CARGO) run --features generate
SCHEMA_GEN := $(CARGO_GEN) --bin schema_generator
CALLS_GEN := $(CARGO_GEN) --bin generate_calls

# Export environment variables
export SCHEMA_PATH
export SCHEMA_OUTPUT_PATH
export RUST_LOG_LEVEL

# Main targets
.PHONY: all
all: clean setup generate build test

.PHONY: dev
dev: clean setup generate build

# Setup and generation
.PHONY: setup
setup:
	@echo "=== Creating directory structure ==="
	@mkdir -p src/api src/types schema/src
	@touch src/api/calls.rs src/api/subscription_calls.rs
	@touch src/types/mod.rs
	@touch schema/src/lib.rs
	@rm -rf schema/schema

.PHONY: format-generated
format-generated:
	@echo "=== Formatting generated code ==="
	$(CARGO_FMT) --all
	$(CARGO_CLIPPY) --workspace --all-features -- -D warnings --allow=clippy::all
	@echo "=== Generated code formatting complete ==="

.PHONY: cleanup-generated
cleanup-generated:
	@echo "=== Cleaning up generated code ==="
	# Remove any backup files
	find . -name "*.rs.bk" -type f -delete
	# Remove any empty files
	find . -name "*.rs" -type f -empty -delete
	@echo "=== Code cleanup complete ==="

.PHONY: generate
generate: clone patch-schema generate-schema generate-calls build-schema format-generated cleanup-generated

.PHONY: clone
clone:
	@echo "=== Cloning API documentation ==="
	@if [ ! -d "deriv-api-docs" ]; then \
		git clone $(API_DOCS_REPO); \
	else \
		echo "API docs already exist, updating..."; \
		cd deriv-api-docs && git pull; \
	fi

.PHONY: patch-schema
patch-schema:
	@echo "=== Applying schema patches ==="
	@if [ -f "schema.patch" ]; then \
		cd deriv-api-docs && \
		git checkout . && \
		git apply --reject --whitespace=fix ../schema.patch || true; \
	fi

.PHONY: generate-schema
generate-schema:
	@echo "=== Generating schema types ==="
	@$(SCHEMA_GEN)
	@echo "=== Schema generation complete ==="

.PHONY: generate-calls
generate-calls:
	@echo "=== Generating API calls ==="
	@$(CALLS_GEN)
	@echo "=== API calls generation complete ==="

.PHONY: build-schema
build-schema:
	@echo "=== Building schema package ==="
	$(CARGO_BUILD) -p deriv-api-schema

# Build and test targets
.PHONY: build
build:
	@echo "=== Building project ==="
	$(CARGO_BUILD) --workspace --all-features

.PHONY: test
test:
	@echo "=== Running tests ==="
	$(CARGO_TEST) --workspace --all-features

# Code quality targets
.PHONY: check
check:
	@echo "=== Running code checks ==="
	$(CARGO_CHECK) --workspace
	$(CARGO_CLIPPY) --workspace -- -D warnings
	$(CARGO_FMT) --all -- --check

.PHONY: format
format:
	@echo "=== Formatting code ==="
	$(CARGO_FMT) --all

.PHONY: lint
lint:
	@echo "=== Running linter ==="
	$(CARGO_CLIPPY) --workspace -- -D warnings

# Documentation
.PHONY: doc
doc:
	@echo "=== Generating documentation ==="
	$(CARGO_DOC) --workspace --no-deps --open

# Cleanup targets

.PHONY: clean
clean:
	@echo "=== Cleaning up ==="
	-$(CARGO) clean
	rm -rf deriv-api-docs
	rm -f src/api/calls.rs src/api/subscription_calls.rs
	rm -rf schema/src/*.rs
	rm -rf schema/schema  # Remove any accidentally created nested directory
	@mkdir -p schema/src
	@touch schema/src/lib.rs

.PHONY: clean-all
clean-all: clean
	@echo "=== Deep cleaning all artifacts ==="
	rm -rf target/
	rm -rf schema/target/
	rm -rf Cargo.lock
	rm -rf schema/Cargo.lock
	rm -rf examples/*/target/

# Development helpers
.PHONY: watch
watch:
	@echo "=== Starting watch mode ==="
	$(CARGO) watch -x check -x test

.PHONY: update
update:
	@echo "=== Updating dependencies ==="
	$(CARGO) update --workspace

# Examples
.PHONY: examples
examples:
	@echo "=== Building examples ==="
	@for example in examples/*; do \
		if [ -d "$$example" ]; then \
			echo "Building $$example..."; \
			(cd "$$example" && $(CARGO_BUILD)); \
		fi \
	done

# Verbose builds
.PHONY: verbose
verbose:
	@echo "=== Running with verbose output ==="
	RUST_LOG=trace $(MAKE) all

.PHONY: verbose-schema
verbose-schema:
	@echo "=== Generating schema with verbose output ==="
	RUST_LOG=trace $(MAKE) generate-schema

.PHONY: verbose-calls
verbose-calls:
	@echo "=== Generating calls with verbose output ==="
	RUST_LOG=trace $(MAKE) generate-calls

# CI/CD helpers
.PHONY: ci
ci: format check test build

# Coverage report
.PHONY: coverage
coverage:
	@echo "=== Generating coverage report ==="
	$(CARGO) tarpaulin --workspace --out html

# Tool installation
.PHONY: install-tools
install-tools:
	@echo "=== Installing development tools ==="
	$(CARGO) install cargo-watch
	$(CARGO) install cargo-edit
	$(CARGO) install cargo-tarpaulin
	$(CARGO) install cargo-insta

# Error checking
.PHONY: check-errors
check-errors:
	@echo "=== Checking for common errors ==="
	@find . -name "*.rs" -type f -exec grep -l "panic!" {} \; || true
	@find . -name "*.rs" -type f -exec grep -l "unwrap()" {} \; || true
	@find . -name "*.rs" -type f -exec grep -l "expect(" {} \; || true

# Help
.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make all              - Clean, setup, generate, build and test"
	@echo "  make dev              - Clean, setup, generate and build (no tests)"
	@echo "  make setup            - Create directory structure"
	@echo "  make generate         - Generate all code from schema"
	@echo "  make build            - Build the project"
	@echo "  make test             - Run tests"
	@echo "  make check            - Run all checks"
	@echo "  make format           - Format code"
	@echo "  make lint             - Run linter"
	@echo "  make clean            - Clean build artifacts"
	@echo "  make clean-all        - Clean everything including all target directories"
	@echo "  make doc              - Generate documentation"
	@echo "  make watch            - Watch for changes"
	@echo "  make update           - Update dependencies"
	@echo "  make examples         - Build examples"
	@echo "  make verbose          - Run all with verbose output"
	@echo "  make verbose-schema   - Run schema generation with verbose output"
	@echo "  make verbose-calls    - Run calls generation with verbose output"
	@echo "  make install-tools    - Install development tools"
	@echo "  make coverage         - Generate coverage report"
	@echo "  make ci               - Run CI checks"
	@echo "  make check-errors     - Check for common coding errors"

# Default target
.DEFAULT:
	@echo "No target specified. Run 'make help' for available targets."
