# Build for std/no_std, with/without features
build:
  cargo build
  cargo build --all-features
  cargo build --target thumbv6m-none-eabi
  cargo build --target thumbv6m-none-eabi --features bincode,borsh,postcard,rkyv,rkyv-safe,serde

# Check for std/no_std, with/without features
check:
  cargo check
  cargo check --all-features
  cargo check --target thumbv6m-none-eabi
  cargo check --target thumbv6m-none-eabi --features bincode,borsh,bytemuck,postcard,rkyv,rkyv-safe,serde
  cargo +nightly check --all-features

# Remove target directories
clean:
  cargo clean --manifest-path fuzz/Cargo.toml
  cargo clean

# Run fuzz-tests
fuzz:
  cargo +nightly fuzz run addition -- -runs=16777216
  cargo +nightly fuzz run increment -- -runs=16777216

# Try to find the MSRV
msrv:
  cargo msrv -- cargo check --all-features

# Unit tests with/without features, Kani model checking, and fuzz-testing
test:
  cargo test
  cargo test --all-features
  cargo kani --tests --all-features

# Update dependencies and lock files
update:
  cargo update
  cargo update --manifest-path=fuzz/Cargo.toml
