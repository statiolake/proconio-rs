pub mod read;
pub mod source;

#[macro_export]
macro_rules! input {
    ($($rest:tt)*) => {
        let stdin = std::io::stdin();
        let stdin = stdin.lock();
        let stdin = $crate::source::BufferedSource::new(stdin);
        $crate::input_from_source!(from stdin, $($rest)* );
    };
}

#[macro_export]
macro_rules! input_from_source {
    (from $source:expr, $($var:ident: $kind:tt,)*) => {
        let mut s = $source;
        $(
            let $var = $crate::read_value!($kind, &mut s);
        )*
    };
}

#[macro_export]
macro_rules! read_value {
    ([$kind:tt; $len:expr], $source:expr) => {{
        let mut res = Vec::new();
        res.reserve($len);
        for _ in 0..$len {
            res.push($crate::read_value!($kind, $source));
        }
        res
    }};
    (chars, $source:expr) => {
        $crate::read_value!(@ty Vec<char>, $source);
    };
    (bytes, $source:expr) => {
        $crate::read_value!(@ty Vec<u8>, $source);
    };
    ($ty:tt, $source:expr) => {
        $crate::read_value!(@ty $ty, $source);
    };
    (@ty $ty:ty, $source:expr) => {
        <$crate::source::BufferedSource<_> as $crate::source::ReadValue<$ty>>::read_value($source)
    }
}

#[cfg(test)]
mod tests {
    use crate::source::BufferedSource;
    use std::io::BufReader;
    #[test]
    fn input_number() {
        let source = BufferedSource::new(BufReader::new(&b"    32   54 -23\r\r\n\nfalse"[..]));
        input_from_source! {
            from source,
            n: u8,
            m: u32,
            l: i32,
        }

        assert_eq!(n, 32);
        assert_eq!(m, 54);
        assert_eq!(l, -23);
    }

    #[test]
    fn input_str() {
        let source = BufferedSource::new(BufReader::new(&b"  string   chars\nbytes"[..]));
        input_from_source! {
            from source,
            string: String,
            chars: chars,
            bytes: bytes,
        }

        assert_eq!(string, "string");
        assert_eq!(chars, ['c', 'h', 'a', 'r', 's']);
        assert_eq!(bytes, b"bytes");
    }

    #[test]
    fn input_array() {
        let source = BufferedSource::new(BufReader::new(
            &b"5 4 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5"[..],
        ));

        input_from_source! {
            from source,
            n: usize,
            m: usize,
            a: [[i32; n]; m],
        }

        assert_eq!(
            a,
            [
                [1, 2, 3, 4, 5],
                [1, 2, 3, 4, 5],
                [1, 2, 3, 4, 5],
                [1, 2, 3, 4, 5]
            ]
        );
    }
}
