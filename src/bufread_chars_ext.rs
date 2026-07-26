use std::io::BufRead;

/// An iterator over the `char`s of `BufRead`.
#[derive(Debug)]
pub struct BufReadChars<'a, T>
where
    T: ?Sized,
{
    inner: &'a mut T,
    line: String,
    curr_pos: usize,
}

impl<'a, T> BufReadChars<'a, T>
where
    T: ?Sized,
{
    // Creates a new `BufReadChars` from a reference implementing `BufRead`.
    fn new(inner: &'a mut T) -> Self {
        Self {
            inner,
            line: String::new(),
            curr_pos: 0,
        }
    }

    // Advances the iterator and returns the next char.
    fn take_char(&mut self) -> Option<char>
    where
        T: BufRead,
    {
        if self.curr_pos == self.line.len() {
            self.line.clear();
            self.curr_pos = 0;
            if self.inner.read_line(&mut self.line).ok()? == 0 {
                return None;
            }
        }
        let ch = self.line[self.curr_pos..].chars().next()?;
        self.curr_pos += ch.len_utf8();
        Some(ch)
    }
}

impl<'a, T> Iterator for BufReadChars<'a, T>
where
    T: BufRead,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_char()
    }
}

/// Extends `BufRead` trait with the method `chars` which returns an iterator over `char`s.
pub trait BufReadCharsExt: BufRead {
    /// Returns an iterator over `char`s of `BufRead`.
    fn chars(&mut self) -> BufReadChars<'_, Self> {
        BufReadChars::new(self)
    }
}

impl<T: BufRead> BufReadCharsExt for T {}
