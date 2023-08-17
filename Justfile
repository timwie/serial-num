# Build for std/no_std, with/without features
build:
  cargo build
  cargo build --all-features
  cargo build --target thumbv6m-none-eabi
  cargo build --target thumbv6m-none-eabi --features bincode,borsh,rkyv,rkyv-safe,serde

# Check for std/no_std, with/without features
check:
  cargo check
  cargo check --all-features
  cargo check --target thumbv6m-none-eabi
  cargo check --target thumbv6m-none-eabi --features bincode,borsh,rkyv,rkyv-safe,serde

# Unit tests with/without features, Kani model checking, and fuzz-testing
test:
  cargo test
  cargo test --all-features
  cargo kani --all-features
  rustup run nightly cargo fuzz run addition --all-features -- -runs=16777216
