# Test Harness Documentation

This document explains the testing approach for `dure`, a distributed e-commerce platform.

## Overview

The test harness provides:

1. **Unit Tests** - Component-level testing for business logic
2. **Integration Tests** - Testing interactions between modules
3. **Platform Tests** - Platform-specific testing (Desktop, Android, WASM)
4. **E2E Tests** - End-to-end workflow testing

## Quick Start

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Run specific test
cargo test test_product_creation

# Run integration tests only
cargo test --test integration_*

# Run tests for specific platform
cargo test --target x86_64-unknown-linux-gnu
```

## Test Structure

### Unit Tests

Located in the same files as the code they test, using `#[cfg(test)]`:

```rust
// mobile/src/calc/product.rs

#[cfg(test)]
mod tests {
    use super::*

;

    #[test]
    fn test_product_validation() {
        // Test product validation logic
    }

    #[test]
    fn test_product_price_calculation() {
        // Test price calculation
    }
}
```

### Integration Tests

Located in `mobile/tests/`:

```
mobile/tests/
├── integration_product.rs     # Product management tests
├── integration_order.rs        # Order processing tests
├── integration_hosting.rs      # Hosting setup tests
├── integration_payment.rs      # Payment gateway tests
└── integration_messaging.rs    # WebSocket messaging tests
```

### Platform-Specific Tests

```rust
// Desktop-only test
#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
#[test]
fn test_desktop_tray_icon() {
    // Test system tray functionality
}

// Android-only test
#[cfg(target_os = "android")]
#[test]
fn test_android_permissions() {
    // Test Android permissions
}

// WASM-only test
#[cfg(target_arch = "wasm32")]
#[test]
fn test_wasm_bindings() {
    // Test WASM bindings
}
```

## Test Categories

### Product Management Tests

```bash
# Run product-related tests
cargo test product

# Examples:
# - Product creation/validation
# - Product listing/filtering
# - Product modification
# - Stock management
```

### Order Processing Tests

```bash
# Run order-related tests
cargo test order

# Examples:
# - Order creation
# - Order status updates
# - Payment processing
# - Order fulfillment
```

### Hosting Configuration Tests

```bash
# Run hosting-related tests
cargo test hosting

# Examples:
# - DNS configuration
# - Web hosting setup
# - Database connectivity
# - SSL/TLS certificates
```

### Authentication Tests

```bash
# Run auth-related tests
cargo test auth

# Examples:
# - Key generation
# - Identity management
# - Session handling
# - Firebase/Supabase integration
```

### WebSocket Tests

```bash
# Run WebSocket tests
cargo test websocket

# Examples:
# - Connection establishment
# - Message sending/receiving
# - Authentication flow
# - Error handling
```

## Running Tests

### Basic Test Execution

```bash
# All tests
cargo test

# Specific test file
cargo test --test integration_product

# Specific test function
cargo test test_product_creation

# With output
cargo test -- --nocapture

# With logging
RUST_LOG=debug cargo test -- --nocapture
```

### Platform-Specific Testing

```bash
# Desktop (Linux)
cargo test --target x86_64-unknown-linux-gnu

# Desktop (macOS)
cargo test --target aarch64-apple-darwin

# Desktop (Windows)
cargo test --target x86_64-pc-windows-msvc

# Android (requires emulator or device)
cd mobile
./test.sh  # If test script exists

# WASM
cargo test --target wasm32-unknown-unknown
```

### Performance Testing

```bash
# Run benchmarks (if criterion is set up)
cargo bench

# Profile tests
cargo test --release
perf record cargo test --release
perf report
```

## Test Writing Guidelines

### Test Structure

```rust
#[test]
fn test_name() {
    // 1. Arrange - Set up test data
    let product = Product::new("Test Product", 19.99);

    // 2. Act - Perform the action
    let result = validate_product(&product);

    // 3. Assert - Verify the outcome
    assert!(result.is_ok());
}
```

### Async Tests

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### Error Testing

```rust
#[test]
fn test_validation_error() {
    let invalid_product = Product::new("", -10.0);
    let result = validate_product(&invalid_product);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ValidationError::InvalidPrice);
}
```

### Database Testing

```rust
#[test]
fn test_database_operation() {
    // Use temporary database
    let temp_db = create_temp_database();
    
    // Perform operation
    let result = store_product(&temp_db, product);
    
    // Verify
    assert!(result.is_ok());
    
    // Cleanup happens automatically when temp_db drops
}
```

## Mock Data

Create test fixtures for consistent testing:

```rust
// mobile/tests/fixtures.rs

pub fn sample_product() -> Product {
    Product {
        id: "prod-test-001".to_string(),
        name: "Test Product".to_string(),
        category: "Electronics".to_string(),
        price: 99.99,
        stock: 50,
        status: ProductStatus::Available,
    }
}

pub fn sample_order() -> Order {
    Order {
        id: "order-test-001".to_string(),
        products: vec!["prod-test-001".to_string()],
        quantities: vec![2],
        status: OrderStatus::Pending,
        total: 199.98,
    }
}
```

## CI Integration

### GitHub Actions

The project should have `.github/workflows/ci.yml` with:

```yaml
name: CI

on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]

jobs:
  test:
    name: Test Suite
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache cargo
        uses: Swatinem/rust-cache@v2
      
      - name: Run tests
        run: cargo test --all-features
      
      - name: Run clippy
        run: cargo clippy --all-targets -- -D warnings
```

### Local Pre-push Testing

```bash
# Create a script: scripts/pre-push.sh
#!/bin/bash
set -e

echo "Running tests..."
cargo test

echo "Running clippy..."
cargo clippy --all-targets -- -D warnings

echo "Checking formatting..."
cargo fmt --check

echo "All checks passed!"
```

## Coverage

### Generate Coverage Report

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Generate coverage
cargo llvm-cov --html

# Open report
open target/llvm-cov/html/index.html
```

### Coverage Goals

- **Unit tests**: Aim for 80%+ coverage
- **Integration tests**: Cover critical workflows
- **Platform tests**: Ensure all platforms work

## Troubleshooting

### Tests Hang or Timeout

```bash
# Run with explicit timeout
timeout 120 cargo test

# Run serially to avoid race conditions
cargo test -- --test-threads=1
```

### Database Lock Issues

```bash
# Clean up test databases
rm -rf /tmp/dure-test-*

# Use unique database per test
let db_path = format!("/tmp/dure-test-{}.db", uuid::Uuid::new_v4());
```

### Platform-Specific Test Failures

```bash
# Check platform
echo $RUST_TARGET

# Run with verbose output
cargo test -v -- --nocapture

# Check platform-specific code
cargo test --target <specific-target>
```

## Future Enhancements

Potential testing improvements:

- [ ] Add property-based testing with proptest
- [ ] Add fuzzing with cargo-fuzz
- [ ] Add E2E tests for complete workflows
- [ ] Add performance benchmarks
- [ ] Add load testing for WebSocket server
- [ ] Add UI testing with egui test framework
- [ ] Add Android instrumentation tests
- [ ] Add WASM integration tests

## See Also

- [GUIDELINES_RUST_CODING.md](GUIDELINES_RUST_CODING.md) - Rust coding standards
- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Quick command reference
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) - Common issues and solutions
