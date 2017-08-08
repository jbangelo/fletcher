use std::ops::Add;
use std::marker::PhantomData;
use std::marker::Copy;

pub trait FletcherSum<T>
    : Default + Add<Self> + From<T> + From<<Self as Add>::Output> + Copy
where
    T: Copy,
{
    fn max_chunk_size() -> usize;
    fn combine(lower: &Self, upper: &Self) -> Self;
    fn reduce(self) -> Self;
}

pub struct Fletcher<T, U> {
    a: T,
    b: T,
    phantom: PhantomData<U>,
}

impl<T, U> Fletcher<T, U>
where
    T: FletcherSum<U>,
    U: Copy,
{
    pub fn new() -> Fletcher<T, U> {
        Fletcher {
            a: Default::default(),
            b: Default::default(),
            phantom: PhantomData,
        }
    }

    pub fn update(&mut self, data: &Vec<U>) {
        let max_chunk_size = T::max_chunk_size();

        for chunk in data.chunks(max_chunk_size) {
            let mut intermediate_a = self.a;
            let mut intermediate_b = self.b;

            for byte in chunk {
                intermediate_a = T::from(intermediate_a + T::from(*byte));
                intermediate_b = T::from(intermediate_b + intermediate_a);
            }

            self.a = intermediate_a.reduce();
            self.b = intermediate_b.reduce();
        }

        self.a = self.a.reduce();
        self.b = self.b.reduce();
    }

    pub fn value(&self) -> T {
        T::combine(&self.a, &self.b)
    }
}
