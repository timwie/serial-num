#![no_main]

use libfuzzer_sys::fuzz_target;
use serial_num::Serial;

fuzz_target!(|serial: Serial| {
    let mut a = serial;
    let mut b = serial;
    let mut c = serial;

    a.increase();
    let _ = b.increase_get();
    let _ = c.get_increase();

    assert_eq!(a, b);
    assert_eq!(a, c);

    assert!(!serial.is_nan() || a.is_nan());
    assert!(!serial.is_nan() || b.is_nan());
    assert!(!serial.is_nan() || c.is_nan());

    assert!(serial.is_nan() || serial < a);
    assert!(serial.is_nan() || serial < b);
    assert!(serial.is_nan() || serial < c);

    assert!(serial.is_nan() || a.dist(serial) == 1);
    assert!(serial.is_nan() || b.dist(serial) == 1);
    assert!(serial.is_nan() || c.dist(serial) == 1);

    assert!(serial.is_nan() || a.diff(serial) == 1);
    assert!(serial.is_nan() || b.diff(serial) == 1);
    assert!(serial.is_nan() || c.diff(serial) == 1);

    assert!(serial.is_nan() || serial.diff(a) == -1);
    assert!(serial.is_nan() || serial.diff(b) == -1);
    assert!(serial.is_nan() || serial.diff(c) == -1);
});
