<h1 align="center">
serial-num
  
[![Latest Version]][crates.io]
[![Documentation]][docs.rs]
![License]

[Latest Version]: https://img.shields.io/crates/v/serial-num
[crates.io]: https://crates.io/crates/serial-num
[Documentation]: https://img.shields.io/docsrs/serial-num
[docs.rs]: https://docs.rs/serial-num/latest/serial_num/
[License]: https://img.shields.io/crates/l/serial-num
</h1>

This crate offers a two-byte serial number with wraparound.
A serial number is an identifier assigned incrementally to an item.
In many cases, you can use a `u32` or `u64` and call it
a day, without having to worry about overflow. The niche benefit of this type
is that it only uses the space of a `u16`, with the problem of overflow solved
by wraparound.


```toml
[dependencies]
serial-num = "0.2"

# or with additional features:
[dependencies]
serial-num = { version = "0.2", features = ["serde"] }
```

The following feature flags are available:
* `bincode`: Implements [bincode]'s `Decode/Encode` for the `Serial` type
* `borsh`: Implements [borsh]'s `BorshDeserialize/BorshSerialize` for the `Serial` type
* `serde`: Implements [serde]'s `Deserialize/Serialize` for the `Serial` type

[bincode]: https://crates.io/crates/bincode
[borsh]: https://crates.io/crates/borsh
[serde]: https://crates.io/crates/serde

The Minimum Supported Rust Version (MSRV) for this crate is `1.60.0`.

<br>

## Usage
### Simple example
```rust
use serial_num::Serial;

// the default is a reference point - not serial number "zero"
let mut a = Serial::default();
let mut b = Serial::default();
let mut c = Serial::default();

// three ways to increase
let x = a.increase_get(); // increase, then copy
let y = b.get_increase(); // copy, then increase
c.increase();

assert!(y < x);
assert_eq!(-1_i16, y.diff(x)); // "diff()" is signed
assert_eq!(1_u16, y.dist(x)); // "dist()" is unsigned

// addition is the same as calling "increase()" n times
assert_eq!(y + 1_u16, x);
```

### Wraparound example
```rust
use serial_num::Serial;

// a serial number can be increased indefinitely
let mut x = Serial::default();
for _ in 0..u16::MAX {
    x.increase();
}
let x = x + u16::MAX + u16::MAX + u16::MAX;

// comparison is trivial as long as two serial numbers have
// a distance of less than half of our number space (32767).
let a = Serial::default() + 5;
let b = Serial::default() + 32000;
assert!(a < b); // 5th successor < 32000th successor

// but: the comparison flips if the distance is larger
let a = Serial::default() + 5;
let b = Serial::default() + 65000;
assert!(a > b); // 5th successor > 65000th successor

// this means that you get the right ordering as long as
// you compare one serial number at most with one that
// is its 32767th successor.

// a real use case of this is to sign UDP packets with
// a serial number. this would allow you to restore the
// order of packets at the receiver as long as you never
// look at more than the 32767 last packets (which
// should be much more than you need).
```

### The `NAN` value
```rust
use serial_num::Serial;

// "NAN" exists to have value representing "no serial number",
// since it saves encoding space vs wrapping Serial in an Option.
let nan = Serial::NAN;
let default = Serial::default();

// you can check whether a serial number is NAN
assert!(nan.is_nan());

// NAN cannot be increased
assert_eq!(Serial::NAN, nan + 1_u16);

// distance between two NAN values is zero
assert_eq!(0_u16, nan.dist(nan));
assert_eq!(0_i16, nan.diff(nan));

// distance and difference of non-NAN to NAN is the maximum distance
assert_eq!(32_767_u16, default.dist(nan));
assert_eq!(32_767_u16, nan.dist(default));
assert_eq!(32_767_i16, default.diff(nan));
assert_eq!(32_767_i16, nan.diff(default));

// partial ordering does not include the NAN value
assert_eq!(None, nan.partial_cmp(&default));
assert!(!(nan < default) && !(nan >= default));
```

<br>

## Changelog
### Unreleased
* Add `borsh` feature

### [0.2.0] - 2023-04-27
* Improved documentation
* Set MSRV to `1.60.0`
* Up `bincode` to `^2.0.0-rc.3`

### [0.1.1] - 2023-01-06
* Disabled the `std` features of bincode/serde to enable `no_std` support.

[0.1.1]: https://github.com/timwie/serial-num/releases/tag/v0.1.1
[0.2.0]: https://github.com/timwie/serial-num/releases/tag/v0.2.0

<br>

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
