//! Serial number type with wraparound.
//!
//! # Simple example
//! ```
//! use serial_num::Serial;
//!
//! // the default is a reference point - not serial number "zero"
//! let mut a = Serial::default();
//! let mut b = Serial::default();
//! let mut c = Serial::default();
//!
//! // three ways to increase
//! let x = a.increase_get(); // increase, then copy
//! let y = b.get_increase(); // copy, then increase
//! c.increase();
//!
//! assert!(y < x);
//! assert_eq!(-1_i16, y.diff(x)); // "diff()" is signed
//! assert_eq!(1_u16, y.dist(x)); // "dist()" is unsigned
//!
//! // addition is the same as calling "increase()" n times
//! assert_eq!(y + 1_u16, x);
//! ```
//!
//! # Wraparound example
//! ```
//! use serial_num::Serial;
//!
//! // a serial number can be increased indefinitely
//! let mut x = Serial::default();
//! for _ in 0..u16::MAX {
//!     x.increase();
//! }
//! let x = x + u16::MAX + u16::MAX + u16::MAX;
//!
//! // comparison is trivial as long as two serial numbers have
//! // a distance of less than half of our number space (32767).
//! let a = Serial::default() + 5;
//! let b = Serial::default() + 32000;
//! assert!(a < b); // 5th successor < 32000th successor
//!
//! // but: the comparison flips if the distance is larger
//! let a = Serial::default() + 5;
//! let b = Serial::default() + 65000;
//! assert!(a > b); // 5th successor > 65000th successor
//!
//! // this means that you get the right ordering as long as
//! // you compare one serial number at most with one that
//! // is its 32767th successor.
//!
//! // a real use case of this is to sign UDP packets with
//! // a serial number. this would allow you to restore the
//! // order of packets at the receiver as long as you never
//! // look at more than the 32767 last packets (which
//! // should be much more than you need).
//! ```
//!
//! # The `NAN` value
//! ```
//! use serial_num::Serial;
//!
//! // "NAN" exists to have value representing "no serial number",
//! // since it saves encoding space vs wrapping Serial in an Option.
//! let nan = Serial::NAN;
//! let default = Serial::default();
//!
//! // you can check whether a serial number is NAN
//! assert!(nan.is_nan());
//!
//! // NAN cannot be increased
//! assert_eq!(Serial::NAN, nan + 1_u16);
//!
//! // distance between two NAN values is zero
//! assert_eq!(0_u16, nan.dist(nan));
//! assert_eq!(0_i16, nan.diff(nan));
//!
//! // distance and difference of non-NAN to NAN is the maximum distance
//! assert_eq!(32_767_u16, default.dist(nan));
//! assert_eq!(32_767_u16, nan.dist(default));
//! assert_eq!(32_767_i16, default.diff(nan));
//! assert_eq!(32_767_i16, nan.diff(default));
//!
//! // partial ordering does not include the NAN value
//! assert_eq!(None, nan.partial_cmp(&default));
//! assert!(!(nan < default) && !(nan >= default));
//! ```
#![deny(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::suspicious,
    clippy::wildcard_dependencies,
    elided_lifetimes_in_paths,
    legacy_derive_helpers,
    unknown_lints,
    unused_imports,
    unused_mut,
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_intra_doc_links
)]
#![warn(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::cast_possible_wrap,
    clippy::integer_division,
    clippy::std_instead_of_core
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::decimal_literal_representation,
    clippy::implicit_return,
    clippy::min_ident_chars,
    clippy::missing_docs_in_private_items,
    clippy::missing_trait_methods,
    clippy::question_mark_used
)]
#![cfg_attr(
    not(any(feature = "arbitrary", feature = "bitcode", feature = "speedy",)),
    no_std
)]

#[cfg(test)]
mod tests;

#[cfg(all(test, not(miri), feature = "arbitrary"))]
mod tests_prop;

#[cfg(kani)]
mod tests_kani;

use core::cmp::Ordering;
use core::ops::Add;

/// Two-byte serial number with wraparound.
///
/// A serial number is an identifier assigned incrementally to an item.
/// In many cases, you can use a `u32` or `u64` and call it
/// a day, without having to worry about overflow. The niche benefit of this type
/// is that it only uses the space of a `u16`, with the problem of overflow solved
/// by wraparound.
///
/// This is an "opaque" type, similar to `Instants`.
/// Serial numbers get their significance when being compare to one another,
/// but there is no method to get the "inner counter". Another similarity
/// is that there is no "maximum" serial number, since every
/// serial number has a successor.
///
/// The window used for comparing two serial numbers is half of our number space,
/// `(u16::MAX-1)/2 = 32767`. If two serial numbers are within that window, we simply compare
/// the numbers as you normally would. If we compare numbers that do not fit into
/// that window, like `5` and `65000`, the comparison is flipped, and we say `65000 < 5`.
/// This is based on the assumption that we got to `5` by increasing `65000` beyond
/// the point of wraparound at `u16::MAX-1 = 65534`. The assumption only holds if the items you
/// assign serial numbers to have a short enough lifetime. The ordering of items in your state
/// will get messed up if there is an item that is the `32767`th successor of another item.
///
/// The final value in our number space, `u16::MAX`, is reserved for the special
/// [`NAN`](Self::NAN) value. This is done to save space - you don't need to wrap
/// this type in an `Option` if only some items are assigned a serial number.
#[must_use]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "bincode", derive(bincode::Decode, bincode::Encode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Decode, bitcode::Encode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshDeserialize, borsh::BorshSerialize)
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(
    feature = "postcard",
    derive(
        postcard::experimental::max_size::MaxSize,
        postcard::experimental::schema::Schema
    )
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(compare(PartialEq)),
    archive_attr(derive(Clone, Copy, Debug))
)]
#[cfg_attr(feature = "rkyv-safe", archive(check_bytes))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct Serial(u16);

