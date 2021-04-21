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
use std::any::type_name;
use std::fmt::Debug;
use std::io::BufRead;
use std::str::FromStr;

impl<T: FromStr> Readable for T
where
    T::Err: Debug,
{
    type Output = T;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> T {
        let token = source.next_token_unwrap();
        match token.parse() {
            Ok(v) => v,
            Err(e) => panic!(
                concat!(
                    "failed to parse the input `{input}` ",
                    "to the value of type `{ty}`: {err:?}; ",
                    "ensure that the input format is collectly specified ",
                    "and that the input value must handle specified type.",
                ),
                input = token,
                ty = type_name::<T>(),
                err = e,
            ),
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
        // panic if the subtraction overflows
        usize::read(source)
            .checked_sub(1)
            .expect("attempted to read the value 0 as a Usize1")
    }
}

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
