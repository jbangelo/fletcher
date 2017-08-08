use generic_fletcher::Fletcher;
use generic_fletcher::FletcherSum;

pub type Fletcher64 = Fletcher<u64, u32>;

impl FletcherSum<u32> for u64 {
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
