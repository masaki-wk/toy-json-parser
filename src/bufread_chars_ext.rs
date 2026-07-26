use std::io::BufRead;

/// An iterator over [`char`]s read from [`BufRead`].
#[derive(Debug)]
pub struct BufReadChars<T> {
    inner: T,
    line: String,
    curr_pos: usize,
}

impl<T> BufReadChars<T> {
    // Creates a new `BufReadChars` from a mutable reference implementing `BufRead`.
    fn new(inner: T) -> Self {
        Self {
            inner,
            line: String::new(),
            curr_pos: 0,
        }
    }

    // Advances the iterator and returns the next `char`.
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

impl<T> Iterator for BufReadChars<T>
where
    T: BufRead,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_char()
    }
}

/// Extends [`BufRead`] trait with [`chars`] method that returns an iterator over [`char`]s.
///
/// [`chars`]: BufReadCharsExt::chars
///
/// # Examples
///
/// ```
/// # use std::io::Cursor;
/// use toy_json_parser::BufReadCharsExt as _;
/// # fn test() -> Option<()> {
/// let s = String::from("bar");
/// let reader = Cursor::new(s);
/// let mut iter = reader.chars();
/// assert_eq!(iter.next(), Some('b'));
/// assert_eq!(iter.next(), Some('a'));
/// assert_eq!(iter.next(), Some('r'));
/// assert_eq!(iter.next(), None);
/// # Some(())
/// # }
/// ```
///
pub trait BufReadCharsExt: BufRead {
    /// Returns an iterator over [`char`]s read from this [`BufRead`].
    fn chars(self) -> BufReadChars<Self>
    where
        Self: Sized,
    {
        BufReadChars::new(self)
    }
}

impl<T: BufRead> BufReadCharsExt for T {}
