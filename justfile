# Run all tests with default features
test:
    cargo test

# Run all tests with all features enabled
test-all:
    cargo test --all-features

# Run tests using cross for a specific target (uses target/cross to isolate build artifacts)
test-cross target *args:
    CARGO_TARGET_DIR=target/cross cross test --target {{target}} {{args}}

# Run cross tests for both Thumb-1 and Thumb-2 configurations on ARMv7 Linux
test-cross-thumb *args:
    CROSS_FORCE_THUMB1=1 CARGO_TARGET_DIR=target/cross cross test --target armv7-unknown-linux-gnueabihf {{args}}
    CROSS_FORCE_THUMB2=1 CARGO_TARGET_DIR=target/cross cross test --target armv7-unknown-linux-gnueabihf {{args}}

# Run clippy for all features
lint:
    cargo clippy --all-features -- -D warnings

# Check compilation for all features
check:
    cargo check --all-features

# Check compilation for both RISC-V variants (standard RV32 and embedded RV32E)
check-riscv:
    FORCE_RISCV32_STD=1 cargo check --lib --target riscv32imac-unknown-none-elf
    FORCE_RISCV32_E=1 cargo check --lib --target riscv32imac-unknown-none-elf

# Run bare-metal RISC-V tests on QEMU for both standard RV32 and embedded RV32E configurations
test-riscv:
    @echo "Building and running Standard RV32 tests in QEMU..."
    RUSTFLAGS="-C link-arg=-Tqemu-tests/riscv32-virt.ld -C panic=abort" FORCE_RISCV32_STD=1 cargo build --bin riscv_test --target riscv32imac-unknown-none-elf
    qemu-system-riscv32 -machine virt -bios none -nographic -kernel target/riscv32imac-unknown-none-elf/debug/riscv_test
    @echo "Building and running Embedded RV32E tests in QEMU..."
    RUSTFLAGS="-C link-arg=-Tqemu-tests/riscv32-virt.ld -C panic=abort" FORCE_RISCV32_E=1 cargo build --bin riscv_test --target riscv32imac-unknown-none-elf
    qemu-system-riscv32 -machine virt -bios none -nographic -kernel target/riscv32imac-unknown-none-elf/debug/riscv_test


# Format the code
fmt *args:
    cargo fmt --all {{args}}
