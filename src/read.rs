use crate::source::{ReadSource, Source};

impl ReadSource for String {
    type Output = String;
    fn read(source: &mut Source) -> String {
        source.next_token_unwrap().into()
    }
}

pub type Chars = Vec<char>;
impl ReadSource for Chars {
    type Output = Chars;
    fn read(source: &mut Source) -> Chars {
        source.next_token_unwrap().chars().collect()
    }
}

pub type Bytes = Vec<u8>;
impl ReadSource for Bytes {
    type Output = Bytes;
    fn read(source: &mut Source) -> Bytes {
        source.next_token_unwrap().bytes().collect()
    }
}

macro_rules! impl_read_source_for_primitives {
    ($($ty:ty)*) => {
        $(
            impl ReadSource for $ty  {
                type Output = $ty;
                fn read(source: &mut Source) -> $ty {
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
