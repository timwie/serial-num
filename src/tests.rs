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

#[test]
fn min() {
    assert_eq!(Serial(0), Serial(0).min(Serial(5)));
    assert_eq!(Serial(0), Serial(5).min(Serial(0)));

    assert_eq!(Serial(0), Serial(0).min(Serial(MID_U16)));
    assert_eq!(Serial(0), Serial(MID_U16).min(Serial(0)));

    assert_eq!(Serial(MID_U16 + 1), Serial(0).min(Serial(MID_U16 + 1)));
    assert_eq!(Serial(MID_U16 + 1), Serial(MID_U16 + 1).min(Serial(0)));
}

#[test]
fn max() {
    assert_eq!(Serial(5), Serial(0).max(Serial(5)));
    assert_eq!(Serial(5), Serial(5).max(Serial(0)));

    assert_eq!(Serial(MID_U16), Serial(0).max(Serial(MID_U16)));
    assert_eq!(Serial(MID_U16), Serial(MID_U16).max(Serial(0)));

    assert_eq!(Serial(0), Serial(0).max(Serial(MID_U16 + 1)));
    assert_eq!(Serial(0), Serial(MID_U16 + 1).max(Serial(0)));
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
    use borsh::BorshDeserialize;

    for n in 0..u16::MAX {
        let expected = Serial(n);

        let encoded = borsh::to_vec(&expected).unwrap();
        assert_eq!(2, encoded.len());

        let actual = Serial::try_from_slice(&encoded).unwrap();
        assert_eq!(expected, actual);
    }
}

#[test]
#[cfg(feature = "bytemuck")]
fn bytemuck_cast_roundtrip() {
    let original = Serial(42);
    let casted: u16 = bytemuck::cast(original);
    let casted_back: Serial = bytemuck::cast(casted);
    assert_eq!(original, casted_back);
}

#[test]
#[cfg(feature = "bytemuck")]
fn bytemuck_cast_and_zeroed() {
    let serial = Serial(42);
    let actual_bytes = bytemuck::bytes_of(&serial);
    let expected_bytes = 42_u16.to_le_bytes();
    assert_eq!(&expected_bytes, actual_bytes);

    assert_eq!(&serial, bytemuck::from_bytes::<Serial>(&actual_bytes));

    let actual_u16: u16 = bytemuck::cast(serial);
    let expected_u16 = 42_u16;
    assert_eq!(expected_u16, actual_u16);

    let mut actual_zeroed = Serial(42);
    bytemuck::write_zeroes(&mut actual_zeroed);
    let expected_zeroed = Serial(0);
    assert_eq!(expected_zeroed, actual_zeroed);

    let mut actual_zeroed = [Serial(0), Serial(1), Serial(2)];
    bytemuck::fill_zeroes(&mut actual_zeroed);
    let expected_zeroed = [Serial(0), Serial(0), Serial(0)];
    assert_eq!(expected_zeroed, actual_zeroed);
}

#[test]
#[cfg(feature = "postcard")]
fn postcard_maxsize() {
    use postcard::experimental::max_size::MaxSize;
    assert_eq!(3, Serial::POSTCARD_MAX_SIZE);
    assert_eq!(3, u16::POSTCARD_MAX_SIZE); // sanity check
}

#[test]
#[cfg(feature = "postcard")]
fn postcard_roundtrip() {
    for n in 0..u16::MAX {
        let expected = Serial(n);

        let mut buf = [0_u8; 3];

        let bytes = postcard::to_slice(&expected, &mut buf).unwrap();

        let actual: Serial = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(actual, expected);
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
