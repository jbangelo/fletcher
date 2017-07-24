pub mod fletcher16;
pub mod fletcher32;

#[cfg(test)]
mod tests {
    #[test]
    fn fletcher16_test() {
        let data = vec![0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];
        let mut checksum = super::fletcher16::Fletcher16::new();

        checksum.update(&data);
        println!("{}", checksum.value());
        assert!(checksum.value() == 0x3fad);

    }

    #[test]
    fn fletcher32_test() {
        let data = vec![0xF02A, 0xCB0D, 0x5639, 0x6501, 0x2384, 0x75BB];
        let mut checksum = super::fletcher32::Fletcher32::new();

        checksum.update(&data);
        println!("{}", checksum.value());
        assert!(checksum.value() == 0xdcf30fb3);
    }
}
