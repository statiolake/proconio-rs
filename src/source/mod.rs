use std::io::BufRead;

pub mod line;
pub mod once;

pub mod auto {
    #[cfg(debug_assertions)]
    pub use super::line::LineSource as AutoSource;
    #[cfg(not(debug_assertions))]
    pub use super::once::OnceSource as AutoSource;
}

pub trait Source<R: BufRead> {
    /// Gets a next token.
    fn next_token(&mut self) -> Option<&str>;

    /// Force gets a next token.
    fn next_token_unwrap(&mut self) -> &str {
        self.next_token().expect("failed to get token")
    }
}

impl<R: BufRead, S: Source<R>> Source<R> for &'_ mut S {
    fn next_token(&mut self) -> Option<&str> {
        (*self).next_token()
    }
}

/// A trait representing which type can be read from `Source`.
///
/// If you want to read your own type using `input!`, you can implement this
/// trait for your type.  Alternatively, you can add `#[derive(ReadSource)]` if
/// you put `use proconio_derive::ReadSource` in your source.  It automatically
/// implements `ReadSource` if all members of your type are `ReadSource`.
pub trait ReadSource {
    type Output;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output;
}
