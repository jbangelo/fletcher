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

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
extern crate byteorder;

use core::{marker::{Copy, PhantomData}, ops::Add};

/// Defines the required traits for the accumulator type
/// used in the algorithm
pub trait FletcherAccumulator<T>: Add<Self> + From<T> + From<<Self as Add>::Output> + Copy {
    /// Should return a reasonable default value
    ///
    /// Usual default values have the least significant bits set
    /// and the most significant bits cleared, i.e. 0x00ff
    fn default_value() -> Self;
    /// Should return the maximum number of words to sum before reducing
    ///
    /// This value should be the maximum summations that can happen before
    /// either accumulator overflows. This can be determined by
    /// putting the maximum word value into the algorithm and counting
    /// the number of words can be added before an overflow occurs.
    fn max_chunk_size() -> usize;
    /// Combines the two accumulator values into a single value
    ///
    /// This function can assume that the accumulators have already
    /// been fully reduced. This usually involves simply shifting
    /// the upper accumulator value into the MSB
    fn combine(lower: &Self, upper: &Self) -> Self;
    /// Reduces the accumulator value
    ///
    /// This function needs to reduce the accumulator value in a manner
    /// that rounds the value according to one's compliment math. This
    /// is usually accomplished with masking and shifting
    fn reduce(self) -> Self;
}

/// A generic type for holding intermediate checksum values
pub struct Fletcher<T, U> {
    a: T,
    b: T,
    phantom: PhantomData<U>,
}

impl<T, U> Fletcher<T, U>
    where
        T: FletcherAccumulator<U>,
        U: Copy,
{
    pub fn new() -> Fletcher<T, U> {
        Fletcher {
            a: T::default_value(),
            b: T::default_value(),
            phantom: PhantomData,
        }
    }

    /// The core fletcher checksum algorithm
    ///
    /// The input data is processed in chunks which reduces the
    /// number of calls to `reduce()`. The size of the chunks depends
    /// on the accumulator size and data size.
    pub fn update(&mut self, data: &[U]) {
        let max_chunk_size = T::max_chunk_size();

        for chunk in data.chunks(max_chunk_size) {
            let mut intermediate_a = self.a;
            let mut intermediate_b = self.b;

            for byte in chunk {
                intermediate_a = T::from(intermediate_a + T::from(*byte));
                intermediate_b = T::from(intermediate_b + intermediate_a);
            }

            self.a = intermediate_a.reduce();
            self.b = intermediate_b.reduce();
        }

        // One last reduction must be done since we  process in chunks
        self.a = self.a.reduce();
        self.b = self.b.reduce();
    }

    /// Returns the current checksum value of the `Fletcher` object
    pub fn value(&self) -> T {
        T::combine(&self.a, &self.b)
    }
}

impl<T, U> Default for Fletcher<T, U>
    where
        T: FletcherAccumulator<U>,
        U: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Produces a 16-bit checksum from a stream of 8-bit data.
///
/// # Example
/// ```
/// let data: [u8; 6] = [0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];
/// let mut checksum = fletcher::Fletcher16::new();
/// checksum.update(&data);
/// assert_eq!(checksum.value(), 0x3FAD);
/// ```
pub type Fletcher16 = Fletcher<u16, u8>;

impl FletcherAccumulator<u8> for u16 {
    fn default_value() -> Self {
        0x00ff
    }

    fn max_chunk_size() -> usize {
        21
    }

    fn combine(lower: &Self, upper: &Self) -> Self {
        lower | (upper << 8)
    }

    fn reduce(self) -> Self {
        (self & 0xff) + (self >> 8)
    }
}

/// Produces a 32-bit checksum from a stream of 16-bit data.
///
/// # Example
/// ```
/// let data: [u16; 6] = [0xF02A, 0xCB0D, 0x5639, 0x6501, 0x2384, 0x75BB];
/// let mut checksum = fletcher::Fletcher32::new();
/// checksum.update(&data);
/// assert_eq!(checksum.value(), 0xDCF30FB3);
/// ```
pub type Fletcher32 = Fletcher<u32, u16>;

impl FletcherAccumulator<u16> for u32 {
    fn default_value() -> Self {
        0x0000ffff
    }

    fn max_chunk_size() -> usize {
        360
    }

    fn combine(lower: &Self, upper: &Self) -> Self {
        lower | (upper << 16)
    }

    fn reduce(self) -> Self {
        (self & 0xffff) + (self >> 16)
    }
}

/// Produces a 64-bit checksum from a stream of 32-bit data.
///
/// # Example
/// ```
/// let data: [u32; 6] = [0xA0F15604, 0x82856B93, 0xC4395038, 0xF3CAC9CB, 0x39B7C44B, 0xEB0F23DA];
/// let mut checksum = fletcher::Fletcher64::new();
/// checksum.update(&data);
/// assert_eq!(checksum.value(), 0x9D0768B50041C3C3);
/// ```
pub type Fletcher64 = Fletcher<u64, u32>;

