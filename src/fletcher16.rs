use generic_fletcher::Fletcher;
use generic_fletcher::FletcherSum;

pub type Fletcher16 = Fletcher<u16, u8>;

impl FletcherSum<u8> for u16 {
    fn max_chunk_size() -> usize {
        20
    }

    fn combine(lower: &Self, upper: &Self) -> Self {
        lower | (upper << 8)
    }

    fn reduce(self) -> Self {
        (self & 0xff) + (self >> 8)
    }
}
