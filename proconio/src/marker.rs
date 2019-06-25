// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Declares special types and aliases.

/// Chars: read a string as array of chars.
pub struct Chars;

/// Bytes: read a string as array of bytes.
pub struct Bytes;

/// Usize1: 1-indexed usize.  Output of reading has type usize.
// Note: I want this to the empty enum (since they shouldn't be instanciated), but doing so causes
// ICE as of Rust 1.35.
pub struct Usize1;

/// Isize1: 1-indexed isize.  Output of reading has type isize.
// Note: I want this to the empty enum (since they shouldn't be instanciated), but doing so causes
// ICE as of Rust 1.35.
pub struct Isize1;
