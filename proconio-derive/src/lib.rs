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
/// Internally this is the same with
///
/// ```
/// use std::io::Write as _;
/// let __proconio_stdout = std::io::stdout();
/// let mut __proconio_stdout = std::io::BufWriter::new(std::io::stdout());
/// let __proconio_res = {
///     // Your code goes here, but `print!(...)` is replaced by
///     // `write!(__proconio_stdout, ...).unwrap();`.  The same goes for `println!`.
/// };
/// __proconio_stdout.flush().unwrap();
/// return __proconio_res;
/// ```
#[proc_macro_attribute]
pub fn fastout(attr: TokenStream, input: TokenStream) -> TokenStream {
    fastout::main(attr, input)
}
