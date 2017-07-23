pub struct Fletcher32 {
    a: u32,
    b: u32,
}

impl Fletcher32 {
    pub fn new() -> Fletcher32 {
        Fletcher32 {
            a: 0x0000ffff,
            b: 0x0000ffff,
        }
    }

    pub fn update(&mut self, data: &Vec<u16>) {
        for byte in data {
            let new_a = (self.a + *byte as u32) % 65536;
            let new_b = (self.b + new_a) % 65536;

            self.a = new_a;
            self.b = new_b;
        }
    }

    pub fn value(&self) -> u32 {
        self.a | (self.b << 16)
    }
}
