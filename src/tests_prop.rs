use super::*;
use proptest::prelude::*;
use std::format;

proptest! {
    #[test]
    fn add_without_overflow(serial in any::<Serial>(), n: u16) {
        let res = serial + n;
        if n == u16::MAX {
            prop_assert_eq!(serial, res);
        }
    }

    #[test]
    fn increase_without_overflow(serial in any::<Serial>()) {
        let mut a = serial;
        let mut b = serial;
        let mut c = serial;

        a.increase();
        let _ = b.increase_get();
        let _ = c.get_increase();

        if !serial.is_nan() {
            prop_assert!(serial.precedes(a));
            prop_assert!(serial.precedes_or_eq(a));
            prop_assert!(!serial.succeeds_or_eq(a));
            prop_assert!(!serial.succeeds(a));

            prop_assert!(serial.precedes(b));
            prop_assert!(serial.precedes_or_eq(b));
            prop_assert!(!serial.succeeds_or_eq(b));
            prop_assert!(!serial.succeeds(b));

            prop_assert!(serial.precedes(c));
            prop_assert!(serial.precedes_or_eq(c));
            prop_assert!(!serial.succeeds_or_eq(c));
            prop_assert!(!serial.succeeds(c));
        } else {
            prop_assert!(a.is_nan());
            prop_assert!(b.is_nan());
            prop_assert!(c.is_nan());
        }
    }

    #[test]
    fn max_dist(a in any::<Serial>(), b in any::<Serial>()) {
        if a == b {
            prop_assert_eq!(0, a.dist(b));
        }
        prop_assert!(a.dist(b) <= 32767);
        prop_assert_eq!(a.dist(b), b.dist(a));
    }

    #[test]
    fn max_diff(a in any::<Serial>(), b in any::<Serial>()) {
        let a_diff_b = a.diff(b);
        let b_diff_a = b.diff(a);

        if a == b {
            prop_assert_eq!(0, a_diff_b);
        }

        prop_assert!(a_diff_b <= 32767 || a.diff(b) >= a_diff_b);
        prop_assert!(a_diff_b == b_diff_a.abs() || a_diff_b.abs() == b_diff_a);
    }

    #[test]
    fn cmp(a in any::<Serial>(), b in any::<Serial>()) {
        match (a.partial_cmp(b), b.partial_cmp(a)) {
            (Some(ord1), Some(ord2)) => prop_assert_eq!(ord1, ord2.reverse()),
            (None, None) => prop_assert!(a.is_nan() || b.is_nan()),
            _ => unreachable!()
        }
    }

    #[test]
    fn or(num in any::<Serial>()) {
        prop_assert_eq!(num.or(Serial::NAN), num);
        if num.is_nan() {
            prop_assert_eq!(num.or(Serial(5)), Serial(5));
        } else {
            prop_assert_eq!(num.or(Serial(5)), num);
        }
    }

    #[test]
    fn or_default(num in any::<Serial>()) {
        if num.is_nan() {
            prop_assert_eq!(num.or_default(), Serial::default());
        } else {
            prop_assert_eq!(num.or_default(), num);
        }
    }

    #[test]
    fn take(mut num in any::<Serial>()) {
        let num_copy = num;

        prop_assert_eq!(num.take(), num_copy);
        prop_assert_eq!(num, Serial::NAN);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_json_roundtrip(expected in any::<Serial>()) {
        let encoded = serde_json::to_string(&expected).unwrap();
        let actual: Serial = serde_json::from_str(&encoded).unwrap();
        prop_assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(feature = "bincode")]
    fn bincode_roundtrip(expected in any::<Serial>()) {
        let cfg = bincode::config::standard().with_fixed_int_encoding();

        let mut buf = [0_u8; 2];
        let n_bytes = bincode::encode_into_slice(expected, &mut buf, cfg).unwrap();
        prop_assert_eq!(2, n_bytes);

        let (actual, _): (Serial, _) = bincode::decode_from_slice(&buf, cfg).unwrap();
        prop_assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(feature = "borsh")]
    fn borsh_roundtrip(expected in any::<Serial>()) {
        use borsh::BorshDeserialize;

        let encoded = borsh::to_vec(&expected).unwrap();
        prop_assert_eq!(2, encoded.len());

        let actual = Serial::try_from_slice(&encoded).unwrap();
        prop_assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(feature = "bytemuck")]
    fn bytemuck_cast_roundtrip(original in any::<Serial>()) {
        let casted: u16 = bytemuck::cast(original);
        let casted_back: Serial = bytemuck::cast(casted);
        prop_assert_eq!(original, casted_back);
    }

    #[test]
    #[cfg(feature = "rkyv")]
    #[allow(unsafe_code)]
    fn rkyv_roundtrip(expected in any::<Serial>()) {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&expected).unwrap();

        let archived =
            unsafe { rkyv::access_unchecked::<ArchivedSerial>(&bytes[..]) };

        prop_assert_eq!(archived, &expected);
    }

    #[test]
    #[cfg(feature = "rkyv-safe")]
    fn rkyv_safe_roundtrip(expected in any::<Serial>()) {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&expected).unwrap();

        let archived = rkyv::access::<ArchivedSerial, rkyv::rancor::Error>(&bytes[..]).unwrap();

        prop_assert_eq!(archived, &expected);
    }

    #[test]
    #[cfg(feature = "speedy")]
    fn speedy_roundtrip(expected in any::<Serial>()) {
        use speedy::{Readable, Writable};

        let encoded = expected.write_to_vec().unwrap();
        prop_assert_eq!(2, encoded.len());

        let actual = Serial::read_from_buffer(&encoded).unwrap();
        prop_assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(feature = "bitcode")]
    fn bitcode_roundtrip(expected in any::<Serial>()) {
        let encoded = bitcode::encode(&expected);
        prop_assert_eq!(2, encoded.len());

        let actual: Serial = bitcode::decode(&encoded).unwrap();
        prop_assert_eq!(expected, actual);
    }
}
