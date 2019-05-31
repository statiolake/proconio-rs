use std::io::{BufRead, Read as _};

/// User input source.  If you use `input!` it obtains stdin, or if you use
/// `input_from_source!` it obtains the specified source.
pub struct Source<R: BufRead> {
    source: R,
}

impl<R: BufRead> Source<R> {
    /// Creates `Source` using specified reader of `BufRead`.
    pub fn new(source: R) -> Source<R> {
        Source { source }
    }

    /// Gets a next token.  Return type is currently the iterator of `char`.
    pub fn next_token<'a>(&'a mut self) -> impl Iterator<Item = char> + 'a {
        (&mut self.source)
            .bytes()
            .map(|x| x.expect("failed to read from source") as char)
            .skip_while(|x| x.is_whitespace())
            .take_while(|x| !x.is_whitespace())
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
    fn read<R: BufRead>(source: &mut Source<R>) -> Self::Output;
}
