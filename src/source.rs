use std::io::{BufRead, Read as _};

pub struct BufferedSource<R: BufRead> {
    buffered_source: R,
}

impl<R: BufRead> BufferedSource<R> {
    pub fn new(buffered_source: R) -> BufferedSource<R> {
        BufferedSource { buffered_source }
    }

    pub fn next_token<'a>(&'a mut self) -> impl Iterator<Item = char> + 'a {
        (&mut self.buffered_source)
            .bytes()
            .map(|x| x.expect("failed to read from source") as char)
            .skip_while(|x| x.is_whitespace())
            .take_while(|x| !x.is_whitespace())
    }
}

pub trait ReadValue<T> {
    fn read_value(&mut self) -> T;
}
