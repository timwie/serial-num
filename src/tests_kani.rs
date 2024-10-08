use super::*;

#[kani::proof]
fn check_increase() {
    let mut s = Serial(kani::any());
    s.increase();
}

#[kani::proof]
fn check_increase_get() {
    let mut s = Serial(kani::any());
    let _ = s.increase_get();
}

#[kani::proof]
fn check_get_increase() {
    let mut s = Serial(kani::any());
    let _ = s.get_increase();
}

#[kani::proof]
fn check_dist() {
    let a = Serial(kani::any());
    let b = Serial(kani::any());
    let _ = a.dist(b);
    let _ = b.dist(a);
}

#[kani::proof]
fn check_diff() {
    let a = Serial(kani::any());
    let b = Serial(kani::any());
    let _ = a.diff(b);
    let _ = b.diff(a);
}

#[kani::proof]
fn check_add() {
    let _ = Serial(kani::any()) + kani::any();
}

#[kani::proof]
fn check_cmp() {
    let a = Serial(kani::any());
    let b = Serial(kani::any());
    let _ = a.partial_cmp(b);
    let _ = b.partial_cmp(a);
}

#[kani::proof]
fn check_or() {
    let num = Serial(kani::any());
    assert_eq!(num.or(Serial::NAN), num);
    if num.is_nan() {
        assert_eq!(num.or(Serial(5)), Serial(5));
    } else {
        assert_eq!(num.or(Serial(5)), num);
    }
}

#[kani::proof]
fn check_or_default() {
    let num = Serial(kani::any());
    if num.is_nan() {
        assert_eq!(num.or_default(), Serial::default());
    } else {
        assert_eq!(num.or_default(), num);
    }
}

#[kani::proof]
fn check_take() {
    let mut num = Serial(kani::any());
    let num_copy = num;

    assert_eq!(num.take(), num_copy);
    assert_eq!(num, Serial::NAN);
}

#[cfg(feature = "serde")]
// TODO: kani proof has infinite loop
// #[kani::proof]
fn check_serde_json() {
    let expected = Serial(kani::any());
    let encoded = serde_json::to_string(&expected).unwrap();
    let actual: Serial = serde_json::from_str(&encoded).unwrap();
    assert_eq!(expected, actual);
}

#[cfg(feature = "bincode")]
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

#[cfg(feature = "borsh")]
#[kani::proof]
fn check_borsh() {
    use borsh::BorshDeserialize;

    let expected = Serial(kani::any());

    let encoded = borsh::to_vec(&expected).unwrap();
    assert_eq!(2, encoded.len());

    let actual = Serial::try_from_slice(&encoded).unwrap();
    assert_eq!(expected, actual);
}

#[cfg(feature = "bytemuck")]
#[kani::proof]
fn bytemuck_cast_roundtrip() {
    let original = Serial(kani::any());
    let casted: u16 = bytemuck::cast(original);
    let casted_back: Serial = bytemuck::cast(casted);
    assert_eq!(original, casted_back);
}

// TODO: kani proof loops
// #[kani::proof]
#[cfg(feature = "rkyv")]
#[allow(unsafe_code)]
fn check_rkyv() {
    let expected = Serial(kani::any());

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&expected).unwrap();

    let archived = unsafe { rkyv::access_unchecked::<ArchivedSerial>(&bytes[..]) };

    assert_eq!(archived, &expected);
}

// TODO: kani proof loops
// #[kani::proof]
#[cfg(feature = "rkyv")]
fn check_rkyv_safe() {
    let expected = Serial(kani::any());

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&expected).unwrap();

    let archived = rkyv::access::<ArchivedSerial, rkyv::rancor::Error>(&bytes[..]).unwrap();

    assert_eq!(archived, &expected);
}

#[cfg(feature = "speedy")]
#[kani::proof]
fn check_speedy() {
    use speedy::{Readable, Writable};

    let expected = Serial(kani::any());

    let encoded = expected.write_to_vec().unwrap();
    assert_eq!(2, encoded.len());

    let actual = Serial::read_from_buffer(&encoded).unwrap();
    assert_eq!(expected, actual);
}

// TODO: kani proof has infinite loop
// #[kani::proof]
#[cfg(feature = "bitcode")]
fn check_bitcode() {
    let expected = Serial(kani::any());

    let encoded = bitcode::encode(&expected);
    assert_eq!(2, encoded.len());

    let actual: Serial = bitcode::decode(&encoded).unwrap();
    assert_eq!(expected, actual);
}
