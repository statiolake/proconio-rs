#![recursion_limit = "256"]
pub mod read;
pub mod source;
pub mod types;

use crate::source::auto::AutoSource;
use lazy_static::lazy_static;
use std::io;
use std::io::{BufReader, Stdin};
use std::sync::Mutex;

lazy_static! {
    pub static ref STDIN_SOURCE: Mutex<AutoSource<BufReader<Stdin>>> =
        Mutex::new(AutoSource::new(BufReader::new(io::stdin())));
}

/// read input from stdin.
#[macro_export]
macro_rules! input {
    (from $source:expr $(,)?) => {};
    (from $source:expr, mut $var:ident: $kind:tt $($rest:tt)*) => {
        let mut s = $source;
        let mut $var = $crate::read_value!($kind; &mut s);
        input!(from &mut s $($rest)*);
    };
    (from $source:expr, $var:ident: $kind:tt $($rest:tt)*) => {
        let mut s = $source;
        let $var = $crate::read_value!($kind; &mut s);
        input!(from &mut s $($rest)*);
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

    // normal other
    ($ty:tt; $source:expr) => {
        $crate::read_value!(@ty $ty; $source);
    };

    // actual reading
    (@ty $ty:ty; $source:expr) => {
        <$ty as $crate::source::Readable>::read($source)
    }
}

#[macro_export]
macro_rules! derive_readable {
    ($(#[$($attr:meta)*])* struct $name:ident $($tt:tt)*) => {
        derive_readable! {
            @general
            @attr ($(#[$($attr)*])*)
            @vis ()
            @name $name
            @rest $($tt)*
        }
    };

    ($(#[$($attr:meta)*])* $svis:vis struct $name:ident $($tt:tt)*) => {
        derive_readable! {
            @general
            @attr ($(#[$($attr)*])*)
            @vis ($svis)
            @name $name
            @rest $($tt)*
        }
    };

    (@general @attr ($($attr:tt)*) @vis ($($svis:tt)*) @name $name:ident @rest { $($tt:tt)* }) => {
        derive_readable! {
            @normal
            @attr ($($attr)*)
            @vis ($($svis)*)
            @name $name
            @fields {}
            @rest $($tt)*,
        }
    };

    (@general @attr ($($attr:tt)*) @vis ($($svis:tt)*) @name $name:ident @rest ( $($tt:tt)* );) => {
        derive_readable! {
            @tuple
            @attr ($($attr)*)
            @vis ($($svis)*)
            @name $name
            @fields ()
            @rest $($tt)*,
        }
    };

    (@general @attr ($($attr:tt)*) @vis ($($svis:tt)*) @name $name:ident @rest ;) => {
        derive_readable! {
            @unit
            @attr ($($attr)*)
            @vis ($($svis)*)
            @name $name
        }
    };

    (@normal @attr ($($attr:tt)*) @vis ($($svis:tt)*) @name $name:ident @fields {$(($($fvis:tt)*) $field:ident: $ty:ty,)*} @rest $(,)? ) => {
        $($attr)*
        $($svis)* struct $name {
            $(
                $($fvis)* $field: <$ty as $crate::source::Readable>::Output,
            )*
        }

        impl $crate::source::Readable for $name {
            type Output = $name;
            fn read<R: std::io::BufRead, S: $crate::source::Source<R>>(source: &mut S) -> $name {
                $name {
                    $(
                        $field: <$ty as $crate::source::Readable>::read(source),
                    )*
                }
            }
        }
    };

    (@normal @attr ($($attr:tt)*) @vis ($($svis:tt)*) @name $name:ident @fields {$(($($fvis:tt)*) $field:ident: $ty:ty,)*} @rest $cfvis:vis $cfield:ident: $cty:ty, $($tt:tt)*) => {
        derive_readable! {
            @normal
            @attr ($($attr)*)
            @vis ($($svis)*)
            @name $name
            @fields {
                $(
                    ($($fvis)*) $field: $ty,
                )*
                ($cfvis) $cfield: $cty,
            }
            @rest $($tt)*
        }
    };

    (@normal @attr ($($attr:tt)*) @vis ($($svis:tt)*) @name $name:ident @fields {$(($($fvis:tt)*) $field:ident: $ty:ty,)*} @rest $cfield:ident: $cty:ty, $($tt:tt)*) => {
        derive_readable! {
            @normal
            @attr ($($attr)*)
            @vis ($($svis)*)
            @name $name
            @fields {
                $(
                    ($($fvis)*) $field: $ty,
                )*
                () $cfield: $cty,
            }
            @rest $($tt)*
        }
    };

    (@tuple @attr ($($attr:tt)*) @vis ($($svis:tt)*) @name $name:ident @fields ($(($($fvis:tt)*) $ty:ty,)*) @rest $(,)? ) => {
        $($attr)*
        $($svis)* struct $name (
            $(
                $($fvis)* <$ty as $crate::source::Readable>::Output,
            )*
        );

        impl $crate::source::Readable for $name {
            type Output = $name;
            fn read<R: std::io::BufRead, S: $crate::source::Source<R>>(source: &mut S) -> $name {
                $name (
                    $(
                        <$ty as $crate::source::Readable>::read(source),
                    )*
                )
            }
        }
    };

    (@tuple @attr ($($attr:tt)*) @vis ($($svis:tt)*) @name $name:ident @fields ($(($($fvis:tt)*) $ty:ty,)*) @rest $cfvis:vis $cty:ty, $($tt:tt)*) => {
        derive_readable! {
            @tuple
            @attr ($($attr)*)
            @vis ($($svis)*)
            @name $name
            @fields (
                $(
                    ($($fvis)*) $ty,
                )*
                ($cfvis) $cty,
            )
            @rest $($tt)*
        }
    };

    (@tuple @attr ($($attr:tt)*) @vis ($($svis:tt)*) @name $name:ident @fields ($(($($fvis:tt)*) $ty:ty,)*) @rest $cty:ty, $($tt:tt)*) => {
        derive_readable! {
            @tuple
            @attr ($($attr)*)
            @vis ($($svis)*)
            @name $name
            @fields (
                $(
                    ($($fvis)*) $ty,
                )*
                () $cty,
            )
            @rest $($tt)*
        }
    };

    (@unit @attr ($($attr:tt)*) @vis ($($svis:tt)*) @name $name:ident) => {
        $($attr)*
        $($svis)* struct $name;

        impl $crate::source::Readable for $name {
            type Output = $name;
            fn read<R: std::io::BufRead, S: $crate::source::Source<R>>(source: &mut S) -> $name {
                $name
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::source::auto::AutoSource;

    #[test]
    fn input_number() {
        let source = AutoSource::from("    32   54 -23\r\r\n\nfalse");

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
        use crate::types::{Bytes, Chars};
        let source = AutoSource::from("  string   chars\nbytes");

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
        let source = AutoSource::from("5 4 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5");

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
        let source = AutoSource::from("4 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5");

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
        let mut source = AutoSource::from("4 1 2 3 4\n1 2\r\n\r\r\n3 4");

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

    #[test]
    fn input_iusize1() {
        use crate::types::Usize1;

        let mut source = AutoSource::from("4 1 2 3 4 5 6 7 8");

        input! {
            from &mut source,
            n: usize,
        }

        for i in 0..n {
            input! {
                from &mut source,
                from: Usize1, to: Usize1
            }

            assert_eq!(from, i * 2);
            assert_eq!(to, i * 2 + 1);
        }
    }

    #[test]
    fn input_mut() {
        let mut source = AutoSource::from("8 1 2 3 4 5 6 7 8");
        input! {
            from &mut source,
            mut n: usize,
        }

        let mut sum = 0;
        while n > 0 {
            input!(from &mut source, x: u32);
            sum += x;
            n -= 1;
        }
        assert_eq!(sum, 36);
    }

    #[test]
    fn input_many() {
        let mut source = AutoSource::from("1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1");
        input! {
            from &mut source,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            _x: i32,
            x: i32,
        }
        assert_eq!(x, 1);
    }
}
