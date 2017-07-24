pub struct Fletcher16 {
    a: u16,
    b: u16,
}

const MAX_CHUNK_SIZE: usize = 256;
const REDUCE_BY_VALUE: u16 = 255;

impl Fletcher16 {
    pub fn new() -> Fletcher16 {
        Fletcher16 {
            a: 0x00ff,
            b: 0x00ff,
        }
    }

    pub fn update(&mut self, data: &Vec<u8>) {
        for chunk in data.chunks(MAX_CHUNK_SIZE) {
            let mut intermediate_a = self.a;
            let mut intermediate_b = self.b;

            for byte in chunk {
                intermediate_a += *byte as u16;
                intermediate_b += intermediate_a;
            }

            self.a = Fletcher16::reduce(intermediate_a);
            self.b = Fletcher16::reduce(intermediate_b);
        }
    }

    pub fn value(&self) -> u16 {
        self.a | (self.b << 8)
    }

    fn reduce(mut value: u16) -> u16 {
        while value > REDUCE_BY_VALUE {
            value -= REDUCE_BY_VALUE;
        }
        value
    }
}
