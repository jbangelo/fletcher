use generic_fletcher::Fletcher;
use generic_fletcher::FletcherAccumulator;

/// Produces a 64-bit checksum from a stream of 32-bit data.
///
/// Example use:
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

#[cfg(test)]
mod test {
    use super::Fletcher64;
    use byteorder::{ByteOrder, LittleEndian};
    use std::vec::Vec;

    fn run_test(test_data: &[u32], expected_value: &u64) {
        let mut fletcher = Fletcher64::new();
        fletcher.update(test_data);
        assert_eq!(fletcher.value(), *expected_value);
    }

    fn convert_bytes(raw_data: &str) -> Vec<u32> {
        let mut output = Vec::new();
        output.resize(raw_data.len() / 4, 0);
        LittleEndian::read_u32_into(raw_data.as_bytes(), &mut output);
        output
    }

    #[test]
    fn ascii_data() {
        {
            let data = convert_bytes("abcde\0\0\0");
            let expected_value = 0xC8C6C527646362C6;
            run_test(&data, &expected_value);
        }

        {
            let data = convert_bytes("abcdef\0\0");
            let expected_value = 0xC8C72B276463C8C6;
            run_test(&data, &expected_value);
        }

        {
            let data = convert_bytes("abcdefgh");
            let expected_value = 0x312E2B28CCCAC8C6;
            run_test(&data, &expected_value);
        }
    }

    #[test]
    fn fletcher64_underflow() {
        let zeros = vec![0; 200000];
        let expected_result = 0xffffffffffffffff;
        run_test(&zeros, &expected_result);
    }

    #[test]
    fn fletcher64_overflow() {
        let zeros = vec![0xffffffff; 200000];
        let expected_result = 0xffffffffffffffff;
        run_test(&zeros, &expected_result);
    }
}
