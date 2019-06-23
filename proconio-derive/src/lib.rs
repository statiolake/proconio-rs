#![recursion_limit = "128"]

//! Macros to easily derive `Readable` and make stdout faster.
//!
//! proconio_derive provides two procedural macros (attributes): `derive_readable` and `fastout`.
//!
//! # Examples for `#[derive_readable]`
//!
//! ```ignore
//! # extern crate proconio;
//! # extern crate proconio_derive;
//! use proconio::input;
//! # use proconio::source::auto::AutoSource;
//! use proconio_derive::derive_readable;
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
//!     to: proconio::types::Usize1, // The real Edge::to has type usize.
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
//! ```
//!
//! # Examples for `#[fastout]`
//!
//! ```ignore
//! use proconio_derive::fastout;
//!
//! #[fastout]
//! fn main() {
//!     print!("{}{}, ", 'h', "ello"); // "hello"       (no newline)
//!     println!("{}!", "world");      // "world!\n"
//!     println!("{}", 123456789);     // "123456789\n"
//! }
//! ```
extern crate proc_macro;

use proc_macro::TokenStream;

mod derive_readable;
mod fastout;

/// Derives `Readable` for your own type.
///
/// If every member of your struct implements `Readable`, your own type can also be `Readable`.
/// All you have to do is just add `#[derive_readable]` to your type definition.  This macro
/// automatically implements `Readable` to your struct and translate your struct's member type to
/// the output type of the read.  For example, if you have `Usize1` in your struct, it will
/// actually be defined as `usize`.  Of course the `Usize1`'s  `Readable` implementation is used to
/// read.
#[proc_macro_attribute]
pub fn derive_readable(attr: TokenStream, input: TokenStream) -> TokenStream {
    derive_readable::main(attr, input)
}

/// Enables buffering for stdout.
///
/// You cannot create a closure containing `print!` or `println!` in `#[fastout]` function.  This
/// is because the closure cannot implement `Send` since `StdoutLock`, which is not a `Send`, is
/// internally captured into the closure.  This causes a trait bound mismatch when used with
/// function requiring its argument closure to be a `Send`, such as `std::thread::spawn()`.
///
/// It is too conservative to make all of such closures compilation error because it is actually no
/// problem to use such a closure only inside a single thread.  However, since trait bound check is
/// done after macro expansions, there is no way to check whther the closure is required to be a
/// `Send` or not.  And the compiler error message for actual mismatch of a `Send` requirement is
/// too confusing, pointing out codes you didn't write (macro-expanded codes) as an error position.
/// In conclusion, for user-friendliness, all of them are prohibited for now.
///
/// Internally this is the same with
///
/// ```
/// let __proconio_stdout = ::std::io::stdout();
/// let mut __proconio_stdout = ::std::io::BufWriter::new(__proconio_stdout.lock());
/// let __proconio_res = {
///     // Your code goes here, with `print!` replaced by
///     //
///     // <::std::io::BufWriter<::std::io::StdoutLock> as ::std::io::Write>::write_fmt(
///     //     &mut __proconio_stdout,
///     //     format_args!(/* print! macro arguments */)
///     // ).unwrap()
///     //
/// };
/// <::std::io::BufWriter<::std::io::StdoutLock> as ::std::io::Write>::flush(
///     &mut __proconio_stdout
/// ).unwrap();
/// return __proconio_res;
/// ```
#[proc_macro_attribute]
pub fn fastout(attr: TokenStream, input: TokenStream) -> TokenStream {
    fastout::main(attr, input)
}
