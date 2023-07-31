use crate::Serial;

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
    let _ = a.partial_cmp(&b);
    let _ = a.cmp(&b);
    let _ = b.partial_cmp(&a);
    let _ = b.cmp(&a);
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
    use borsh::{BorshDeserialize, BorshSerialize};

    let expected = Serial(kani::any());

    let encoded = expected.try_to_vec().unwrap();
    assert_eq!(2, encoded.len());

    let actual = Serial::try_from_slice(&encoded).unwrap();
    assert_eq!(expected, actual);
}

#[cfg(feature = "rkyv")]
// TODO #[kani::proof]
fn check_rkyv() {
    let expected = Serial(kani::any());

    let bytes = rkyv::to_bytes::<_, 256>(&expected).unwrap();

    let actual = unsafe { rkyv::archived_root::<Serial>(&bytes[..]) };
    assert_eq!(actual, &expected);
}

#[cfg(feature = "rkyv-safe")]
// TODO #[kani::proof]
fn check_rkyv_safe() {
    let expected = Serial(kani::any());

    let bytes = rkyv::to_bytes::<_, 256>(&expected).unwrap();

    let actual = rkyv::check_archived_root::<Serial>(&bytes[..]).unwrap();
    assert_eq!(actual, &expected);
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

#[cfg(feature = "bitcode")]
#[kani::proof]
fn check_bitcode() {
    let expected = Serial(kani::any());

    let encoded = bitcode::encode(&expected).unwrap();
    assert_eq!(2, encoded.len());

    let actual: Serial = bitcode::decode(&encoded).unwrap();
    assert_eq!(expected, actual);
}
