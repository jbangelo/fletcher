#![no_std]

pub mod fletcher16;
pub mod fletcher32;
pub mod fletcher64;
pub mod generic_fletcher;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests {
    #[test]
    fn fletcher16_test() {
        let data = vec![0xC1, 0x77, 0xE9, 0xC0, 0xAB, 0x1E];
        let mut checksum = super::fletcher16::Fletcher16::new();

        checksum.update(&data);
        assert!(checksum.value() == 0x3fad);
    }

    #[test]
    fn fletcher16_underflow() {
        let zeros = vec![0; 200000usize];
        let mut checksum = super::fletcher16::Fletcher16::new();
        checksum.update(&zeros);
        assert!(checksum.value() == 0xffff);
    }

    #[test]
    fn fletcher16_overflow() {
        let zeros = vec![0xff; 200000usize];
        let mut checksum = super::fletcher16::Fletcher16::new();
        checksum.update(&zeros);
        assert!(checksum.value() == 0xffff);
    }

    #[test]
    fn fletcher32_test() {
        let data = vec![0xF02A, 0xCB0D, 0x5639, 0x6501, 0x2384, 0x75BB];
        let mut checksum = super::fletcher32::Fletcher32::new();

        checksum.update(&data);
        println!("{}", checksum.value());
        assert!(checksum.value() == 0xdcf30fb3);
    }

    #[test]
    fn fletcher32_underflow() {
        let zeros = vec![0; 200000usize];
        let mut checksum = super::fletcher32::Fletcher32::new();
        checksum.update(&zeros);
        assert!(checksum.value() == 0xffffffff);
    }

    #[test]
    fn fletcher32_overflow() {
        let zeros = vec![0xffff; 200000usize];
        let mut checksum = super::fletcher32::Fletcher32::new();
        checksum.update(&zeros);
        assert!(checksum.value() == 0xffffffff);
    }

    #[test]
    fn fletcher64_underflow() {
        let zeros = vec![0; 200000usize];
        let mut checksum = super::fletcher64::Fletcher64::new();
        checksum.update(&zeros);
        assert!(checksum.value() == 0xffffffffffffffff);
    }

    #[test]
    fn fletcher64_overflow() {
        let zeros = vec![0xffffffff; 200000usize];
        let mut checksum = super::fletcher64::Fletcher64::new();
        checksum.update(&zeros);
        assert!(checksum.value() == 0xffffffffffffffff);
    }
}
