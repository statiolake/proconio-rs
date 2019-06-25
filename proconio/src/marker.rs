// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Declares special marker types.

/// Chars: read a string as array of chars.
pub enum Chars {}

/// Bytes: read a string as array of bytes.
pub enum Bytes {}

/// Usize1: 1-indexed usize.  Output of reading has type usize.
pub enum Usize1 {}

/// Isize1: 1-indexed isize.  Output of reading has type isize.
pub enum Isize1 {}
