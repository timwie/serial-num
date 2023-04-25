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

README is forthcoming :construction:

<br>

## Feature Flags
* `bincode`: Implements [bincode]'s `Decode/Encode` for the `Serial` type
* `serde`: Implements [serde]'s `Deserialize/Serialize` for the `Serial` type

[bincode]: https://crates.io/crates/bincode
[serde]: https://crates.io/crates/serde

<br>

## Changelog
### Unreleased
* Set MSRV to `1.60.0`

### [0.1.1] - 2023-01-06
* Disabled the `std` features of bincode/serde to enable `no_std` support.

[0.1.1]: https://github.com/timwie/serial-num/releases/tag/v0.1.1
