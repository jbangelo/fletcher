pub mod fletcher16;
pub mod fletcher32;

#[cfg(test)]
mod tests {
    #[test]
    fn fletcher16_test() {
        let data1 = vec![b'a', b'b', b'c', b'd', b'e'];
        let mut checksum = ::fletcher16::Fletcher16::new();

        checksum.update(&data1);
        println!("{}", checksum.value());
        assert!(checksum.value() == 51440);
    }
}
