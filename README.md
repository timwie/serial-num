This crate offers a two-byte serial number with wraparound.

A serial number is an identifier assigned incrementally to an item.
In many cases, you can use a `u32` or `u64` and call it
a day, without having to worry about overflow. The niche benefit of this type
is that it only uses the space of a `u16`, with the problem of overflow solved
by wraparound.

One scenario where this is useful, is when signing network packets with
a serial number, and being able to respond with an [ack](https://en.wikipedia.org/wiki/Acknowledgement_(data_networks)) packet
that contains a larger amount of serial numbers than when using `u32` or `u64`.
Especially with a protocol like UDP, the more numbers you can fit into that ack packet,
the more redundancy you get, and the more likely is it that all received packets are also successfully acknowledged.

<br>

```toml
[dependencies]
serial-num = "0.8"

# or with additional features:
[dependencies]
serial-num = { version = "0.8", features = ["serde"] }
```

The Minimum Supported Rust Version (MSRV) for this crate is `1.67.0`.

<br>

## Feature Flags
The following feature flags implement additional traits for the `Serial` type:
* `arbitrary`: derives [arbitrary]'s `Arbitrary` (⚠️ requires `std`)
* `bincode`: derives [bincode]'s `Decode/Encode`
* `bitcode`: derives [bitcode]'s `Decode/Encode` (⚠️ requires `std`)
* `borsh`: derives [borsh]'s `BorshDeserialize/BorshSerialize`
* `bytemuck`: derives [bytemuck]'s `Pod/Zeroable`
* `postcard`: derives [postcard]'s `Schema/MaxSize`
* `rkyv`: derives [rkyv]'s `Archive/Deserialize/Serialize`
* `rkyv-safe`: additionally enables [rkyv]’s safe API
* `serde`: derives [serde]'s `Deserialize/Serialize`
* `speedy`: derives [speedy]'s `Readable/Writable` (⚠️ requires `std`)

[arbitrary]: https://crates.io/crates/arbitrary
[bincode]: https://crates.io/crates/bincode
[bitcode]: https://crates.io/crates/bitcode
[borsh]: https://crates.io/crates/borsh
[bytemuck]: https://crates.io/crates/bytemuck
[postcard]: https://crates.io/crates/postcard
[rkyv]: https://crates.io/crates/rkyv
[serde]: https://crates.io/crates/serde
[speedy]: https://crates.io/crates/speedy


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
* Increase MSRV to `1.69.0`
* Add `postcard` feature

### [0.8.0] - 2023-10-20
* Make `Serial` `#[repr(transparent)]`
* Add `bytemuck` feature

### [0.7.0] - 2023-10-06
* Remove non-canonical `Ord` implementation
* Add `min` and `max` functions, since they are no longer provided by `Ord`
* Up `borsh` to `^1`
* Relax `arbitrary` requirement from `^1.1.0` to `^1`

### [0.6.0] - 2023-09-10
* Do not require the `rkyv/alloc` feature when enabling
  the `rkyv` feature
* Require `rkyv >=0.7,<1` instead of `~0`
  due to `RUSTSEC-2021-0054`
* The `rkyv/alloc` feature actually did not exist prior
  to `0.7`, so this lenient version requirement could
  have lead to problems before. To prevent such issues
  in the future, all dependencies will require to be on
  somewhat recent versions:
  * `arbitrary` changed from `~1` to `^1.1`
  * `bitcode` changed from `~0` to `>=0.4,<1`
  * `borsh` changed from `~0` to `>=0.10,<1`
  * `serde` changed from `~1` to `^1.0.184`
  * `speedy` changed from `~0` to `>=0.8,<1`

### [0.5.1] - 2023-08-17
* Update README
* Update keywords & categories on `crates.io`

### [0.5.0] - 2023-07-28
* Fix `rkyv` feature usage without `rkyv-safe`
* Add `bitcode` feature
* Add `speedy` feature

### [0.4.0] - 2023-05-05
* Set MSRV to `1.66.0`
* Add some `#[must_use]` attributes

### [0.3.1] - 2023-04-28
* Add `documentation` to `Cargo.toml`
* Fix outdated README

### [0.3.0] - 2023-04-28
* Set MSRV to `1.63.0`
* Add `borsh` feature
* Add `rkyv` and `rkyv-safe` features
* Add `arbitrary` feature

### [0.2.0] - 2023-04-27
* Set MSRV to `1.60.0`
* Up `bincode` to `^2.0.0-rc.3`
* Improved documentation

### [0.1.1] - 2023-01-06
* Disabled the `std` features of bincode/serde to enable `no_std` support.

[0.1.1]: https://github.com/timwie/serial-num/releases/tag/v0.1.1
[0.2.0]: https://github.com/timwie/serial-num/releases/tag/v0.2.0
[0.3.0]: https://github.com/timwie/serial-num/releases/tag/v0.3.0
[0.3.1]: https://github.com/timwie/serial-num/releases/tag/v0.3.1
[0.4.0]: https://github.com/timwie/serial-num/releases/tag/v0.4.0
[0.5.0]: https://github.com/timwie/serial-num/releases/tag/v0.5.0
[0.5.1]: https://github.com/timwie/serial-num/releases/tag/v0.5.1
[0.6.0]: https://github.com/timwie/serial-num/releases/tag/v0.6.0
[0.7.0]: https://github.com/timwie/serial-num/releases/tag/v0.7.0
[0.8.0]: https://github.com/timwie/serial-num/releases/tag/v0.8.0
