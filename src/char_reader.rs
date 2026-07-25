use std::io::BufRead;

/// Represents an iterator over chars read from a `BufRead`.
pub struct CharReader<R> {
    inner: R,
    line: String,
    curr_pos: usize,
}

impl<R> CharReader<R> {
    /// Creates a new `CharReader<R>` from a value implementing `BufRead`.
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            line: String::new(),
            curr_pos: 0,
        }
    }
}

impl<R> CharReader<R>
where
    R: BufRead,
{
    // Advances the iterator and returns the next char.
    fn take_char(&mut self) -> Option<char> {
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

impl<R> Iterator for CharReader<R>
where
    R: BufRead,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_char()
    }
}
