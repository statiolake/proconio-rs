// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Declares special marker types.

use crate::source::{Readable, Source};
use std::io::BufRead;

/// Chars: read a string as array of chars.
pub enum Chars {}

impl Readable for Chars {
    type Output = Vec<char>;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Vec<char> {
        source.next_token_unwrap().chars().collect()
    }
}

/// Bytes: read a string as array of bytes.
pub enum Bytes {}

impl Readable for Bytes {
    type Output = Vec<u8>;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Vec<u8> {
        source.next_token_unwrap().bytes().collect()
    }
}

/// Usize1: 1-indexed usize.  Output of reading has type usize.
pub enum Usize1 {}

impl Readable for Usize1 {
    type Output = usize;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> usize {
        // panic if the subtraction overflows
        usize::read(source)
            .checked_sub(1)
            .expect("attempted to read the value 0 as a Usize1")
    }
}

/// Isize1: 1-indexed isize.  Output of reading has type isize.
pub enum Isize1 {}

impl Readable for Isize1 {
    type Output = isize;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> isize {
        // FIXME: Which is appropriate, forbidding all negative values or only isize::MIN. For now
        // we disallow only isize::MIN.
        // ensure the value is more than isize::MIN, or subtract overflows.
        isize::read(source).checked_sub(1).unwrap_or_else(|| {
            panic!(
                concat!(
                    "attempted to read the value {} as a Isize1:",
                    " the value is isize::MIN and cannot be decremented"
                ),
                std::isize::MIN,
            )
        })
    }
}
