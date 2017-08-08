use generic_fletcher::Fletcher;
use generic_fletcher::FletcherSum;

pub type Fletcher32 = Fletcher<u32, u16>;

impl FletcherSum<u16> for u32 {
    fn max_chunk_size() -> usize {
        359
    }

    fn combine(lower: &Self, upper: &Self) -> Self {
        lower | (upper << 16)
    }

    fn reduce(self) -> Self {
        (self & 0xffff) + (self >> 16)
    }
}
