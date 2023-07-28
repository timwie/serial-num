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
#![cfg_attr(not(any(
    feature = "arbitrary",
    feature = "bitcode",
    feature = "speedy",
)), no_std)]

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
/// `NAN` value. This is done to save space - you don't need to wrap
/// this type in an `Option` if only some items are assigned a serial number.
#[must_use]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "bincode", derive(bincode::Decode, bincode::Encode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Decode, bitcode::Encode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshDeserialize, borsh::BorshSerialize)
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
    /// By convention, we say this "serial number" is **less** than any other.
    /// It cannot be increased, or added to.
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
    /// If one of the number is `NAN`, the maximum distance of `32767` is returned.
    /// If both are `NAN`, we say the distance is `0`.
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
    /// If one of the number is `NAN`, the maximum difference of `(-)32767` is returned.
    /// If both are `NAN`, we say the difference is `0`.
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
}

impl Add<u16> for Serial {
    type Output = Serial;

    /// Addition with wraparound.
    ///
    /// You can add any `u16` to the serial number, but be aware that due to the wraparound
    /// semantics, adding more than `(u16::MAX-1)/2 = 32767` leads to a result that is
    /// _less_ than `self`. Adding `u16::MAX` will wraparound to the same value.
    ///
    /// If `self.is_nan()`, the returned serial number is also `NAN`.
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
    /// Returns `None` if one of the values is `NAN`.
    ///
    /// Based on [RFC1982].
    ///
    /// [RFC1982]: https://www.rfc-editor.org/rfc/rfc1982#section-3.2
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            return None;
        }
        Some(self.cmp(other))
    }
}

