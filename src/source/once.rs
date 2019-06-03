use super::Source;
use std::io::BufRead;
use std::marker::PhantomData;
use std::str::SplitWhitespace;

/// User input source.  If you use `input!` it obtains stdin, or if you use
/// `input_from_source!` it obtains the specified source.
pub struct OnceSource<R: BufRead> {
    // Of course this is not 'static actually, but it is always valid reference
    // while entire `Source` is alive.  The actual lifetime is the context's
    // inner lifetime, and it is essentially the lifetime of self.  Also note
    // that there is no way to separate context and tokens since they are both
    // private field.
    //
    // FIXME: find nicer way.
    tokens: SplitWhitespace<'static>,

    context: Box<str>,

    _read: PhantomData<R>,
}

impl<R: BufRead> OnceSource<R> {
    /// Creates `Source` using specified reader of `BufRead`.
    pub fn new(mut source: R) -> OnceSource<R> {
        let mut context = String::new();
        source
            .read_to_string(&mut context)
            .expect("failed to read from stdin");

        // Boxed str is no need to check to pin.
        let context = context.into_boxed_str();

        // We can create tokens first.  But doing so causes "unused variable `context`" warning
        // (here `context` is Source::context, a member of Source`). To avoid the warning at first
        // tokens are dummy and replace it using Source's context.
        let mut res = OnceSource {
            context,
            tokens: "".split_whitespace(),
            _read: PhantomData,
        };

        use std::mem;
        let context: &'static str = unsafe { mem::transmute(&*res.context) };
        mem::replace(&mut res.tokens, context.split_whitespace());

        res
    }
}

impl<R: BufRead> Source<R> for OnceSource<R> {
    /// Gets a next token.
    fn next_token(&mut self) -> Option<&str> {
        self.tokens.next()
    }
}

use std::io::BufReader;
impl<'a> From<&'a str> for OnceSource<BufReader<&'a [u8]>> {
    fn from(s: &'a str) -> OnceSource<BufReader<&'a [u8]>> {
        OnceSource::new(BufReader::new(s.as_bytes()))
    }
}
