//! An implementation of the 16-bit Fletcher checksum. Simply
//! a specialization of the generic fletcher trait.

use generic_fletcher::Fletcher;
use generic_fletcher::FletcherAccumulator;

/// Aliases the generic fletcher struct for normal use of the 16-bit algorithm
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
