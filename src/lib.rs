//! # fletcher
//!
//! A dependency free implementation of the Fletcher's checksum algorithm
//!
//! ## Getting Started
//!
//! If you've got all the data you want a checksum of you can use the [`calc_fletcher16()`],
//! [`calc_fletcher32()`], and [`calc_fletcher64()`] functions to get the checksum values in
//! a single shot.
//!
//! If you are streaming data, or otherwise want to calculate the checksum in chunks you can
//! use the [`Fletcher16`], [`Fletcher32`], and [`Fletcher64`] objects to manage the
//! intermediate state. You can feed in data with the [`Fletcher::update()`] function and
//! get the current checksum value at anytime with the [`Fletcher::value()`] function. These
//! objects also allow you to initialize them with specific values using the
//! [`Fletcher::with_initial_values()`] constructor function.
//!
//! ## Check Values
//!
//! The typical use case for checksums is to generate the value and store it alongside the data,
//! then when you want to validate the integrity of the data you re-generate the checksum and
//! compare it against the stored value. An alternative method is to append special values to the
//! data to ensure that the resultant checksum value ends up being zero. This method does not
//! provide any more or less ability to detect corruption of the data and it doesn't save any space
//! since the check values that are added are the same size as the checksum value that would need
//! to be stored.
//!
//! The [`checkvalues_fletcher16()`], [`checkvalues_fletcher32()`], and
//! [`checkvalues_fletcher64()`] functions provide a one-shot means of generating the needed check
//! vlaues to force the checksum to be zero. Alternatively the [`Fletcher::check_values()`]
//! function is available if you are using the [`Fletcher`] objects.
//!
//! ## Examples
//!
//! Generating checksums:
//! ```rust
//! let data: [u8; 6] = [0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];
//! assert_eq!(fletcher::calc_fletcher16(&data), 0x3FAD);
//! // Or if you want to work on smaller chunks of data
//! let mut checksum = fletcher::Fletcher16::new();
//! checksum.update(&data[0..3]);
//! checksum.update(&data[3..]);
//! assert_eq!(checksum.value(), 0x3FAD);
//! ```
//!
//! Getting check values to force a zero checksum:
//! ```rust
//! let mut data = vec![0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];
//! let checkvalues = fletcher::checkvalues_fletcher16(&data);
//! data.push(checkvalues[0]);
//! data.push(checkvalues[1]);
//! assert_eq!(fletcher::calc_fletcher16(&data), 0);
//! ```

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
extern crate byteorder;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use core::{
    cmp::PartialEq,
    convert::{From, TryInto},
    fmt::Debug,
    ops::{Add, AddAssign, BitAnd, BitOr, Shl, Shr, Sub},
};

/// Base set of values and operations needed for our implementation
pub trait FletcherAccumulator:
    Sized
    + Copy
    + Default
    + From<Self::InputType>
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + Shl<u16, Output = Self>
    + Shr<u16, Output = Self>
    + PartialEq
    + TryInto<Self::InputType>
{
    type InputType: Copy;

    /// The maximum summations that can happen before accumulator
    /// overflows. This can be determined by putting the maximum
    /// word value into the algorithm and counting the number of
    /// words can be added before an overflow occurs.
    const MAX_CHUNK_SIZE: usize;

    /// Bit masking pattern to use in the reduce step. This should
    /// mask out the least significant half of the value, i.e. `0x00ff` for
    /// 16 bit values
    const BIT_MASK: Self;

    /// The number of bit spaces needed to shift the most significant half
    /// of the value into the least significant half of the value. This is
    /// typically half the bit width of the type, i.e. 8 for 16 bit values
    const SHIFT_AMOUNT: u16;
}

impl FletcherAccumulator for u16 {
    type InputType = u8;
    const BIT_MASK: Self = 0x00ff;
    const MAX_CHUNK_SIZE: usize = 21;
    const SHIFT_AMOUNT: u16 = 8;
}

impl FletcherAccumulator for u32 {
    type InputType = u16;
    const BIT_MASK: Self = 0x0000ffff;
    const MAX_CHUNK_SIZE: usize = 360;
    const SHIFT_AMOUNT: u16 = 16;
}

impl FletcherAccumulator for u64 {
    type InputType = u32;
    const BIT_MASK: Self = 0x00000000ffffffff;
    const MAX_CHUNK_SIZE: usize = 92680;
    const SHIFT_AMOUNT: u16 = 32;
}

/// Type to hold the state for calculating a fletcher checksum.
///
/// This is useful if you want to calculate the checksum over several small
/// chunks of data. If you have an entire block of data the functions
/// [`calc_fletcher16`], [`calc_fletcher32`], [`calc_fletcher64`] simplify
/// the process.
#[derive(Clone, Copy, Debug)]
pub struct Fletcher<T>
where
    T: FletcherAccumulator,
{
    a: T,
    b: T,
}

