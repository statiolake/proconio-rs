// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Implements readable to the primitives and other special types (such as Chars, Bytes, ...).
//!
//! Nothing to document.

use crate::marker::{Bytes, Chars, Isize1, Usize1};
use crate::source::{Readable, Source};
use std::io::BufRead;
use std::str::FromStr;

impl<T: FromStr> Readable for T {
    type Output = T;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> T {
        match source.next_token_unwrap().parse() {
            Ok(v) => v,
            Err(_e) => panic!("failed to parse input."),
        }
    }
}

impl Readable for Chars {
    type Output = Vec<char>;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Vec<char> {
        source.next_token_unwrap().chars().collect()
    }
}

impl Readable for Bytes {
    type Output = Vec<u8>;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Vec<u8> {
        source.next_token_unwrap().bytes().collect()
    }
}

impl Readable for Usize1 {
    type Output = usize;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> usize {
        usize::read(source) - 1
    }
}

impl Readable for Isize1 {
    type Output = isize;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> isize {
        isize::read(source) - 1
    }
}
