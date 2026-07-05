use std::iter::Peekable;

use crate::{CodePos, Token};

/// Represents a lexer.
#[derive(Debug, Clone)]
pub struct Lexer<T>
where
    T: Iterator<Item = char>,
{
    chars: Peekable<T>,
    pos: CodePos,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// Creates a new lexer.
    pub fn new(chars: T) -> Self {
        todo!()
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
