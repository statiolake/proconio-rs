// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use super::Source;
use crate::source::tokens::Tokens;
use std::io::BufRead;

/// Source reading stream line by line.
///
/// It is a wrapper for `BufRead`.  You can create `LineSource` from any type implementing
/// `BufRead`.
pub struct LineSource<R: BufRead> {
    tokens: Tokens,
    reader: R,
}

impl<R: BufRead> LineSource<R> {
    /// Creates a `LineSource` by specified `BufRead`.
    pub fn new(reader: R) -> LineSource<R> {
        // dummy values.
        LineSource {
            tokens: "".to_owned().into(),
            reader,
        }
    }

    fn prepare(&mut self) {
        while self.tokens.is_empty() {
            let mut line = String::new();
            let num_bytes = self
                .reader
                .read_line(&mut line)
                .expect("failed to get linel maybe an IO error.");

            if num_bytes == 0 {
                // reached EOF
                return;
            }

            self.tokens = line.into();
        }
    }
}

impl<R: BufRead> Source<R> for LineSource<R> {
    /// Gets a next token.
    fn next_token(&mut self) -> Option<&str> {
        // while tokens are empty, reads a new line.
        self.prepare();
        self.tokens.next_token()
    }

    /// Check if tokens are empty
    fn is_empty(&mut self) -> bool {
        self.prepare();
        self.tokens.is_empty()
    }
}

use std::io::BufReader;

/// You can create `LineSource` from `&str`.  Since `&[u8]` is a `Read`, `BufRead` can be easily
/// created by wrapping using `BufReader`.
impl<'a> From<&'a str> for LineSource<BufReader<&'a [u8]>> {
    fn from(s: &'a str) -> LineSource<BufReader<&'a [u8]>> {
        LineSource::new(BufReader::new(s.as_bytes()))
    }
}
