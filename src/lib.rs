//! A dependency free implementation of the Fletcher's checksum algorithm
//!
//! Fletcher's Checksum is a checksum algorithm that was developed to provide
//! nearly the same error checking capability as a CRC but with a faster
//! software implementation. This is not a cryptographically secure checksum,
//! it's only meant to be used for checking the integrity of data NOT the
//! authenticity.
//!
//! ### Algorithm Pros
//! This algorithm is faster to run in software than most CRCs. This is
//! because the CRC algorithm was originally designed to be simple to implement
//! in hardware, but not neccesarily in software. The Fletcher Checksum was
//! designed specifically to be suited for implementation in software.
//!
//! ### Algorithm Cons
//! This checksum algorithm does suffer from not being able to distinguish 0xFF
//! from 0x00. Meaning a block of data with all bits set to 1 will have the
//! exact same the same checksum as a block of data with all bits set to 0. This
//! comes from the fact that the algorithm uses one's complement math.
//!
//! Fletcher's checksum isn't quite as good at detecting bit errors in data as
//! a CRC with a well choosen polynomial.
//!
//! # How To Use
//! The checksum objects take in slices of data to process. There is no minimum
//! length required of the slices, all of the provided data will be processed
//! to completion. The type of the input data is dictated by the size of the
//! checksum value. i.e. a 64-bit checksum operates on 32-bit wide values.
//!
//! The checksum object can be queried for it's current checksum value as any
//! time with the `value()` function.
//!
//! # Example
//! ```
//! let data: [u8; 6] = [0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];
//! let mut checksum = fletcher::Fletcher16::new();
//! checksum.update(&data);
//! assert_eq!(checksum.value(), 0x3FAD);
//! ```

#![no_std]

mod fletcher16;
mod fletcher32;
mod fletcher64;
pub mod generic_fletcher;

pub use fletcher16::Fletcher16;
pub use fletcher32::Fletcher32;
pub use fletcher64::Fletcher64;

/// Get the 16-bit checksum in one shot
pub fn fletcher16(data: &[u8]) -> u16 {
    let mut checksum = Fletcher16::new();
    checksum.update(data);
    checksum.value()
}

/// Get the 32-bit checksum in one shot
pub fn fletcher32(data: &[u16]) -> u32 {
    let mut checksum = Fletcher32::new();
    checksum.update(data);
    checksum.value()
}

/// Get the 64-bit checksum in one shot
pub fn fletcher64(data: &[u32]) -> u64 {
    let mut checksum = Fletcher64::new();
    checksum.update(data);
    checksum.value()
}

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
extern crate byteorder;
