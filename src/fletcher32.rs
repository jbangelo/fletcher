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
            let mut new_a = self.a + *byte as u32;
            if new_a >= 65535 {
                new_a -= 65535;
            }

            let mut new_b = self.b + new_a;
            if new_b >= 65535 {
                new_b -= 65535;
            }

            self.a = new_a;
            self.b = new_b;
        }
    }

    pub fn value(&self) -> u32 {
        self.a | (self.b << 16)
    }
}
