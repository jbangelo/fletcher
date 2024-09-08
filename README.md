# fletcher

A dependency free implementation of the Fletcher's checksum algorithm

[![crates.io](https://img.shields.io/crates/v/fletcher.svg)](https://crates.io/crates/fletcher)
[![docs.rs](https://img.shields.io/docsrs/fletcher)](https://docs.rs/fletcher/latest/fletcher/)
[![Build Status](https://github.com/jbangelo/fletcher/actions/workflows/build.yml/badge.svg)](https://github.com/jbangelo/fletcher)
[![Crates.io License](https://img.shields.io/crates/l/fletcher)](https://github.com/jbangelo/fletcher/blob/main/LICENSE)

Fletcher's Checksum was developed to provide nearly the same error checking
capability as a CRC but with a faster software implementation. This is not a
cryptographically secure checksum, it's only meant to be used for checking
the integrity of data NOT the authenticity.

## Algorithm Pros ✅

This algorithm is faster to run in software than most CRCs. This is because the
CRC algorithm was originally designed to be simple to implement in hardware, but
not neccesarily in software. The Fletcher Checksum was designed specifically to
be suited for implementation in software.

## Algorithm Cons ❌

This checksum algorithm does suffer from not being able to distinguish `0xFF`
from `0x00`. Meaning a block of data with all bits set to 1 will have the exact
same the same checksum as a block of data with all bits set to 0. This comes
from the fact that the algorithm uses one's complement math.

Fletcher's checksum isn't quite as good at detecting bit errors in data as a CRC
with a well choosen polynomial.

## How To Use

If you have an entire block of data you want to get the checksum of you can
use the calc functions (`calc_fletcher16`, `calc_fletcher32`, `calc_fletcher64`)
to get the checksum in a single function call.

If you are getting the data in chunks you can make a `Fletcher` object
(`Fletcher16`, `Fletcher32`, `Fletcher64`) to  manage the intermediate
state between chunks of data. The checksum objects take in slices of data to
process. There is no minimum length required of the slices, all of the provided
data will be processed to completion. The type of the input data is dictated by
the size of the checksum value. i.e. a 64-bit checksum operates on 32-bit wide
values.

The checksum object can be queried for it's current checksum value as any
time with the `Fletcher::value()` function.

### Example

```rust
let data: [u8; 6] = [0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];
assert_eq!(fletcher::calc_fletcher16(&data), 0x3FAD);
// Or if you want to work on smaller chunks of data
let mut checksum = fletcher::Fletcher16::new();
checksum.update(&data[0..3]);
checksum.update(&data[3..]);
assert_eq!(checksum.value(), 0x3FAD);
```

### Check Bytes

It's trivial to determine the additional values needed to cause the Fletcher
checksum produced to be zero. These are sometimes called check bytes or check
values. This crate provides functions to generate these additional values to
append to the data, they are the `Fletcher::check_values()` if you use the
`Fletcher` object or the `checkvalues_fletcher16()`, `checkvalues_fletcher32()`,
`checkvalues_fletcher64()`.