impl FletcherAccumulator<u32> for u64 {
    fn default_value() -> Self {
        0x00000000ffffffff
    }

    fn max_chunk_size() -> usize {
        92680
    }

    fn combine(lower: &Self, upper: &Self) -> Self {
        lower | (upper << 32)
    }

    fn reduce(self) -> Self {
        (self & 0xffffffff) + (self >> 32)
    }
}

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
mod test {
    use super::{Fletcher, FletcherAccumulator};
    use byteorder::{ByteOrder, LittleEndian};
    use std::vec::Vec;

    fn run_test<T, U>(test_data: &[U], expected_value: &T)
    where
        T: FletcherAccumulator<U> + core::cmp::Eq + core::fmt::Debug,
        U: Copy
    {
        let mut fletcher = Fletcher::<T, U>::new();
        fletcher.update(test_data);
        assert_eq!(fletcher.value(), *expected_value);
    }

    #[test]
    fn fletcher16_ascii_data() {
        {
            let test_data = "abcde";
            let expected_result = 0xC8F0u16;
            run_test(test_data.as_bytes(), &expected_result);
        }

        {
            let test_data = "abcdef";
            let expected_result = 0x2057u16;
            run_test(test_data.as_bytes(), &expected_result);
        }

        {
            let test_data = "abcdefgh";
            let expected_result = 0x0627u16;
            run_test(test_data.as_bytes(), &expected_result);
        }
    }

    #[test]
    fn fletcher16_test() {
        let data = vec![0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];
        let expected_result = 0x3fadu16;
        run_test(&data, &expected_result);
    }

    #[test]
    fn fletcher16_underflow() {
        let zeros = vec![0; 200000];
        let expected_result = 0xffffu16;
        run_test(&zeros, &expected_result);
    }

    #[test]
    fn fletcher16_overflow() {
        let ones = vec![0xff; 200000];
        let expected_result = 0xffffu16;
        run_test(&ones, &expected_result);
    }

    fn convert_bytes_u16(raw_data: &str) -> Vec<u16> {
        let mut output = Vec::new();
        output.resize(raw_data.len() / 2, 0);
        LittleEndian::read_u16_into(raw_data.as_bytes(), &mut output);
        output
    }

    #[test]
    fn fletcher32_ascii_data() {
        {
            let data = convert_bytes_u16("abcde\0");
            let expected_value = 0xF04FC729u32;
            run_test(&data, &expected_value);
        }

        {
            let data = convert_bytes_u16("abcdef");
            let expected_value = 0x56502D2Au32;
            run_test(&data, &expected_value);
        }

        {
            let data = convert_bytes_u16("abcdefgh");
            let expected_value = 0xEBE19591u32;
            run_test(&data, &expected_value);
        }
    }

    #[test]
    fn fletcher32_test() {
        let data = vec![0xF02A, 0xCB0D, 0x5639, 0x6501, 0x2384, 0x75BB];
        let expected_result = 0xdcf30fb3u32;
        run_test(&data, &expected_result);
    }

    #[test]
    fn fletcher32_underflow() {
        let zeros = vec![0; 200000];
        let expected_result = 0xffffffffu32;
        run_test(&zeros, &expected_result);
    }

    #[test]
    fn fletcher32_overflow() {
        let ones = vec![0xffff; 200000];
        let expected_result = 0xffffffffu32;
        run_test(&ones, &expected_result);
    }

    fn convert_bytes_u32(raw_data: &str) -> Vec<u32> {
        let mut output = Vec::new();
        output.resize(raw_data.len() / 4, 0);
        LittleEndian::read_u32_into(raw_data.as_bytes(), &mut output);
        output
    }

    #[test]
    fn fletcher64_ascii_data() {
        {
            let data = convert_bytes_u32("abcde\0\0\0");
            let expected_value = 0xC8C6C527646362C6u64;
            run_test(&data, &expected_value);
        }

        {
            let data = convert_bytes_u32("abcdef\0\0");
            let expected_value = 0xC8C72B276463C8C6u64;
            run_test(&data, &expected_value);
        }

        {
            let data = convert_bytes_u32("abcdefgh");
            let expected_value = 0x312E2B28CCCAC8C6u64;
            run_test(&data, &expected_value);
        }
    }

    #[test]
    fn fletcher64_underflow() {
        let zeros = vec![0; 200000];
        let expected_result = 0xffffffffffffffffu64;
        run_test(&zeros, &expected_result);
    }

    #[test]
    fn fletcher64_overflow() {
        let zeros = vec![0xffffffff; 200000];
        let expected_result = 0xffffffffffffffffu64;
        run_test(&zeros, &expected_result);
    }
}
