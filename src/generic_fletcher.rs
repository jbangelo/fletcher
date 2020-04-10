//! A generic implementation of the Fletcher Checksum algorithm
//!
//! This module defines a trait for the checksum data and implements
//! the checksum algorithm in therms of that trait. The trait imposes
//! some basic requirments on the type used for the checksum, all of
//! which are met by integral types.

use core::marker::Copy;
use core::marker::PhantomData;
use core::ops::Add;

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
