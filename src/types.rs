//! Declares special types and aliases.

/// Chars: read a string as array of chars.
pub type Chars = Vec<char>;

/// Bytes: read a string as array of bytes.
pub type Bytes = Vec<u8>;

/// Usize1: 1-indexed usize.  Output of reading has type usize.
// Note: I want this to the empty enum (since they shouldn't be instanciated), but doing so causes
// ICE as of Rust 1.35.
pub struct Usize1;

/// Isize1: 1-indexed isize.  Output of reading has type isize.
// Note: I want this to the empty enum (since they shouldn't be instanciated), but doing so causes
// ICE as of Rust 1.35.
pub struct Isize1;
