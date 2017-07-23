pub struct Fletcher16 {
    a: u16,
    b: u16,
}

impl Fletcher16 {
    pub fn new() -> Fletcher16 {
        Fletcher16 {
            a: 0x00ff,
            b: 0x00ff,
        }
    }

    pub fn update(&mut self, data: &Vec<u8>) {
        for byte in data {
            let mut new_a = self.a + *byte as u16;
            if new_a >= 255 {
                new_a -= 255;
            }

            let mut new_b = self.b + new_a;
            if new_b >= 255 {
                new_b -= 255;
            }

            self.a = new_a;
            self.b = new_b;
        }
    }

    pub fn value(&self) -> u16 {
        self.a | (self.b << 8)
    }
}
