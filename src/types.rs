//! Declares special types and aliases.

/// Chars: read a string as array of chars.
pub type Chars = Vec<char>;

/// Bytes: read a string as array of bytes.
pub type Bytes = Vec<u8>;

/// Usize1: 1-indexed usize.  Output of reading has type usize.
#[derive(Debug)]
pub struct Usize1;

/// Isize1: 1-indexed isize.  Output of reading has type isize.
#[derive(Debug)]
pub struct Isize1;
