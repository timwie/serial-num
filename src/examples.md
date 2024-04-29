### Simple example
```rust
use serial_num::Serial;

// the default is a reference point - not serial number "zero"
let mut a = Serial::default();
let mut b = Serial::default();
let mut c = Serial::default();

// three ways to increase
let x = a.increase_get(); // increase, then copy
let y = b.get_increase(); // copy, then increase
c.increase();

assert!(y.precedes(x));
assert_eq!(-1_i16, y.diff(x)); // "diff()" is signed
assert_eq!(1_u16, y.dist(x)); // "dist()" is unsigned

// addition is the same as calling "increase()" n times
assert_eq!(y + 1_u16, x);
```

### Wraparound example
```rust
use serial_num::Serial;

// a serial number can be increased indefinitely
let mut x = Serial::default();
for _ in 0..u16::MAX {
    x.increase();
}
let x = x + u16::MAX + u16::MAX + u16::MAX;

// comparison is trivial as long as two serial numbers have
// a distance of less than half of our number space (32767).
let a = Serial::default() + 5;
let b = Serial::default() + 32000;
assert!(a.precedes(b)); // 5th successor < 32000th successor

// but: the comparison flips if the distance is larger
let a = Serial::default() + 5;
let b = Serial::default() + 65000;
assert!(a.succeeds(b)); // 5th successor > 65000th successor

// this means that you get the right ordering as long as
// you compare one serial number at most with one that
// is its 32767th successor:
let num = Serial::default();
assert!(num.precedes(num + 32767));     //  0 < 32767 (still intuitive)
assert!(num.succeeds(num + 32768));     //  0 > 32768 (flip #1)
assert!(num.succeeds(num + 65534));     //  0 > 65534
assert!(num == num + 65535);            // 0 == 65535 (due to same internal representation)
assert!(num.precedes(num + 65535 + 1)); //  0 < 65536 (flip #2)
```

### The `NAN` value
```rust
use serial_num::Serial;

// "NAN" exists to have value representing "no serial number",
// since it saves encoding space vs wrapping Serial in an Option.
let nan = Serial::NAN;
let num = Serial::default();

// you can check whether a serial number is NAN
assert!(nan.is_nan());

// NAN cannot be increased
assert_eq!(Serial::NAN, nan + 1_u16);

// distance between two NAN values is zero
assert_eq!(0_u16, nan.dist(nan));
assert_eq!(0_i16, nan.diff(nan));

// distance and difference of non-NAN to NAN is the maximum distance
assert_eq!(32_767_u16, num.dist(nan));
assert_eq!(32_767_u16, nan.dist(num));
assert_eq!(32_767_i16, num.diff(nan));
assert_eq!(32_767_i16, nan.diff(num));

// partial ordering does not include the NAN value
assert_eq!(None, nan.partial_cmp(num));
assert!(!nan.precedes(num) && !nan.succeeds(num));
```
