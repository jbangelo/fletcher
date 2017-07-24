pub struct Fletcher32 {
    a: u32,
    b: u32,
}

const MAX_CHUNK_SIZE: usize = 65536;
const REDUCE_BY_VALUE: u32 = 65535;

impl Fletcher32 {
    pub fn new() -> Fletcher32 {
        Fletcher32 {
            a: 0x0000ffff,
            b: 0x0000ffff,
        }
    }

    pub fn update(&mut self, data: &Vec<u16>) {
        for chunk in data.chunks(MAX_CHUNK_SIZE) {
            let mut intermediate_a = self.a;
            let mut intermediate_b = self.b;

            for byte in chunk {
                intermediate_a += *byte as u32;
                intermediate_b += intermediate_a;
            }

            self.a = Fletcher32::reduce(intermediate_a);
            self.b = Fletcher32::reduce(intermediate_b);
        }
    }

    pub fn value(&self) -> u32 {
        self.a | (self.b << 16)
    }

    fn reduce(mut value: u32) -> u32 {
        while value > REDUCE_BY_VALUE {
            value -= REDUCE_BY_VALUE;
        }
        value
    }
}
