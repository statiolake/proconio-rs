// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Implements readable to the primitive types.
//!
//! Nothing to document.

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