const NAN_U16: u16 = u16::MAX;
const NAN_U32: u32 = 65_535;
const MAX_U16: u16 = u16::MAX - 1;
const MID_I32: i32 = 32_767;
const MID_U16: u16 = 32_767;

impl Serial {
    /// Special value representing "no serial number".
    ///
    /// By convention, this "number" cannot be increased, or added to.
    pub const NAN: Self = Self(NAN_U16);

    /// Returns `true` if this number is [`NAN`](Self::NAN).
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        self == Self::NAN
    }

    /// Increases `self` with wraparound.
    #[inline]
    pub fn increase(&mut self) {
        if self.is_nan() {
            return;
        }
        if self.0 < MAX_U16 {
            self.0 += 1;
        } else {
            self.0 = 0; // wraparound
        }
    }

    /// Increases `self` with wraparound, and returns a copy.
    #[inline]
    pub fn increase_get(&mut self) -> Self {
        self.increase();
        *self
    }

    /// Returns a copy of `self`, and increases `self` with wraparound.
    #[inline]
    pub fn get_increase(&mut self) -> Self {
        let num = *self;
        self.increase();
        num
    }

    /// Distance with wraparound.
    ///
    /// For the signed difference, use [`Self::diff()`].
    ///
    /// If one of the number is [`NAN`](Self::NAN), the maximum distance of `32767` is returned.
    /// If both are [`NAN`](Self::NAN), we say the distance is `0`.
    #[inline]
    #[must_use]
    pub fn dist(self, other: Self) -> u16 {
        if self.is_nan() && other.is_nan() {
            return 0;
        }
        if self.is_nan() || other.is_nan() {
            return MID_U16; // max distance
        }
        if self.0 == other.0 {
            return 0;
        }

        let min = self.min(other);
        let max = self.max(other);

        if min.0 < max.0 {
            max.0 - min.0
        } else {
            // min is less, but has higher number
            //  => sum these distances: min->MAX + 0->max + MAX->0
            MAX_U16 - min.0 + max.0 + 1
        }
    }

    /// Difference with wraparound.
    ///
    /// If `self < other`, the result is negative,
    /// and if `self > other`, the result is positive.
    ///
    /// For the unsigned distance, use [`Self::dist()`].
    ///
    /// If one of the number is [`NAN`](Self::NAN), the maximum difference of `(-)32767`
    /// is returned. If both are [`NAN`](Self::NAN), we say the difference is `0`.
    #[inline]
    #[must_use]
    pub fn diff(self, other: Self) -> i16 {
        let dist = self.dist(other);
        if let Some(Ordering::Less) = self.partial_cmp(&other) {
            -(dist as i16)
        } else {
            dist as i16
        }
    }

    /// Compares and returns the smaller of two numbers.
    ///
    /// The returned number is the "predecessor" of the other.
    ///
    /// If one number is [`NAN`](Self::NAN), then the other is returned.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(Ordering::Less) => self,
            Some(_) => other,
            None if self.is_nan() => other,
            None => self,
        }
    }

    /// Compares and returns the larger of two numbers.
    ///
    /// The returned number is the "successor" of the other.
    ///
    /// If one number is [`NAN`](Self::NAN), then the other is returned.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(Ordering::Greater) => self,
            Some(_) => other,
            None if self.is_nan() => other,
            None => self,
        }
    }
}

impl Add<u16> for Serial {
    type Output = Serial;

    /// Addition with wraparound.
    ///
    /// You can add any `u16` to the serial number, but be aware that due to the wraparound
    /// semantics, adding more than `(u16::MAX-1)/2 = 32767` leads to a result that is
    /// _less_ than `self`. Adding `u16::MAX` will wraparound to the same value.
    ///
    /// If `self.is_nan()`, then the returned serial number is also [`NAN`](Self::NAN).
    #[inline]
    fn add(self, rhs: u16) -> Self::Output {
        if self.is_nan() {
            return self;
        }
        let n = (u32::from(self.0) + u32::from(rhs)) % (NAN_U32);
        Self(n as u16)
    }
}

impl PartialOrd for Serial {
    /// Partial comparison with wraparound.
    ///
    /// Returns `None` if one of the values is [`NAN`](Self::NAN).
    ///
    /// Based on [RFC1982].
    ///
    /// [RFC1982]: https://www.rfc-editor.org/rfc/rfc1982#section-3.2
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            return None;
        }
        if self.0 == other.0 {
            return Some(Ordering::Equal);
        }

        let a = i32::from(self.0);
        let b = i32::from(other.0);

        // a < b if either:
        //  - b has the greater number and is within our window
        //  - a has the greater number and is outside our window
        if (b > a && b - a <= MID_I32) || (a > b && a - b > MID_I32) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}
