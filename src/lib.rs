pub mod read;
pub mod source;
pub mod types;

/// read input from stdin.
#[macro_export]
macro_rules! input {
    ($($rest:tt)*) => {
        use std::io::{BufReader, Read as _};
        let stdin = std::io::stdin();
        let stdin = stdin.lock();
        let source = $crate::source::Source::new(stdin);
        $crate::input_from_source!(from source, $($rest)* );
    };
}

/// read input from specified source.
#[macro_export]
macro_rules! input_from_source {
    (from $source:expr, $($var:ident: $kind:tt),* $(,)?) => {
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
        $crate::read_value!(@ty $crate::types::Chars; $source);
    };

    // Bytes: Vec<u8>
    (Bytes; $source:expr) => {
        $crate::read_value!(@ty $crate::types::Bytes; $source);
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

    #[test]
    fn input_number() {
        let source = Source::from_str("    32   54 -23\r\r\n\nfalse");

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
        let source = Source::from_str("  string   chars\nbytes");

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
        let source = Source::from_str("5 4 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5");

        input_from_source! {
            from source,
            n: usize,
            m: usize,
            a: [[i32; n]; m] // no trailing comma is OK
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
        let source = Source::from_str("4 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5");

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

    #[test]
    fn input_multiple_times() {
        let mut source = Source::from_str("4 1 2 3 4 1 2 3 4");

        input_from_source! {
            from &mut source,
            n: usize,
        }

        for i in 0..n {
            input_from_source! {
                from &mut source,
                j: i32, k: i32,
            }

            assert_eq!(j, if i % 2 == 0 { 1 } else { 3 });
            assert_eq!(k, if i % 2 == 0 { 2 } else { 4 });
        }
    }
}
