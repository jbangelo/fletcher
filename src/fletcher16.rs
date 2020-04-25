use generic_fletcher::Fletcher;
use generic_fletcher::FletcherAccumulator;

/// Produces a 16-bit checksum from a stream of 8-bit data.
///
/// # Example
/// ```
/// let data: [u8; 6] = [0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];
/// let mut checksum = fletcher::Fletcher16::new();
/// checksum.update(&data);
/// assert_eq!(checksum.value(), 0x3FAD);
/// ```
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

#[cfg(test)]
mod test {
    use super::Fletcher16;

    fn run_test(test_data: &[u8], expected_value: &u16) {
        let mut fletcher = Fletcher16::new();
        fletcher.update(test_data);
        assert_eq!(fletcher.value(), *expected_value);
    }

    #[test]
    fn ascii_data() {
        {
            let test_data = "abcde";
            let expected_result = 0xC8F0;
            run_test(test_data.as_bytes(), &expected_result);
        }

        {
            let test_data = "abcdef";
            let expected_result = 0x2057;
            run_test(test_data.as_bytes(), &expected_result);
        }

        {
            let test_data = "abcdefgh";
            let expected_result = 0x0627;
            run_test(test_data.as_bytes(), &expected_result);
        }
    }

    #[test]
    fn fletcher16_test() {
        let data = vec![0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];
        let expected_result = 0x3fad;
        run_test(&data, &expected_result);
    }

    #[test]
    fn fletcher16_underflow() {
        let zeros = vec![0; 200000];
        let expected_result = 0xffff;
        run_test(&zeros, &expected_result);
    }

    #[test]
    fn fletcher16_overflow() {
        let ones = vec![0xff; 200000];
        let expected_result = 0xffff;
        run_test(&ones, &expected_result);
    }
}