impl<T> Fletcher<T>
where
    T: FletcherAccumulator,
{
    /// Construct a new checksum object using the default initial value
    pub fn new() -> Fletcher<T> {
        Fletcher {
            a: T::default(),
            b: T::default(),
        }
    }

    /// Construct a new checksum object with a specific set of initial values
    pub fn with_initial_values(
        a: <T as FletcherAccumulator>::InputType,
        b: <T as FletcherAccumulator>::InputType,
    ) -> Fletcher<T> {
        Fletcher {
            a: a.into(),
            b: b.into(),
        }
    }

    /// Updates the checksum with the given input data
    pub fn update(&mut self, data: &[<T as FletcherAccumulator>::InputType]) {
        for chunk in data.chunks(<T as FletcherAccumulator>::MAX_CHUNK_SIZE) {
            let mut intermediate_a = self.a;
            let mut intermediate_b = self.b;

            for element in chunk {
                intermediate_a += (*element).into();
                intermediate_b += intermediate_a;
            }

            self.a = Self::reduce(intermediate_a);
            self.b = Self::reduce(intermediate_b);
        }

        // One last reduction must be done since we  process in chunks
        self.a = Self::reduce(self.a);
        self.b = Self::reduce(self.b);
    }

    /// Returns the current checksum value
    pub fn value(&self) -> T {
        Self::combine(self.a, self.b)
    }

    pub fn check_values(&self) -> [T::InputType; 2]
    where
        <T as TryInto<<T as FletcherAccumulator>::InputType>>::Error: Debug,
    {
        let checksum_a: T = T::BIT_MASK - Self::reduce(self.a + self.b);
        let checksumb_b: T = T::BIT_MASK - Self::reduce(self.a + checksum_a);

        [
            checksum_a.try_into().unwrap(),
            checksumb_b.try_into().unwrap(),
        ]
    }

    /// Combines the two accumulator values into a single value
    ///
    /// This function assumes that the accumulators have already
    /// been fully reduced.
    fn combine(lower: T, upper: T) -> T {
        lower | (upper << T::SHIFT_AMOUNT)
    }

    /// Reduces the accumulator value
    ///
    /// This function needs to reduce the accumulator value in a manner
    /// that rounds the value according to one's compliment math.
    fn reduce(value: T) -> T {
        let lhs: T = value & T::BIT_MASK;
        let rhs: T = value >> T::SHIFT_AMOUNT;
        let result: T = lhs + rhs;
        if result == T::BIT_MASK {
            T::default()
        } else {
            result
        }
    }
}

impl<T> Default for Fletcher<T>
where
    T: FletcherAccumulator,
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
pub type Fletcher16 = Fletcher<u16>;

/// Produces a 32-bit checksum from a stream of 16-bit data.
///
/// # Example
/// ```
/// let data: [u16; 6] = [0xF02A, 0xCB0D, 0x5639, 0x6501, 0x2384, 0x75BB];
/// let mut checksum = fletcher::Fletcher32::new();
/// checksum.update(&data);
/// assert_eq!(checksum.value(), 0xDCF30FB3);
/// ```
pub type Fletcher32 = Fletcher<u32>;

/// Produces a 64-bit checksum from a stream of 32-bit data.
///
/// # Example
/// ```
/// let data: [u32; 6] = [0xA0F15604, 0x82856B93, 0xC4395038, 0xF3CAC9CB, 0x39B7C44B, 0xEB0F23DA];
/// let mut checksum = fletcher::Fletcher64::new();
/// checksum.update(&data);
/// assert_eq!(checksum.value(), 0x9D0768B50041C3C3);
/// ```
pub type Fletcher64 = Fletcher<u64>;

/// Get the 16-bit checksum in one shot
pub fn calc_fletcher16(data: &[u8]) -> u16 {
    let mut checksum = Fletcher16::new();
    checksum.update(data);
    checksum.value()
}

/// Get the 2 bytes to append to the end of data to cause the
/// computed checksum to be be `0`
pub fn checkvalues_fletcher16(data: &[u8]) -> [u8; 2] {
    let mut checksum = Fletcher16::new();
    checksum.update(data);
    checksum.check_values()
}

/// Get the 32-bit checksum in one shot
pub fn calc_fletcher32(data: &[u16]) -> u32 {
    let mut checksum = Fletcher32::new();
    checksum.update(data);
    checksum.value()
}

/// Get the 2 16-bit values to append to the end of data
/// to cause the computed checksum to be be `0`
pub fn checkvalues_fletcher32(data: &[u16]) -> [u16; 2] {
    let mut checksum = Fletcher32::new();
    checksum.update(data);
    checksum.check_values()
}

/// Get the 64-bit checksum in one shot
pub fn calc_fletcher64(data: &[u32]) -> u64 {
    let mut checksum = Fletcher64::new();
    checksum.update(data);
    checksum.value()
}

