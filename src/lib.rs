pub mod read;
pub mod source;

/// read input from stdin.
#[macro_export]
macro_rules! input {
    ($($rest:tt)*) => {
        let stdin = std::io::stdin();
        let stdin = stdin.lock();
        let stdin = $crate::source::Source::new(stdin);
        $crate::input_from_source!(from stdin, $($rest)* );
    };
}

/// read input from specified source.
#[macro_export]
macro_rules! input_from_source {
    (from $source:expr, $($var:ident: $kind:tt,)*) => {
        let mut s = $source;
        $(
            let $var = $crate::read_value!($kind; &mut s);
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! read_value {
    // array
    ([$kind:tt; $len:expr]; $source:expr) => {{
        let mut res = Vec::new();
        res.reserve($len);
        for _ in 0..$len {
            res.push($crate::read_value!($kind; $source));
        }
        res
    }};

    // tuple
    (($($kind:tt),*); $source:expr) => {
        (
            $($crate::read_value!($kind; $source),)*
        )
    };

    // Chars: Vec<char>
    (Chars; $source:expr) => {
        $crate::read_value!(@ty $crate::read::Chars; $source);
    };

    // Bytes: Vec<u8>
    (Bytes; $source:expr) => {
        $crate::read_value!(@ty $crate::read::Bytes; $source);
    };

    // normal other
    ($ty:tt; $source:expr) => {
        $crate::read_value!(@ty $ty; $source);
    };

    // actual reading
    (@ty $ty:ty; $source:expr) => {
        <$ty as $crate::source::ReadSource>::read($source)
    }
}

#[cfg(test)]
mod tests {
    use crate::source::Source;
    use std::io::BufReader;

    #[test]
    fn input_number() {
        let source = Source::new(BufReader::new(&b"    32   54 -23\r\r\n\nfalse"[..]));

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
        let source = Source::new(BufReader::new(&b"  string   chars\nbytes"[..]));

        input_from_source! {
            from source,
            string: String,
            chars: Chars,
            bytes: Bytes,
        }

        assert_eq!(string, "string");
        assert_eq!(chars, ['c', 'h', 'a', 'r', 's']);
        assert_eq!(bytes, b"bytes");
    }

    #[test]
    fn input_array() {
        let source = Source::new(BufReader::new(
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

    #[test]
    fn input_tuple() {
        let source = Source::new(BufReader::new(
            &b"4 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5"[..],
        ));

        input_from_source! {
            from source,
            n: usize,
            t: [(i32, i32, i32, i32, i32); n],
        }

        assert_eq!(
            t,
            [
                (1, 2, 3, 4, 5),
                (1, 2, 3, 4, 5),
                (1, 2, 3, 4, 5),
                (1, 2, 3, 4, 5)
            ]
        );
    }
}
