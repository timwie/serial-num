#![no_main]

use libfuzzer_sys::fuzz_target;
use serial_num::Serial;

fuzz_target!(|tuple: (Serial, u16)| {
    let (serial, n) = tuple;
    let _ = serial + n;
});