/// Get the 2 32-bit values to append to the end of data
/// to cause the computed checksum to be be `0`
pub fn checkvalues_fletcher64(data: &[u32]) -> [u32; 2] {
    let mut checksum = Fletcher64::new();
    checksum.update(data);
    checksum.check_values()
}

#[cfg(test)]
mod test {
    use super::*;
    use byteorder::{ByteOrder, LittleEndian};
    use std::vec::Vec;

    fn run_test<T>(test_data: &[<T as FletcherAccumulator>::InputType], expected_value: &T)
    where
        T: FletcherAccumulator + core::cmp::Eq + core::fmt::Debug,
    {
        let mut fletcher = Fletcher::<T>::new();
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
        let expected_result = 0u16;
        run_test(&zeros, &expected_result);
    }

    #[test]
    fn fletcher16_overflow() {
        let ones = vec![0xff; 200000];
        let expected_result = 0x0000u16;
        run_test(&ones, &expected_result);
    }

    #[test]
    fn fletcher16_initial_value() {
        let data = vec![0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];

        let mut defaulted_checksum = Fletcher16::new();
        defaulted_checksum.update(&data);

        let intermediate_value = defaulted_checksum.value();
        let mut initial_value_checksum = Fletcher16::with_initial_values(
            (intermediate_value & 0xFF) as u8,
            (intermediate_value >> 8) as u8,
        );

        assert_eq!(defaulted_checksum.value(), initial_value_checksum.value());

        defaulted_checksum.update(&data);
        initial_value_checksum.update(&data);

        assert_eq!(defaulted_checksum.value(), initial_value_checksum.value());
    }

    quickcheck! {
        fn fletcher16_check_values(data: Vec<u8>) -> bool {
            let check_bytes = checkvalues_fletcher16(&data);
            let mut data = data.clone();
            data.push(check_bytes[0]);
            data.push(check_bytes[1]);

            calc_fletcher16(&data) == 0
        }
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
        let expected_result = 0u32;
        run_test(&zeros, &expected_result);
    }

    #[test]
    fn fletcher32_overflow() {
        let ones = vec![0xffff; 200000];
        let expected_result = 0x00000000u32;
        run_test(&ones, &expected_result);
    }

    #[test]
    fn fletcher32_initial_value() {
        let data = vec![0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];

        let mut defaulted_checksum = Fletcher32::new();
        defaulted_checksum.update(&data);

        let intermediate_value = defaulted_checksum.value();
        let mut initial_value_checksum = Fletcher32::with_initial_values(
            (intermediate_value & 0xFFFF) as u16,
            (intermediate_value >> 16) as u16,
        );

        assert_eq!(defaulted_checksum.value(), initial_value_checksum.value());

        defaulted_checksum.update(&data);
        initial_value_checksum.update(&data);

        assert_eq!(defaulted_checksum.value(), initial_value_checksum.value());
    }

    quickcheck! {
        fn fletcher32_check_values(data: Vec<u16>) -> bool {
            let check_words = checkvalues_fletcher32(&data);
            let mut data = data.clone();
            data.push(check_words[0]);
            data.push(check_words[1]);

            calc_fletcher32(&data) == 0
        }
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
        let expected_result = 0u64;
        run_test(&zeros, &expected_result);
    }

    #[test]
    fn fletcher64_overflow() {
        let zeros = vec![0xffffffff; 200000];
        let expected_result = 0x0000000000000000u64;
        run_test(&zeros, &expected_result);
    }

    #[test]
    fn fletcher64_initial_value() {
        let data = vec![0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];

        let mut defaulted_checksum = Fletcher64::new();
        defaulted_checksum.update(&data);

        let intermediate_value = defaulted_checksum.value();
        let mut initial_value_checksum = Fletcher64::with_initial_values(
            (intermediate_value & 0xFFFFFFFF) as u32,
            (intermediate_value >> 32) as u32,
        );

        assert_eq!(defaulted_checksum.value(), initial_value_checksum.value());

        defaulted_checksum.update(&data);
        initial_value_checksum.update(&data);

        assert_eq!(defaulted_checksum.value(), initial_value_checksum.value());
    }

    quickcheck! {
        fn fletcher64_check_values(data: Vec<u32>) -> bool {
            let check_words = checkvalues_fletcher64(&data);
            let mut data = data.clone();
            data.push(check_words[0]);
            data.push(check_words[1]);

            calc_fletcher64(&data) == 0
        }
    }

    #[test]
    fn issue_8() {
        let data = [0x06, 0x83, 0x0d, 0x00, 0xc5, 0x18, 0xe5, 0x08, 0xef, 0x11];
        let csum = super::calc_fletcher16(&data);

        assert_ne!(csum, 0xff63);
        assert_eq!(csum, 0x0063);
    }

    #[test]
    fn wiki_example() {
        let data = [0x01, 0x02, 0xF8, 0x04];
        let csum = super::calc_fletcher16(&data);

        assert_eq!(csum, 0x0000);
    }
}
