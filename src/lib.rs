#![feature(concat_idents)]

pub mod read;

#[macro_export]
macro_rules! input {
    ($source:expr, $($var:ident: $ty:ident,)*) => {
        let mut s = $source;
        $(
            let $var = {
                use $crate::read::*;
                concat_idents!(read_, $ty)(&mut s)
            };
        )*
    };
    ($(rest:tt)*) => {
        use std::io::Read as _;
        let __stdin = std::io::stdin();
        let __stdin = __stdin.lock();
        input!(__stdin; $(rest)* );
    };
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    #[test]
    fn input_number() {
        let s = BufReader::new(&b"    32   54 -23\r\r\n\nfalse"[..]);
        input! {
            s,
            n: u32,
            m: u32,
            l: i32,
            b: bool,
        }

        assert_eq!(n, 32);
        assert_eq!(m, 54);
        assert_eq!(l, -23);
        assert_eq!(b, false);
    }
}
