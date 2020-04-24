//! An implementation of the 32-bit Fletcher checksum. Simply
//! a specialization of the generic fletcher trait.

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

#[cfg(test)]
mod test {
    use super::Fletcher32;
    use byteorder::{ByteOrder, LittleEndian};
    use std::vec::Vec;

    fn run_test(test_data: &[u16], expected_value: &u32) {
        let mut fletcher = Fletcher32::new();
        fletcher.update(test_data);
        assert_eq!(fletcher.value(), *expected_value);
    }

    fn convert_bytes(raw_data: &str) -> Vec<u16> {
        let mut output = Vec::new();
        output.resize(raw_data.len() / 2, 0);
        LittleEndian::read_u16_into(raw_data.as_bytes(), &mut output);
        output
    }

    #[test]
    fn ascii_data() {
        {
            let data = convert_bytes("abcde\0");
            let expected_value = 0xF04FC729;
            run_test(&data, &expected_value);
        }

        {
            let data = convert_bytes("abcdef");
            let expected_value = 0x56502D2A;
            run_test(&data, &expected_value);
        }

        {
            let data = convert_bytes("abcdefgh");
            let expected_value = 0xEBE19591;
            run_test(&data, &expected_value);
        }
    }

    #[test]
    fn fletcher32_test() {
        let data = vec![0xF02A, 0xCB0D, 0x5639, 0x6501, 0x2384, 0x75BB];
        let expected_result = 0xdcf30fb3;
        run_test(&data, &expected_result);
    }

    #[test]
    fn fletcher32_underflow() {
        let zeros = vec![0; 200000];
        let expected_result = 0xffffffff;
        run_test(&zeros, &expected_result);
    }

    #[test]
    fn fletcher32_overflow() {
        let ones = vec![0xffff; 200000];
        let expected_result = 0xffffffff;
        run_test(&ones, &expected_result);
    }
}
