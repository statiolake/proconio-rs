pub mod line;
pub mod once;

// TODO: this should be inverted
#[cfg(not(debug_assertions))]
pub use self::line::Source;
#[cfg(debug_assertions)]
pub use self::once::Source;

use std::io::BufRead;

/// A trait representing which type can be read from `Source`.
///
/// If you want to read your own type using `input!`, you can implement this
/// trait for your type.  Alternatively, you can add `#[derive(ReadSource)]` if
/// you put `use proconio_derive::ReadSource` in your source.  It automatically
/// implements `ReadSource` if all members of your type are `ReadSource`.
pub trait ReadSource {
    type Output;
    fn read<R: BufRead>(source: &mut Source<R>) -> Self::Output;
}