impl Ord for Serial {
    /// Compare with wraparound.
    ///
    /// By convention, we say `NAN` is **less** than any other serial number.
    ///
    /// Based on [RFC1982].
    ///
    /// [RFC1982]: https://www.rfc-editor.org/rfc/rfc1982#section-3.2
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_nan() && other.is_nan() {
            Ordering::Equal
        } else if self.is_nan() {
            Ordering::Less
        } else if other.is_nan() {
            Ordering::Greater
        } else if self.0 == other.0 {
            Ordering::Equal
        } else {
            let a = i32::from(self.0);
            let b = i32::from(other.0);

            // a < b if either:
            //  - b has the greater number and is within our window
            //  - a has the greater number and is outside our window
            if (b > a && b - a <= MID_I32) || (a > b && a - b > MID_I32) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increase_nan() {
        let mut nan = Serial::NAN;
        nan.increase();
        assert_eq!(nan, Serial::NAN)
    }

    #[test]
    fn cmp1() {
        let a = Serial::default();
        let b = Serial(MAX_U16);
        assert!(a > b);
        assert!(b < a);
    }

    #[test]
    fn cmp2() {
        let a = Serial(1000);
        let b = Serial(u16::MAX - 1000);
        assert!(a > b);
        assert!(b < a);
    }

    #[test]
    fn cmp_edge_case() {
        let zero = Serial::default();
        let mid = Serial(32767);

        assert!(mid > zero);
        assert!(zero < mid);
    }

    #[test]
    fn dist1() {
        let a = Serial::default();
        let b = Serial(MAX_U16);
        assert_eq!(a.dist(b), 1);
        assert_eq!(b.dist(a), 1);
    }

    #[test]
    fn dist2() {
        let a = Serial(1000);
        let b = Serial(MAX_U16 - 1000);
        let expected_a_diff_to_zero = 1000;
        let expected_b_diff_to_max = 1000;
        let expected = 1 + expected_a_diff_to_zero + expected_b_diff_to_max;
        assert_eq!(a.dist(b), expected);
        assert_eq!(b.dist(a), expected);
    }

    #[test]
    fn dist3() {
        let zero = Serial::default();
        let mid = Serial(32767);
        assert!(zero < mid);

        let actual1 = zero.dist(mid);
        let actual2 = mid.dist(zero);
        assert_eq!(actual1, actual2);
        assert_eq!(actual1, 32767);

        // if we increase by one, the order flips around, and the distance stays the same
        let mid_plus_one = Serial(32768);
        assert!(zero > mid_plus_one);

        let actual1 = zero.dist(mid_plus_one);
        let actual2 = mid_plus_one.dist(zero);
        assert_eq!(actual1, actual2);
        assert_eq!(actual1, 32767);
    }

    #[test]
    fn simple_example() {
        let a = Serial(5_u16);
        let b = Serial(7_u16);

        assert!(a < b);
        assert!(b > a);

        let diff = b.dist(a);
        assert_eq!(diff, 2);
    }

    #[test]
    fn wraparound_example() {
        // serial number 5 comes after sequence number 65000
        let a = Serial(5_u16);
        let mut b = Serial(65000_u16);
        assert!(a > b);
        assert!(b < a);

        let dist = b.dist(a);
        let expected_diff = MAX_U16 - 65000 + 5 + 1;
        assert_eq!(dist, expected_diff);

        let mut n_increases = 0;
        while b != a {
            let _ = b.get_increase();
            n_increases += 1;
        }
        assert_eq!(n_increases, expected_diff);
    }

    #[test]
    fn diff() {
        let a = Serial::default();
        let mut b = Serial::default();

        for _ in 0..MAX_U16 {
            b.increase();
            let diff_pos = a.diff(b);
            let diff_neg = b.diff(a);
            assert_eq!(-diff_pos, diff_neg);
        }
    }

    #[test]
    fn plus() {
        assert_eq!(Serial(5), Serial(3) + 2);

        assert_eq!(Serial(MAX_U16), Serial(0) + MAX_U16);
        assert_eq!(Serial(0), Serial(0) + MAX_U16 + 1);

        assert_eq!(Serial(0), Serial(0) + u16::MAX);
        assert_eq!(Serial(MAX_U16), Serial(MAX_U16) + u16::MAX);

        assert_eq!(Serial(5 + MID_U16), Serial(5) + MID_U16);

        assert!(Serial(0) < Serial(0) + MID_U16);
        assert!(Serial(0) > Serial(1) + MID_U16);

        assert_eq!(Serial::NAN, Serial::NAN + 1);
    }

    /// A test with a lot of coverage, but no assertions.
    #[test]
    fn no_overflows() {
        let candidates = [
            0,
            1,
            2,
            MID_U16 - 1,
            MID_U16,
            MID_U16 + 1,
            MAX_U16 - 2,
            MAX_U16 - 1,
            MAX_U16,
            NAN_U16,
        ];

        for n in candidates {
            for m in candidates {
                let a = Serial(n);
                let b = Serial(m);

                let _ = a.is_nan();
                let _ = a.dist(b);
                let _ = a.diff(b);
                let _ = a.partial_cmp(&b);
                let _ = a.cmp(&b);

                let _ = a + 0;
                let _ = a + MID_U16;
                let _ = a + u16::MAX;

                let mut c = Serial(n);
                for _ in 0..u16::MAX {
                    c.increase();
                    let _ = c.increase_get();
                    let _ = c.get_increase();
                }
            }
        }
    }

    #[test]
    #[cfg(feature = "bincode")]
    fn bincode_roundtrip() {
        let cfg = bincode::config::standard().with_fixed_int_encoding();

        for n in 0..u16::MAX {
            let expected = Serial(n);

            let mut buf = [0_u8; 2];
            let n_bytes = bincode::encode_into_slice(expected, &mut buf, cfg).unwrap();
            assert_eq!(2, n_bytes);

            let (actual, _): (Serial, _) = bincode::decode_from_slice(&buf, cfg).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    #[cfg(feature = "borsh")]
    fn borsh_roundtrip() {
        use borsh::{BorshDeserialize, BorshSerialize};

        for n in 0..u16::MAX {
            let expected = Serial(n);

            let encoded = expected.try_to_vec().unwrap();
            assert_eq!(2, encoded.len());

            let actual = Serial::try_from_slice(&encoded).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    #[cfg(feature = "rkyv")]
    fn rkyv_roundtrip() {
        for n in 0..u16::MAX {
            let expected = Serial(n);

            let bytes = rkyv::to_bytes::<_, 256>(&expected).unwrap();

            let actual = unsafe { rkyv::archived_root::<Serial>(&bytes[..]) };
            assert_eq!(actual, &expected);
        }
    }

    #[test]
    #[cfg(feature = "rkyv-safe")]
    fn rkyv_safe_roundtrip() {
        for n in 0..u16::MAX {
            let expected = Serial(n);

            let bytes = rkyv::to_bytes::<_, 256>(&expected).unwrap();

            let actual = rkyv::check_archived_root::<Serial>(&bytes[..]).unwrap();
            assert_eq!(actual, &expected);
        }
    }

    #[test]
    #[cfg(feature = "arbitrary")]
    fn arbitrary() {
        use arbitrary::{Arbitrary, Unstructured};

        let raw_data: &[u8] = "get_raw_data_from_fuzzer()".as_bytes();

        let mut unstructured = Unstructured::new(raw_data);

        _ = Serial::arbitrary(&mut unstructured).unwrap();
    }

    #[test]
    #[cfg(feature = "speedy")]
    fn speedy_roundtrip() {
        use speedy::{Readable, Writable};

        for n in 0..u16::MAX {
            let expected = Serial(n);

            let encoded = expected.write_to_vec().unwrap();
            assert_eq!(2, encoded.len());

            let actual = Serial::read_from_buffer(&encoded).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    #[cfg(feature = "bitcode")]
    fn bitcode_roundtrip() {
        for n in 0..u16::MAX {
            let expected = Serial(n);

            let encoded = bitcode::encode(&expected).unwrap();
            assert_eq!(2, encoded.len());

            let actual: Serial = bitcode::decode(&encoded).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

#[cfg(kani)]
#[kani::proof]
fn check_increase() {
    let mut s = Serial(kani::any());
    s.increase();
}

#[cfg(kani)]
#[kani::proof]
fn check_increase_get() {
    let mut s = Serial(kani::any());
    let _ = s.increase_get();
}

#[cfg(kani)]
#[kani::proof]
fn check_get_increase() {
    let mut s = Serial(kani::any());
    let _ = s.get_increase();
}

#[cfg(kani)]
#[kani::proof]
fn check_dist() {
    let a = Serial(kani::any());
    let b = Serial(kani::any());
    let _ = a.dist(b);
    let _ = b.dist(a);
}

#[cfg(kani)]
#[kani::proof]
fn check_diff() {
    let a = Serial(kani::any());
    let b = Serial(kani::any());
    let _ = a.diff(b);
    let _ = b.diff(a);
}

#[cfg(kani)]
#[kani::proof]
fn check_add() {
    let _ = Serial(kani::any()) + kani::any();
}

#[cfg(kani)]
#[kani::proof]
fn check_cmp() {
    let a = Serial(kani::any());
    let b = Serial(kani::any());
    let _ = a.partial_cmp(&b);
    let _ = a.cmp(&b);
    let _ = b.partial_cmp(&a);
    let _ = b.cmp(&a);
}

#[cfg(all(kani, feature = "bincode"))]
#[kani::proof]
fn check_bincode() {
    let cfg = bincode::config::standard().with_fixed_int_encoding();

    let expected = Serial(kani::any());

    let mut buf = [0_u8; 2];
    let n_bytes = bincode::encode_into_slice(expected, &mut buf, cfg).unwrap();
    assert_eq!(2, n_bytes);

    let (actual, _): (Serial, _) = bincode::decode_from_slice(&buf, cfg).unwrap();
    assert_eq!(expected, actual);
}

#[cfg(all(kani, feature = "borsh"))]
#[kani::proof]
fn check_borsh() {
    use borsh::{BorshDeserialize, BorshSerialize};

    let expected = Serial(kani::any());

    let encoded = expected.try_to_vec().unwrap();
    assert_eq!(2, encoded.len());

    let actual = Serial::try_from_slice(&encoded).unwrap();
    assert_eq!(expected, actual);
}

#[cfg(all(kani, feature = "rkyv"))]
// TODO #[kani::proof]
fn check_rkyv() {
    let expected = Serial(kani::any());

    let bytes = rkyv::to_bytes::<_, 256>(&expected).unwrap();

    let actual = unsafe { rkyv::archived_root::<Serial>(&bytes[..]) };
    assert_eq!(actual, &expected);
}

#[cfg(all(kani, feature = "rkyv-safe"))]
// TODO #[kani::proof]
fn check_rkyv_safe() {
    let expected = Serial(kani::any());

    let bytes = rkyv::to_bytes::<_, 256>(&expected).unwrap();

    let actual = rkyv::check_archived_root::<Serial>(&bytes[..]).unwrap();
    assert_eq!(actual, &expected);
}

#[cfg(all(kani, feature = "speedy"))]
#[kani::proof]
fn check_speedy() {
    use speedy::{Readable, Writable};

    let expected = Serial(kani::any());

    let encoded = expected.write_to_vec().unwrap();
    assert_eq!(2, encoded.len());

    let actual = Serial::read_from_buffer(&encoded).unwrap();
    assert_eq!(expected, actual);
}

#[cfg(all(kani, feature = "bitcode"))]
#[kani::proof]
fn check_bitcode() {
    let expected = Serial(kani::any());

    let encoded = bitcode::encode(&expected).unwrap();
    assert_eq!(2, encoded.len());

    let actual: Serial = bitcode::decode(&encoded).unwrap();
    assert_eq!(expected, actual);
}
