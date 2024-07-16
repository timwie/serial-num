This crate offers a two-byte serial number with wraparound.

[Release notes](https://github.com/timwie/serial-num/blob/main/RELEASES.md)

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

Due to the limited number space, recently allocated serial numbers may
duplicate very old serial numbers, but not other recently allocated serial numbers.
To avoid ambiguity with these non-unique numbers, [RFC 1982 "Serial Number Arithmetic"](https://datatracker.ietf.org/doc/html/rfc1982),
defines special rules for calculations involving these kinds of serial numbers. 

<br>

```toml
[dependencies]
serial-num = "0.10"

# or with additional features:
[dependencies]
serial-num = { version = "0.10", features = ["serde"] }
```

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
