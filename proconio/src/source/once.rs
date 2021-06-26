// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use super::Source;
use std::io::BufRead;
use std::iter::Peekable;
use std::marker::PhantomData;
use std::str::SplitWhitespace;

/// Source reading entire content for the first time.
///
/// It is a wrapper for `BufRead`.  You can create `OnceSource` from any type implementing
/// `BufRead`.
pub struct OnceSource<R: BufRead> {
    // Of course this is not 'static actually, but it is always valid reference
    // while entire `Source` is alive.  The actual lifetime is the context's
    // inner lifetime, and it is essentially the lifetime of self.  Also note
    // that there is no way to separate context and tokens since they are both
    // private field.
    //
    // FIXME: find nicer way.
    tokens: Peekable<SplitWhitespace<'static>>,

    // context `tokens` is reffering to
    context: Box<str>,

    // to consume `R`.  Actually `OnceSource` is not need to have `R`, since reading is done in its
    // constructor.  This is for the consistency with `LineSource` (To use smoothly through `AutoSource`).
    _read: PhantomData<R>,
}

impl<R: BufRead> OnceSource<R> {
    /// Creates `Source` using specified reader of `BufRead`.
    pub fn new(mut source: R) -> OnceSource<R> {
        let mut context = String::new();
        source
            .read_to_string(&mut context)
            .expect("failed to read from source; maybe an IO error.");

        // Boxed str is no need to check to pin.
        let context = context.into_boxed_str();

        // We can create tokens first.  But doing so causes "unused variable
        // `context`" warning (here `context` is Source::context, a member of
        // Source`). To avoid the warning at first tokens are dummy and replace
        // it using Source's context.
        let mut res = OnceSource {
            context,
            tokens: "".split_whitespace().peekable(),
            _read: PhantomData,
        };

        use std::mem;
        let context: &'static str = unsafe { mem::transmute(&*res.context) };
        res.tokens = context.split_whitespace().peekable();

        res
    }
}

impl<R: BufRead> Source<R> for OnceSource<R> {
    /// Gets a next token.
    fn next_token(&mut self) -> Option<&str> {
        self.tokens.next()
    }

    /// Check if tokens are empty
    fn is_empty(&mut self) -> bool {
        self.tokens.peek().is_none()
    }
}

use std::io::BufReader;

/// You can create `OnceSource` from `&str`.  Since `&[u8]` is a `Read`, `BufRead` can be easily
/// created by wrapping using `BufReader`.
impl<'a> From<&'a str> for OnceSource<BufReader<&'a [u8]>> {
    fn from(s: &'a str) -> OnceSource<BufReader<&'a [u8]>> {
        OnceSource::new(BufReader::new(s.as_bytes()))
    }
}
