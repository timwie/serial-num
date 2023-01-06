//! Serial number type with wraparound.
#![no_std]

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
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bincode", derive(bincode::Decode, bincode::Encode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let n = (self.0 as u32 + rhs as u32) % (NAN_U32);
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

        if self.0 == other.0 {
            return Some(Ordering::Equal);
        }

        let a = self.0 as i32;
        let b = other.0 as i32;

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
        } else {
            self.partial_cmp(other).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
