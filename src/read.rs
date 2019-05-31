use crate::source::BufferedSource;
use std::io::BufRead;

macro_rules! define_read {
    ($(($ty:ty, $fnname:ident))*) => {
        $(
            #[inline]
            pub fn $fnname<R: BufRead>(buffered_source: &mut BufferedSource<R>) -> $ty {
                let number = buffered_source.next_token();
                let number: String = number.collect();
                number.parse().expect("failed to parse")
            }
        )*
    };
}

define_read! {
    (u8, read_u8)
    (u16, read_u16)
    (u32, read_u32)
    (u64, read_u64)
    (u128, read_u128)
    (usize, read_usize)

    (i8, read_i8)
    (i16, read_i16)
    (i32, read_i32)
    (i64, read_i64)
    (i128, read_i128)
    (isize, read_isize)

    (bool, read_bool)
}
