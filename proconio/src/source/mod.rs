// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Defines whitespace-splitted token stream wrapping actual stream like stdin.
//!
//! The main is trait `Source`.  This is implemented to the following two type of source:
//!
//! 1. Read entire source at once.  (`once::OnceSource`)
//! 1. Read source line by line.  (`line::LineSource`)
//!
//! `OnceSource` is very fast, while `LineSource` is handy for local debugging. `OnceSource` must
//! read entire input before any other work and you must put EOF (Ctrl-D on Unix or Ctrl-Z on
//! Windows) after input.  LineSource reads source one by one.  Simply press enter to input.
//!
//! There is another source named `auto::AutoSource`.  `AutoSource` is `OnceSource` in release build,
//! is `LineSource` in debug build.  If you use debug build in local testing, `LineSource`,
//! convenience version is used.  In judge server it is compiled in release mode, so `OnceSource`,
//! faster version is used.  This is usually no problem in judging (except interactive problem?).
//!
//! If you use `input!` macro with no source specified then it uses `AutoSource` with stdin.  So,
//! locally `LineSource` are used, in the server `OnceSource` are used.  `OnceSource` and
//! `LineSource` behaves samely in point of the read result, but, unintentionally, it may differ in
//! a bare possibility. If it should differ, you can manually specify `LineSource` as `source` of
//! `input!`.
use std::io::BufRead;

pub mod line;
pub mod once;

pub mod auto {
    //! Defines `AutoSource`.
    //!
    //! It is `LineSource` for debug build, `OnceSource` for release build.

    #[cfg(debug_assertions)]
    pub use super::line::LineSource as AutoSource;
    #[cfg(not(debug_assertions))]
    pub use super::once::OnceSource as AutoSource;
}

/// The main trait. Types implementing this trait can be used for source of `input!` macro.
pub trait Source<R: BufRead> {
    /// Gets a whitespace-splitted next token.
    fn next_token(&mut self) -> Option<&str>;

    /// Check if tokens are empty
    fn is_empty(&mut self) -> bool;

    /// Force gets a whitespace-splitted next token.
    fn next_token_unwrap(&mut self) -> &str {
        self.next_token().expect("failed to get token")
    }
}

// &mut S where S: Source is also source.
impl<R: BufRead, S: Source<R>> Source<R> for &'_ mut S {
    fn next_token(&mut self) -> Option<&str> {
        (*self).next_token()
    }

    fn is_empty(&mut self) -> bool {
        (*self).is_empty()
    }
}

/// A trait representing which type can be read from `Source`.
///
/// If you want to read your own type using `input!`, you can implement this trait for your type.
/// Alternatively, you can add `#[derive_readable]` if you put `use
/// proconio_derive::derive_readable` in your source.  It automatically implements `Readable` if
/// all members of your type are `Readable`.
pub trait Readable {
    type Output;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output;
}
