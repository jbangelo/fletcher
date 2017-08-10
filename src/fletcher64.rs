//! An implementation of the 64-bit Fletcher checksum. Simply
//! a specialization of the generic fletcher trait.

use generic_fletcher::Fletcher;
use generic_fletcher::FletcherAccumulator;

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
