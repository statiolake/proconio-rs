use std::io::{BufRead, Read};

pub struct BufferedSource<R: BufRead> {
    buffered_source: R,
}

impl<R: BufRead> BufferedSource<R> {
    pub fn next_token<'a>(&'a mut self) -> impl Iterator<Item = char> + 'a {
        (&mut self.buffered_source)
            .bytes()
            .map(|x| x.expect("failed to read from source") as char)
            .skip_while(|x| x.is_whitespace())
            .take_while(|x| !x.is_whitespace())
    }

    pub fn new(buffered_source: R) -> BufferedSource<R> {
        BufferedSource { buffered_source }
    }
}
