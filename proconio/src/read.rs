// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Implements readable to the primitives and other special types (such as Chars, Bytes, ...).
//!
//! Nothing to document.

use crate::source::{Readable, Source};
use crate::types::{Bytes, Chars, Isize1, Usize1};
use std::io::BufRead;

macro_rules! impl_read_source_for_primitives {
    ($($ty:ty)*) => {
        $(
            impl Readable for $ty  {
                type Output = $ty;
                fn read<R: BufRead, S: Source<R>>(source: &mut S) -> $ty {
                    source
                        .next_token_unwrap()
                        .parse()
                        .expect("failed to parse")
                }
            }
        )*
    }
}

impl_read_source_for_primitives! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    char bool f32 f64
}

impl Readable for String {
    type Output = String;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> String {
        source.next_token_unwrap().into()
    }
}

impl Readable for Chars {
    type Output = Chars;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Chars {
        source.next_token_unwrap().chars().collect()
    }
}

impl Readable for Bytes {
    type Output = Bytes;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Bytes {
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
