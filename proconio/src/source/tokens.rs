use std::{iter::Peekable, ptr::NonNull, str::SplitWhitespace};

pub(super) struct Tokens {
    // iterator that refers to `current_context`
    tokens: Peekable<SplitWhitespace<'static>>,

    // context `tokens` reffering to
    _current_context: CurrentContext,
}

impl Tokens {
    pub(super) fn next_token(&mut self) -> Option<&str> {
        self.tokens.next()
    }

    pub(super) fn is_empty(&mut self) -> bool {
        self.tokens.peek().is_none()
    }
}

impl From<String> for Tokens {
    fn from(current_context: String) -> Self {
        let current_context = CurrentContext::from(current_context);

        // # Safety
        //
        // - `tokens` is dropped before `current_context`.
        // - `current_context` is not accessed directly until dropped.
        unsafe {
            let tokens = current_context.0.as_ref().split_whitespace().peekable();
            Self {
                tokens,
                _current_context: current_context,
            }
        }
    }
}

// # Safety
//
// - `current_context` is not accessed directly until dropped.
// - `Box<str>: Send`.
unsafe impl Send for Tokens {}

// # Safety
//
// - `current_context` is not accessed directly until dropped.
// - `Box<str>: Sync`.
unsafe impl Sync for Tokens {}

struct CurrentContext(NonNull<str>);

impl From<String> for CurrentContext {
    fn from(s: String) -> Self {
        // deallocating the extra capacity
        let s = s.into_boxed_str();

        Self(NonNull::new(Box::leak(s)).unwrap())
    }
}

impl Drop for CurrentContext {
    fn drop(&mut self) {
        // # Safety
        //
        // The pointee should be no longer refferred.
        unsafe { Box::from_raw(self.0.as_mut()) };
    }
}
