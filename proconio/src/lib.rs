// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

#![allow(clippy::needless_doctest_main, clippy::print_literal)]

//! Easy IO library for competitive programming.
//!
//! `proconio` provides an easy way to read values from stdin (or other source).  The main is
//! `input!` macro.
//!
//! # Examples
//!
//! The macro's user interface is basically the same with [tanakh's input
//! macro](https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8).
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let source = AutoSource::from("32 54 -23");
//!
//! input! {
//! #   from source,
//!     n: u8,
//!     m: u32,
//!     l: i32,
//! }
//!
//! // now you can use n, m and l as variable.
//! println!("{} {} {}", n, m, l);
//! # assert_eq!(n, 32);
//! # assert_eq!(m, 54);
//! # assert_eq!(l, -23);
//! ```
//!
//! In above code, variables n, m and l are declared and stored values are read from stdin.
//!
//! You can declare mutable variables like below:
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let source = AutoSource::from("32 54");
//!
//! input! {
//! #   from source,
//!     n: u32,
//!     mut m: u32,
//! }
//!
//! m += n; // OK: m is mutable
//! # assert_eq!(n, 32);
//! # assert_eq!(m, 86);
//! ```
//!
//! You can read an array or a matrix like this:
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let source = AutoSource::from("5 4 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5");
//!
//! input! {
//! #    from source,
//!     n: usize,
//!     m: usize,
//!     a: [[i32; n]; m] // `a` is Vec<Vec<i32>>, (m, n)-matrix.
//! }
//! # assert_eq!(
//! #     a,
//! #     [
//! #         [1, 2, 3, 4, 5],
//! #         [1, 2, 3, 4, 5],
//! #         [1, 2, 3, 4, 5],
//! #         [1, 2, 3, 4, 5]
//! #     ]
//! # );
//! ```
//!
//! If the first input is the length of the array, you can omit the length.  This is the only way
//! to read jagged array (an array of arrays of which the member arrays can be of different sizes)
//! at once.  (Of course you can use `input!` multiple times in for-loop to read such an array
//! since `input!` can be used multiple times.)
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let source = AutoSource::from("3 3 1 2 3 0 2 1 2");
//!
//! input! {
//! #   from source,
//!     n: usize,
//!     a: [[i32]; n],
//! }
//!
//! // if you enter "3  3 1 2 3  0  2 1 2" to the stdin, the result is as follows.
//! assert_eq!(
//!     a,
//!     vec![
//!         vec![1, 2, 3],
//!         vec![],
//!         vec![1, 2],
//!     ]
//! );
//! ```
//!
//! Strings can be read as `Vec<u8>` or `Vec<char>`.  Use `Bytes` and `Chars` to do so:
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! use proconio::marker::{Bytes, Chars};
//! # let source = AutoSource::from("  string   chars\nbytes");
//!
//! input! {
//! #     from source,
//!     string: String, // read as String
//!     chars: Chars,   // read as Vec<char>
//!     bytes: Bytes,   // read as Vec<u8>
//! }
//!
//! // if you enter "string chars bytes" to the stdin, they are like this.
//! assert_eq!(string, "string");
//! assert_eq!(chars, ['c', 'h', 'a', 'r', 's']);
//! assert_eq!(bytes, b"bytes");
//! ```
//!
//! You can read tuples:
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let source = AutoSource::from("1 2 3 4 5");
//!
//! input! {
//! #   from source,
//!     t: (i32, i32, i32, i32, i32),
//! }
//!
//! // if you enter "1 2 3 4 5" to the stdin, `t` is like this.
//! assert_eq!(t, (1, 2, 3, 4, 5));
//! ```
//!
//! And you can freely combine these types.
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let source = AutoSource::from("4 3 1 1 1 1 2 2 2 2 3 3 3 3 4 4 4 4 5 5 5 5");
//!
//! input! {
//! #   from source,
//!     n: usize,
//!     m: usize,
//!     t: [([u32; m], i32); n],
//! }
//! # assert_eq!(
//! #     t,
//! #     [
//! #         (vec![1,1,1],1),
//! #         (vec![2,2,2],2),
//! #         (vec![3,3,3],3),
//! #         (vec![4,4,4],4),
//! #     ]
//! # );
//! ```
//!
//! You can use `input!` macro multiple times.  For the second time, `input!` macro reads rest of
//! input.  It works even if the first input stops at the middle of a line.  The subsequent reads
//! will be started at the rest of the line.  This may be helpful for problems where multiple
//! datasets are given once.
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let mut source = AutoSource::from("4 2 1 2 2 3 4 2 1 2 2 3 4");
//!
//! input! {
//! #   from &mut source,
//!     n: usize,
//! }
//!
//! for i in 0..n {
//!     input! {
//! #       from &mut source,
//!         m: usize,
//!         a: [i32; m],
//!     }
//! #   assert_eq!(a[0], if i % 2 == 0 { 1 } else { 3 });
//! #   assert_eq!(a[1], if i % 2 == 0 { 2 } else { 4 });
//! }
//! ```
//!
//! In addition to `Chars` and `Bytes`, `Usize1` and `Isize1` are also special types.  They are
//! read as `usize` and `isize` respectively, but the read value is decremented.  It enables us to
//! automatically convert 1-indexed vertices numbers to 0-indexed array indices.
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! use proconio::marker::Usize1;
//! # let mut source = AutoSource::from("4   1 3   3 4   6 1   5 3");
//!
//! input! {
//! #   from &mut source,
//!     n: usize,
//!     edges: [(Usize1, Usize1); n],
//! }
//!
//! // if you enter "4   1 3   3 4   6 1   5 3", the decremented value is stored.
//! assert_eq!(edges[0], (0, 2));
//! assert_eq!(edges[1], (2, 3));
//! assert_eq!(edges[2], (5, 0));
//! assert_eq!(edges[3], (4, 2));
//! ```
//!
//! `Usize1` and `Isize1` doesn't hold actual value, so you cannot have value of the type.  Thus,
//! they are only useful inside `input!` or `#[derive_readable]`.  You can think the reason these
//! types exist is to tell "how to read the value".  This how-to can be defined through `Readable`
//! trait.  This trait doesn't require the output type to be the same with the implementor.
//! `Usize1` is implementing `Readable` trait, and there the type of read value is defined as
//! `usize`.  You can implement `Readable` for your own type to read values in customized way.
//!
//! Finally, you can make your own types `Readable` using `#[derive_readable]` attribute.  Types
//! used in the struct are automatically translated to their output types, so a member declared as
//! `Usize1` has type `usize` as real struct.
//!
//! **Note:** Using `#[derive_readable]` requires `derive` feature enabled.  To do so, open your
//! Cargo.toml and modify the line of proconio from:
//!
//! ```toml
//! proconio = "=(version)"
//! ```
//!
//! to:
//!
//! ```toml
//! proconio = { version = "=(version)", features = ["derive"] }
//! ```
//!
//! Example of `#[derive_readable]`:
//!
//! ```
//! # #[cfg(feature = "derive")]
//! # {
//! # extern crate proconio;
//! use proconio::input;
//! # use proconio::source::auto::AutoSource;
//! use proconio::derive_readable;
//!
//! // Unit struct can derive readable.  This generates a no-op for the reading.  Not ignoring
//! // the read value, but simply skip reading process.  You cannot use it to discard the input.
//! #[derive_readable]
//! #[derive(PartialEq, Debug)]
//! struct Weight;
//!
//! #[derive_readable]
//! #[derive(PartialEq, Debug)]
//! struct Cost(i32);
//!
//! #[derive_readable]
//! #[derive(Debug)]
//! struct Edge {
//!     from: usize,
//!     to: proconio::marker::Usize1, // The real Edge::to has type usize.
//!     weight: Weight,
//!     cost: Cost,
//! }
//!
//! fn main() {
//! #   let source = AutoSource::from("12 32 35");
//!     input! {
//! #       from source,
//!         edge: Edge,
//!     }
//!
//!     // if you enter "12 32 35" to the stdin, the values are as follows.
//!     assert_eq!(edge.from, 12);
//!     assert_eq!(edge.to, 31);
//!     assert_eq!(edge.weight, Weight);
//!     assert_eq!(edge.cost, Cost(35));
//! }
//! # }
//! ```
//!
//! # `#[fastout]`
//!
//! If you import `proconio::fastout`, you can use `#[fastout]` attribute.  Adding this attribute
//! to your `main()`, your `print!` and `println!` become faster.
//!
//! **Note:** Using `#[proconio::fastout]` requires `derive` feature enabled.  To do so, open your
//! Cargo.toml and modify the line of proconio from:
//!
//! ```toml
//! proconio = "=(version)"
//! ```
//!
//! to:
//!
//! ```toml
//! proconio = { version = "=(version)", features = ["derive"] }
//! ```
//!
//! Example of `#[fastout]`:
//!
//! ```
//! # #[cfg(feature = "derive")]
//! # {
//! # extern crate proconio;
//! use proconio::fastout;
//!
//! #[fastout]
//! fn main() {
//!     print!("{}{}, ", 'h', "ello"); // "hello"       (no newline)
//!     println!("{}!", "world");      // "world!\n"
//!     println!("{}", 123456789);     // "123456789\n"
//! }
//! # }
//! ```
//!
//! ## Closures having `print!` or `println!` in `#[fastout]` function
//!
//! You cannot create a closure containing `print!` or `println!` in `#[fastout]` function.  This
//! is because the closure becomes thread-unsafe since the closure refers the unlocked stdout
//! introduced by `#[fastout]` attribute.  If this were not prohibited, an invalid usage of such a
//! closure would produce a very complex error messages.  For example, `std::thread::spawn()`,
//! which requires its argument closure to be thread-safe, causes a confusing error.
//!
//! Yes, it is too conservative to make all of such closures compilation error because it is
//! actually no problem to use such a closure only inside a single thread.  This is related to a
//! limitation in `#[fastout]` implementation.
//!
//! For more technical details, see documentation for `#[fastout]` in `proconio-derive`.
//!
//! ### How to resolve this error
//!
//! Consider you want to run this code:
//!
//! ```compile_fail
//! use proconio::fastout;
//!
//! #[fastout]
//! fn main() {
//!     let thread = std::thread::spawn(|| {
//!         let x = 3;
//!         let y = x * x;
//!         println!("{}", y);
//!     });
//!
//!     thread.join().unwrap();
//! }
//! ```
//!
//! You will get an error like below.
//!
//! ```text
//! error: Closures in a #[fastout] function cannot contain `print!` or `println!` macro
//!
//! note: If you want to run your entire logic in a thread having extended size of stack, you can
//! define a new function instead.  See documentation (https://.....) for more details.
//!
//! note: This is because if you use this closure with `std::thread::spawn()` or any other
//! functions requiring `Send` for an argument closure, the compiler emits an error about thread
//! unsafety for our internal implementations.  If you are using the closure just in a single
//! thread, it's actually no problem, but we cannot check the trait bounds at the macro-expansion
//! time.  So for now, all closures having `print!` or `println!` is prohibited regardless of the
//! `Send` requirements.
//!  --> src/test.rs:10:9
//!    |
//! 10 |         println!("{}", y);
//!    |         ^^^^^^^
//! ```
//!
//! If your `print!` is relying on the calculation in the thread, you can instead return the result
//! from the thread.
//!
//! ```
//! # #[cfg(feature = "derive")]
//! # {
//! use proconio::fastout;
//!
//! #[fastout]
//! fn main() {
//!     let thread = std::thread::spawn(|| {
//!         let x = 3;
//!         x * x
//!     });
//!
//!     let y = thread.join().unwrap();
//! #   assert_eq!(y, 9);
//!     println!("{}", y);
//! }
//! # }
//! ```
//!
//! If you are doing so complex job that it's too difficult to returning the results from your
//! closure...
//!
//! ```compile_fail
//! use proconio::fastout;
//!
//! # fn some_function(_: String) -> impl Iterator<Item = String> { vec!["hello".to_string(), "world".to_string()].into_iter() }
//! # fn some_proc(x: &str) -> &str { x }
//!
//! #[fastout]
//! fn main() {
//!     let context = "some context".to_string();
//!     let thread = std::thread::spawn(move || {
//!         // Use many println! and the order is very important
//!         // It's possible to aggregate the result and print it later, but it's not easy to read
//!         // and seems ugly.
//!         println!("this is header.");
//!         for (i, item) in some_function(context).enumerate() {
//!             print!("Item #{}: ", i);
//!             print!("{}", some_proc(&item));
//!             println!("({})", item);
//!         }
//!     });
//!
//!     thread.join().unwrap();
//! }
//! ```
//!
//! ...you can use a function instead.
//!
//! ```
//! # #[cfg(feature = "derive")]
//! # {
//! use proconio::fastout;
//!
//! # fn some_function(_: String) -> impl Iterator<Item = String> { vec!["hello".to_string(), "world".to_string()].into_iter() }
//! # fn some_proc(x: &str) -> &str { x }
//!
//! // You can add #[fastout] here
//! #[fastout]
//! fn process(context: String) {
//!     // It's completely OK since this #[fastout] is a thing inside `process()`
//!     println!("this is header.");
//!     for (i, item) in some_function(context).enumerate() {
//!         print!("Item #{}: ", i);
//!         print!("{}", some_proc(&item));
//!         println!("({})", item);
//!     }
//! }
//!
//! // You must not add #[fastout] here!  It causes deadlock.
//! // #[fastout]
//! fn main() {
//!     let context = "some context".to_string();
//!     let thread = std::thread::spawn(move || process(context));
//!     thread.join().unwrap();
//! }
//! # }
//! ```
//!
//! **Important Note:** If you *spawn a new thread* which runs another function annotated with
//! `#[fastout]`, you must not add `#[fastout]` to the caller.  If you add `#[fastout]` in caller
//! too, then the caller has the lock for the stdout, and so callee cannot acquire the lock forever
//! --- deadlock.  This is not the case when the caller and callee is executed in the same thread,
//! since the lock of stdout is reentrant.  We cannot warn about this kind of deadlock since we
//! don't know annotations attached to the function to be called.  (In the above example, we can't
//! know whether the function `process()` has `#[fastout]` attribute or not.)
//!
//! If your code is so complex that you cannot avoid deadlock, you should give up using
//! `#[fastout]` and simply use `println!` or manually handle your stdout in usual Rust way.
//!
//! ## Issues of printing order
//!
//! `#[fastout]` enables buffering to stdout, so if you print something in other functions between
//! two prints in main, the order of printing may differ.  In other words, the below example
//!
//! ```
//! # #[cfg(feature = "derive")]
//! # {
//! # use proconio::fastout;
//! fn foo() { println!("between"); }
//! #[fastout]
//! fn main() {
//!     println!("hello");
//!     foo();
//!     println!("world");
//! }
//! # }
//! ```
//!
//! *likely* prints like
//!
//! ```text
//! between
//! hello
//! world
//! ```
//!
//! If you don't like this behavior, you can remove #[fastout] from your `main()`.
//!

