use crate::source::{BufferedSource, ReadValue};
use std::io::BufRead;

impl<R: BufRead> ReadValue<String> for BufferedSource<R> {
    fn read_value(&mut self) -> String {
        self.next_token().collect()
    }
}

impl<R: BufRead> ReadValue<Vec<char>> for BufferedSource<R> {
    fn read_value(&mut self) -> Vec<char> {
        self.next_token().collect()
    }
}

impl<R: BufRead> ReadValue<Vec<u8>> for BufferedSource<R> {
    fn read_value(&mut self) -> Vec<u8> {
        self.next_token().map(|x| x as _).collect()
    }
}

macro_rules! impl_read_value_for_primitives {
    ($($ty:ty)*) => {
        $(
        impl<R: BufRead> ReadValue<$ty> for BufferedSource<R> {
            fn read_value(&mut self) -> $ty {
                let s: String = self.read_value();
                s.parse().expect("failed to parse")
            }
        }
        )*
    }
}

impl_read_value_for_primitives! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    char bool f32 f64
}
