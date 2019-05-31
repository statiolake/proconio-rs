use std::str::SplitWhitespace;

/// User input source.  If you use `input!` it obtains stdin, or if you use
/// `input_from_source!` it obtains the specified source.
pub struct Source<'a> {
    tokens: SplitWhitespace<'a>,
}

impl Source<'_> {
    /// Creates `Source` using specified reader of `BufRead`.
    pub fn new(source: &str) -> Source {
        Source {
            tokens: source.split_whitespace(),
        }
    }

    /// Gets a next token.  Return type is currently the iterator of `char`.
    pub fn next_token(&mut self) -> Option<&str> {
        self.tokens.next()
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
    fn read(source: &mut Source) -> Self::Output;
}
