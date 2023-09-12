use crate::Serial;
use proptest::prelude::*;
use proptest_arbitrary_interop::arb;

proptest! {
    #[test]
    fn add_without_overflow(serial in arb::<Serial>(), n: u16) {
        let res = serial + n;
        if n == u16::MAX {
            assert_eq!(serial, res);
        }
    }

    #[test]
    fn increase_without_overflow(serial in arb::<Serial>()) {
        let mut a = serial;
        let mut b = serial;
        let mut c = serial;

        a.increase();
        let _ = b.increase_get();
        let _ = c.get_increase();

        if !serial.is_nan() {
            assert!(serial < a);
            assert!(serial < b);
            assert!(serial < c);
        } else {
            assert!(a.is_nan());
            assert!(b.is_nan());
            assert!(c.is_nan());
        }
    }

    #[test]
    fn max_dist(a in arb::<Serial>(), b in arb::<Serial>()) {
        if a == b {
            assert_eq!(0, a.dist(b));
        }
        assert!(a.dist(b) <= 32767);
        assert_eq!(a.dist(b), b.dist(a));
    }

    #[test]
    fn max_diff(a in arb::<Serial>(), b in arb::<Serial>()) {
        let a_diff_b = a.diff(b);
        let b_diff_a = b.diff(a);

        if a == b {
            assert_eq!(0, a_diff_b);
        }

        assert!(a_diff_b <= 32767 || a.diff(b) >= a_diff_b);
        assert!(a_diff_b == b_diff_a.abs() || a_diff_b.abs() == b_diff_a);
    }

    #[test]
    fn cmp(a in arb::<Serial>(), b in arb::<Serial>()) {
        assert_eq!(a.cmp(&b), b.cmp(&a).reverse());

        if !a.is_nan() && !b.is_nan() {
            assert_eq!(a.partial_cmp(&b).unwrap(), b.partial_cmp(&a).unwrap().reverse());
        }
    }


    #[test]
    #[cfg(feature = "bincode")]
    fn bincode_roundtrip(expected in arb::<Serial>()) {
        let cfg = bincode::config::standard().with_fixed_int_encoding();

        let mut buf = [0_u8; 2];
        let n_bytes = bincode::encode_into_slice(expected, &mut buf, cfg).unwrap();
        assert_eq!(2, n_bytes);

        let (actual, _): (Serial, _) = bincode::decode_from_slice(&buf, cfg).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(feature = "borsh")]
    fn borsh_roundtrip(expected in arb::<Serial>()) {
        use borsh::{BorshDeserialize, BorshSerialize};

        let encoded = expected.try_to_vec().unwrap();
        assert_eq!(2, encoded.len());

        let actual = Serial::try_from_slice(&encoded).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(feature = "rkyv")]
    fn rkyv_roundtrip(expected in arb::<Serial>()) {
        let bytes = rkyv::to_bytes::<_, 256>(&expected).unwrap();

        let actual = unsafe { rkyv::archived_root::<Serial>(&bytes[..]) };
        assert_eq!(actual, &expected);
    }

    #[test]
    #[cfg(feature = "rkyv-safe")]
    fn rkyv_safe_roundtrip(expected in arb::<Serial>()) {
        let bytes = rkyv::to_bytes::<_, 256>(&expected).unwrap();

        let actual = rkyv::check_archived_root::<Serial>(&bytes[..]).unwrap();
        assert_eq!(actual, &expected);
    }

    #[test]
    #[cfg(feature = "speedy")]
    fn speedy_roundtrip(expected in arb::<Serial>()) {
        use speedy::{Readable, Writable};

        let encoded = expected.write_to_vec().unwrap();
        assert_eq!(2, encoded.len());

        let actual = Serial::read_from_buffer(&encoded).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(feature = "bitcode")]
    fn bitcode_roundtrip(expected in arb::<Serial>()) {
        let encoded = bitcode::encode(&expected).unwrap();
        assert_eq!(2, encoded.len());

        let actual: Serial = bitcode::decode(&encoded).unwrap();
        assert_eq!(expected, actual);
    }
}