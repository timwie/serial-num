# Build for std/no_std, with/without features
build:
  cargo build
  cargo build --all-features
  cargo build --target thumbv6m-none-eabi
  cargo build --target thumbv6m-none-eabi     --features bincode,borsh,postcard,rkyv,rkyv-safe,serde
  cargo build --target i686-unknown-linux-gnu
  cargo build --target i686-unknown-linux-gnu --features bincode,bitcode,borsh,postcard,rkyv,rkyv-safe,serde

# Check for std/no_std, with/without features
check:
  cargo check
  cargo check --all-features
  cargo check --target thumbv6m-none-eabi
  cargo check --target thumbv6m-none-eabi     --features bincode,borsh,postcard,rkyv,rkyv-safe,serde
  cargo check --target i686-unknown-linux-gnu
  cargo check --target i686-unknown-linux-gnu --features bincode,bitcode,borsh,bytemuck,postcard,rkyv,rkyv-safe,serde

# Unit tests with/without features, and Kani model checking
test:
  cargo test
  cargo test --all-features
  cargo kani --tests --all-features

# Build & test for randomly selected features
random:
  #!/usr/bin/env bash
  FEATURES=('arbitrary' 'bincode' 'bitcode' 'borsh' 'bytemuck' 'postcard' 'rkyv' 'rkyv-safe' 'serde' 'speedy')
  NUM_SELECTED=$(shuf -i 2-${#FEATURES[@]} -n 1)
  SELECTED=$(shuf -e ${FEATURES[@]} -n $NUM_SELECTED | paste -sd, -)
  echo "Randomly selected '$SELECTED'"
  cargo build --features $SELECTED
  cargo test --features $SELECTED
