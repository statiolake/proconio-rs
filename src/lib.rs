pub mod read;
pub mod source;
pub mod types;

use crate::source::Source;
use lazy_static::lazy_static;
use std::io;
use std::io::{BufReader, Stdin};
use std::sync::Mutex;

lazy_static! {
    pub static ref STDIN_SOURCE: Mutex<Source<BufReader<Stdin>>> =
        Mutex::new(Source::new(BufReader::new(io::stdin())));
}

/// read input from stdin.
#[macro_export]
macro_rules! input {
    (from $source:expr, $($var:ident: $kind:tt),* $(,)?) => {
        let mut s = $source;
        $(
            let $var = $crate::read_value!($kind; &mut s);
        )*
    };
    ($($rest:tt)*) => {
        let mut locked_stdin = $crate::STDIN_SOURCE.lock().expect("failed to lock the stdin");
        input! {
            from &mut *locked_stdin,
            $($rest)*
        };
        drop(locked_stdin); // release the lock
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
        let source = Source::from("    32   54 -23\r\r\n\nfalse");

        input! {
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
        let source = Source::from("  string   chars\nbytes");

        input! {
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
        let source = Source::from("5 4 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5");

        input! {
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
        let source = Source::from("4 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5");

        input! {
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
        let mut source = Source::from("4 1 2 3 4\n1 2\r\n\r\r\n3 4");

        input! {
            from &mut source,
            n: usize,
        }

        for i in 0..n {
            input! {
                from &mut source,
                j: i32, k: i32,
            }

            assert_eq!(j, if i % 2 == 0 { 1 } else { 3 });
            assert_eq!(k, if i % 2 == 0 { 2 } else { 4 });
        }
    }
}
