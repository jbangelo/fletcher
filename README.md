# fletcher
A dependency free implementation of the Fletcher's checksum algorithm

[![crates.io](https://img.shields.io/crates/v/fletcher.svg)](https://crates.io/crates/fletcher) ![example workflow](https://github.com/jbangelo/fletcher/actions/workflows/build.yml/badge.svg)

[Fletcher's Checksum](https://en.wikipedia.org/wiki/Fletcher%27s_checksum) is a
checksum algorithm that was developed to provide nearly the same error checking
capability as a CRC but with a faster software implementation. This is not a
cryptographically secure checksum, it's only meant to be used for checking the
integrity of data __NOT__ the authenticity.

## Algorithm Pros
This algorithm is faster to run in software than most CRCs. This is because the
CRC algorithm was originally designed to be simple to implement in hardware, but
not neccesarily in software. The Fletcher Checksum was designed specifically to
be suited for implementation in software.

## Algorithm Cons
This checksum algorithm does suffer from not being able to distinguish `0xFF`
from `0x00`. Meaning a block of data with all bits set to 1 will have the exact
same the same checksum as a block of data with all bits set to 0. This comes
from the fact that the algorithm uses one's complement math.

Fletcher's checksum isn't quite as good at detecting bit errors in data as a CRC
with a well choosen polynomial.

## Installation
Simply add it to the dependency list in your Cargo.toml
```toml
[dependencies]
fletcher = "0.1"
```