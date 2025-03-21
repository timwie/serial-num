## Release Notes
### Unreleased
* **Breaking**: Increase MSRV to `1.85.0`
* **Breaking**: Bump `rkyv` to `>=0.8,<1`
* **Breaking**: Bump `bincode` to `^2`
* **Breaking**: Bump `bitcode` to `>=0.6,<1`
* **Breaking**: Bump `postcard` to `^1.1`
* Add `proptest` feature

### [0.10.0] - 2024-07-17
* Add `or()` function to `Serial`
* Add `or_default()` function to `Serial`
* Add `take()` function to `Serial`

### [0.9.0] - 2024-04-29
* **Breaking**: Increase MSRV to `1.70.0` (minimum version to run the test suite)
* **Breaking**: Remove non-canonical `PartialOrd` implementation ([#1])
* Make `partial_cmp()` an inherent method of `Serial`
* Add `precedes()`, `precedes_or_eq()`, `succeeds()`, and `succeeds_or_eq()` methods
  to replace now missing `<`, `<=`, `>`, and `>=` operators
* Add `postcard` feature

### [0.8.0] - 2023-10-20
* Make `Serial` `#[repr(transparent)]`
* Add `bytemuck` feature

### [0.7.0] - 2023-10-06
* **Breaking**: Remove non-canonical `Ord` implementation
* **Breaking**: Up `borsh` requirement to `^1`
* Add `min` and `max` functions, since they are no longer provided by `Ord`
* Relax `arbitrary` requirement from `^1.1.0` to `^1`

### [0.6.0] - 2023-09-10
* **Breaking**: Require `rkyv >=0.7,<1` instead of `~0` ue to `RUSTSEC-2021-0054`
* **Breaking**: The `rkyv/alloc` feature actually did not exist prior
  to `0.7`, so this lenient version requirement could
  have lead to problems before. To prevent such issues
  in the future, all dependencies will require to be on
  somewhat recent versions:
    * `arbitrary` changed from `~1` to `^1.1`
    * `bitcode` changed from `~0` to `>=0.4,<1`
    * `borsh` changed from `~0` to `>=0.10,<1`
    * `serde` changed from `~1` to `^1.0.184`
    * `speedy` changed from `~0` to `>=0.8,<1`
* Do not require the `rkyv/alloc` feature when enabling the `rkyv` feature

### [0.5.1] - 2023-08-17
* Update README
* Update keywords & categories on `crates.io`

### [0.5.0] - 2023-07-28
* Fix `rkyv` feature usage without `rkyv-safe`
* Add `bitcode` feature
* Add `speedy` feature

### [0.4.0] - 2023-05-05
* **Breaking**: Set MSRV to `1.66.0` (minimum version to run the test suite)
* Add some `#[must_use]` attributes

### [0.3.1] - 2023-04-28
* Add `documentation` to `Cargo.toml`
* Fix outdated README

### [0.3.0] - 2023-04-28
* Set MSRV to `1.63.0` (minimum version to run the test suite)
* Add `borsh` feature
* Add `rkyv` and `rkyv-safe` features
* Add `arbitrary` feature

### [0.2.0] - 2023-04-27
* **Breaking**: Set MSRV to `1.60.0` (minimum version to run the test suite)
* **Breaking**: Up `bincode` to `^2.0.0-rc.3`
* Improved documentation

### [0.1.1] - 2023-01-06
* Disabled the `std` features of bincode/serde to fix `no_std` support.

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
[0.9.0]: https://github.com/timwie/serial-num/releases/tag/v0.9.0
[0.10.0]: https://github.com/timwie/serial-num/releases/tag/v0.10.0

[#1]: https://github.com/timwie/serial-num/issues/1
