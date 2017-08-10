use generic_fletcher::Fletcher;
use generic_fletcher::FletcherAccumulator;

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
