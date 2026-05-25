# Run all tests with default features
test:
    cargo test

# Run all tests with all features enabled
test-all:
    cargo test --all-features

# Run clippy for all features
lint:
    cargo clippy --all-features -- -D warnings

# Check compilation for all features
check:
    cargo check --all-features

# Format the code
fmt *args:
    cargo fmt --all {{args}}
