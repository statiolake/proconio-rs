use std::io::{BufRead, Read as _};

pub struct Source<R: BufRead> {
    source: R,
}

impl<R: BufRead> Source<R> {
    pub fn new(source: R) -> Source<R> {
        Source { source }
    }

    pub fn next_token<'a>(&'a mut self) -> impl Iterator<Item = char> + 'a {
        (&mut self.source)
            .bytes()
            .map(|x| x.expect("failed to read from source") as char)
            .skip_while(|x| x.is_whitespace())
            .take_while(|x| !x.is_whitespace())
    }
}

pub trait ReadSource {
    type Output;
    fn read<R: BufRead>(source: &mut Source<R>) -> Self::Output;
}
