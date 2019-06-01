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

    /// Force gets a next token.
    pub fn next_token_unwrap(&mut self) -> &str {
        self.next_token().expect("failed to get token")
    }

    /// Gets a next token.
    pub fn next_token(&mut self) -> Option<&str> {
        self.tokens.next()
    }
}
