use crate::source::{ReadSource, Source};
use crate::types::{Bytes, Chars, Isize1, Usize1};
use std::io::BufRead;

macro_rules! impl_read_source_for_primitives {
    ($($ty:ty)*) => {
        $(
            impl ReadSource for $ty  {
                type Output = $ty;
                fn read<R: BufRead, S: Source<R>>(source: &mut S) -> $ty {
                    source
                        .next_token_unwrap()
                        .parse()
                        .expect("failed to parse")
                }
            }
        )*
    }
}

impl_read_source_for_primitives! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    char bool f32 f64
}

impl ReadSource for String {
    type Output = String;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> String {
        source.next_token_unwrap().into()
    }
}

impl ReadSource for Chars {
    type Output = Chars;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Chars {
        source.next_token_unwrap().chars().collect()
    }
}

impl ReadSource for Bytes {
    type Output = Bytes;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Bytes {
        source.next_token_unwrap().bytes().collect()
    }
}

impl ReadSource for Usize1 {
    type Output = usize;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> usize {
        usize::read(source) - 1
    }
}

impl ReadSource for Isize1 {
    type Output = isize;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> isize {
        isize::read(source) - 1
    }
}
