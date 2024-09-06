#![doc = include_str!("crate.md")]
#![doc = include_str!("examples.md")]
#![cfg_attr(
    not(any(test, feature = "arbitrary", feature = "bitcode", feature = "speedy",)),
    no_std
)]

#[cfg(test)]
mod tests;

#[cfg(all(test, not(miri), feature = "arbitrary"))]
mod tests_prop;

#[cfg(kani)]
mod tests_kani;

#[cfg(all(test, not(miri)))]
mod tests_readme;

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
/// The window used for comparing two serial numbers is half of the number space,
/// `(u16::MAX-1)/2 = 32767`. If two serial numbers are within that window, we simply compare
/// the numbers as you normally would. If we compare numbers that do not fit into
/// that window, like `5` and `65000`, the comparison is flipped, and we say `65000 < 5`.
/// This is based on the assumption that we got to `5` by increasing `65000` beyond
/// the point of wraparound at `u16::MAX-1 = 65534`. The assumption only holds if the items you
/// assign serial numbers to have a short enough lifetime. The ordering of items in your state
/// will get messed up if there is an item that is the `32767`th successor of another item.
///
/// The final value in the number space, `u16::MAX`, is reserved for the special
/// [`NAN`](Self::NAN) value. This is done to save space - you don't need to wrap
/// this type in an `Option` if only some items are assigned a serial number.
#[doc = include_str!("examples.md")]
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
    #[expect(clippy::arithmetic_side_effects, reason = "overflow is handled")]
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
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "cannot overflow in the arithmetic"
    )]
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
            // min is predecessor, and counter is lower
            // distance is: min->max
            max.0 - min.0
        } else {
            // min is predecessor, but counter is higher
            // distance is: min->MAX + 0->max + MAX->0
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
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "negating 'dist' <= 32767 won't overflow"
    )]
    #[expect(
        clippy::as_conversions,
        reason = "casting 'dist' <= 32767 to i16 won't overflow"
    )]
    #[expect(
        clippy::cast_possible_wrap,
        reason = "casting 'dist' <= 32767 to i16 won't overflow"
    )]
    pub fn diff(self, other: Self) -> i16 {
        let dist = self.dist(other);
        if self.precedes(other) {
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
        match self.partial_cmp(other) {
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
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => self,
            Some(_) => other,
            None if self.is_nan() => other,
            None => self,
        }
    }

    /// Partial comparison with wraparound.
    ///
    /// Returns `None` if one of the values is [`NAN`](Self::NAN).
    ///
    /// Based on [RFC1982].
    ///
    /// [RFC1982]: https://www.rfc-editor.org/rfc/rfc1982#section-3.2
    #[inline]
    #[must_use]
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "overflow is handled by comparing before the arithmetic"
    )]
    pub fn partial_cmp(self, other: Self) -> Option<Ordering> {
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

    /// `True` if `self < other` according to [RFC1982].
    ///
    /// [RFC1982]: https://www.rfc-editor.org/rfc/rfc1982#section-3.2
    #[inline]
    #[must_use]
    pub fn precedes(self, other: Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Less)
    }

    /// `True` if `self <= other` according to [RFC1982].
    ///
    /// [RFC1982]: https://www.rfc-editor.org/rfc/rfc1982#section-3.2
    #[inline]
    #[must_use]
    pub fn precedes_or_eq(self, other: Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Less | Ordering::Equal) => true,
            Some(Ordering::Greater) | None => false,
        }
    }

    /// `True` if `self > other` according to [RFC1982].
    ///
    /// [RFC1982]: https://www.rfc-editor.org/rfc/rfc1982#section-3.2
    #[inline]
    #[must_use]
    pub fn succeeds(self, other: Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Greater)
    }

    /// `True` if `self >= other` according to [RFC1982].
    ///
    /// [RFC1982]: https://www.rfc-editor.org/rfc/rfc1982#section-3.2
    #[inline]
    #[must_use]
    pub fn succeeds_or_eq(self, other: Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater | Ordering::Equal) => true,
            Some(Ordering::Less) | None => false,
        }
    }

    /// Returns `self` if it's not `NAN`, otherwise returns `other`.
    #[inline]
    pub fn or(self, other: Self) -> Self {
        if self.is_nan() {
            other
        } else {
            self
        }
    }

    /// Returns `self` if it's not `NAN`, otherwise returns `Serial::default()`.
    #[inline]
    pub fn or_default(self) -> Self {
        if self.is_nan() {
            Self::default()
        } else {
            self
        }
    }

    /// Replaces `self` with `NAN`, returning the previous value.
    #[inline]
    pub fn take(&mut self) -> Self {
        core::mem::replace(self, Self::NAN)
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
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "the addition cannot overflow"
    )]
    #[expect(clippy::as_conversions, reason = "cannot overflow after modulo usage")]
    fn add(self, rhs: u16) -> Self::Output {
        if self.is_nan() {
            return self;
        }
        let n = (u32::from(self.0) + u32::from(rhs)) % NAN_U32;
        Self(n as u16)
    }
}