#[cfg(feature = "derive")]
pub use proconio_derive::*;

pub mod marker;
pub mod read;
pub mod source;

use crate::source::auto::AutoSource;
use lazy_static::lazy_static;
use std::io;
use std::io::{BufReader, Stdin};
use std::sync::Mutex;

lazy_static! {
    #[doc(hidden)]
    pub static ref STDIN_SOURCE: Mutex<AutoSource<BufReader<Stdin>>> =
        Mutex::new(AutoSource::new(BufReader::new(io::stdin())));
}

/// read input from stdin.
///
/// basic syntax is:
/// ```text
/// input! {
///     from source,          // optional: if you omitted, stdin is used by default.
///     (mut) variable: type, // mut is optional: mut makes the variable mutable.
///     ...
/// }
/// ```
/// the trailing comma is optional.  `source` can be anything implementing `Source`.  This macro
/// moves out the specified source.  If you want to prevent moving, you can use `&mut source` since
/// `&mut S` where `S: Source` also implements `Source`.
#[macro_export]
macro_rules! input {
    // terminator
    (@from [$source:expr] @rest) => {};

    // parse mutability
    (@from [$source:expr] @rest mut $($rest:tt)*) => {
        $crate::input! {
            @from [$source]
            @mut [mut]
            @rest $($rest)*
        }
    };
    (@from [$source:expr] @rest $($rest:tt)*) => {
        $crate::input! {
            @from [$source]
            @mut []
            @rest $($rest)*
        }
    };

    // parse variable pattern
    (@from [$source:expr] @mut [$($mut:tt)?] @rest $var:tt: $($rest:tt)*) => {
        $crate::input! {
            @from [$source]
            @mut [$($mut)*]
            @var $var
            @kind []
            @rest $($rest)*
        }
    };

    // parse kind (type)
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest) => {
        let $($mut)* $var = $crate::read_value!(@source [$source] @kind [$($kind)*]);
    };
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest, $($rest:tt)*) => {
        $crate::input!(@from [$source] @mut [$($mut)*] @var $var @kind [$($kind)*] @rest);
        $crate::input!(@from [$source] @rest $($rest)*);
    };
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::input!(@from [$source] @mut [$($mut)*] @var $var @kind [$($kind)* $tt] @rest $($rest)*);
    };

    (from $source:expr, $($rest:tt)*) => {
        #[allow(unused_variables, unused_mut)]
        let mut s = $source;
        $crate::input! {
            @from [&mut s]
            @rest $($rest)*
        }
    };
    ($($rest:tt)*) => {
        let mut locked_stdin = $crate::STDIN_SOURCE.lock().expect("failed to lock the stdin");
        $crate::input! {
            @from [&mut *locked_stdin]
            @rest $($rest)*
        }
        drop(locked_stdin); // release the lock
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! read_value {
    // array and variable length array
    (@source [$source:expr] @kind [[$($kind:tt)*]]) => {
        $crate::read_value!(@array @source [$source] @kind [] @rest $($kind)*)
    };
    (@array @source [$source:expr] @kind [$($kind:tt)*] @rest) => {{
        let len = <usize as $crate::source::Readable>::read($source);
        $crate::read_value!(@source [$source] @kind [[$($kind)*; len]])
    }};
    (@array @source [$source:expr] @kind [$($kind:tt)*] @rest ; $($rest:tt)*) => {
        $crate::read_value!(@array @source [$source] @kind [$($kind)*] @len [$($rest)*])
    };
    (@array @source [$source:expr] @kind [$($kind:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::read_value!(@array @source [$source] @kind [$($kind)* $tt] @rest $($rest)*)
    };
    (@array @source [$source:expr] @kind [$($kind:tt)*] @len [$($len:tt)*]) => {{
        let len = $($len)*;
        (0..len)
            .map(|_| $crate::read_value!(@source [$source] @kind [$($kind)*]))
            .collect::<Vec<_>>()
    }};

    // tuple
    (@source [$source:expr] @kind [($($kinds:tt)*)]) => {
        $crate::read_value!(@tuple @source [$source] @kinds [] @current [] @rest $($kinds)*)
    };
    (@tuple @source [$source:expr] @kinds [$([$($kind:tt)*])*] @current [] @rest) => {
        (
            $($crate::read_value!(@source [$source] @kind [$($kind)*]),)*
        )
    };
    (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest) => {
        $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)* [$($curr)*]] @current [] @rest)
    };
    (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest, $($rest:tt)*) => {
        $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)* [$($curr)*]] @current [] @rest $($rest)*)
    };
    (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)*] @current [$($curr)* $tt] @rest $($rest)*)
    };

    // unreachable
    (@source [$source:expr] @kind []) => {
        compile_error!("Reached unreachable statement while parsing macro input.  This is a bug.");
    };

    // normal other
    (@source [$source:expr] @kind [$kind:ty]) => {
        <$kind as $crate::source::Readable>::read($source)
    }
}

/// Checks if some of tokens are left on stdin.
///
/// This is useful when the number of test cases is not specified like ICPC problems.
///
/// ```text
/// loop {
///     if is_stdin_empty() {
///         break;
///     }
///
///     // do the normal logic
///     input! { ... }
/// }
/// ```
pub fn is_stdin_empty() -> bool {
    use crate::source::Source;
    let mut lock = STDIN_SOURCE.lock().expect("failed to lock stdin");
    lock.is_empty()
}

#[cfg(test)]
mod tests {
    use crate::source::auto::AutoSource;

    #[test]
    fn input_empty() {
        let source = AutoSource::from("");
        input! {
            from source,
        }
    }

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
        use crate::marker::{Bytes, Chars};
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
        let source = AutoSource::from("5 3 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5");

        input! {
            from source,
            n: usize,
            m: usize,
            a: [i32; n],
            b: [[i32; n]; m] // no trailing comma is OK
        }

        assert_eq!(a, [1, 2, 3, 4, 5]);
        assert_eq!(b, [[1, 2, 3, 4, 5], [1, 2, 3, 4, 5], [1, 2, 3, 4, 5]]);
    }

    #[test]
    fn input_vla() {
        let source = AutoSource::from("5 3 1 2 3 2 1 2 4 1 2 3 4 0 6 1 2 3 4 5 6");

        input! {
            from source,
            n: usize,
            a: [[i32]; n],
        }

        assert_eq!(
            a,
            vec![
                vec![1, 2, 3],
                vec![1, 2],
                vec![1, 2, 3, 4],
                vec![],
                vec![1, 2, 3, 4, 5, 6],
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
        use crate::marker::Usize1;

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
    fn input_with_complex_type() {
        let mut source = AutoSource::from("Hello\n2\n3 1 2 3\n4 5 ab\n4");
        input! {
            from &mut source,
            hello: crate::marker::Bytes,
            from: crate::marker::Usize1,
            vla: [crate::marker::Isize1],
            tuple: (crate::marker::Usize1, crate::marker::Isize1, crate::marker::Chars),
            unit: (crate::marker::Usize1),
        }
        assert_eq!(hello, b"Hello");
        assert_eq!(from, 1);
        assert_eq!(vla, [0, 1, 2]);
        assert_eq!(tuple, (3, 4, vec!['a', 'b']));
        assert_eq!(unit, (3,));
    }

    #[test]
    fn input_single_tt_pattern() {
        let mut source = AutoSource::from("3 42 0\n1 2 3\n");
        input! {
            from &mut source,
            (n, mut k): (usize, usize),
            _: usize,
            xs: [u32; n],
        }
        k += 1;
        assert_eq!(n, 3);
        assert_eq!(k, 43);
        assert_eq!(xs, [1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn input_err_different_type() {
        let mut source = AutoSource::from("3.2\n");
        input! {
            from &mut source,
            _n: i32,
        }
    }

    #[test]
    #[should_panic]
    fn input_err_different_format() {
        let mut source = AutoSource::from("");
        input! {
            from &mut source,
            _n: i32,
        }
    }
}
