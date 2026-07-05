use std::iter::Peekable;
use std::ops::Range;

use crate::{CodePos, Token, TokenKind};

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
        Self {
            chars: chars.peekable(),
            pos: CodePos { line: 1, column: 1 },
        }
    }

    // Advances a line.
    fn advance_line(&mut self) {
        self.pos.line += 1;
        self.pos.column = 1;
    }

    // Advances columns.
    fn advance_column(&mut self, n: usize) {
        self.pos.column += n;
    }

    // Skips whitespace.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.chars.next();
                    self.advance_column(1);
                }
                '\n' => {
                    self.chars.next();
                    self.advance_line();
                }
                _ => break,
            }
        }
    }

    // Reads a string token.
    fn read_string(&mut self) -> TokenKind {
        todo!()
    }

    // Reads a number token.
    fn read_number(&mut self) -> TokenKind {
        todo!()
    }

    // Reads a fixed string.
    fn read_fixed(&mut self, _s: &str, _expected_tokenkind: TokenKind) -> TokenKind {
        todo!()
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let start = self.pos;
        let c = self.chars.next()?;
        enum TokenCategory {
            SingleChar(TokenKind),
            FixedString(TokenKind, &'static str),
            String,
            Number,
            Invalid,
        }
        let category = match c {
            '[' => TokenCategory::SingleChar(TokenKind::LeftSquareBracket),
            ']' => TokenCategory::SingleChar(TokenKind::RightSquareBracket),
            '{' => TokenCategory::SingleChar(TokenKind::LeftCurlyBracket),
            '}' => TokenCategory::SingleChar(TokenKind::RightCurlyBracket),
            ':' => TokenCategory::SingleChar(TokenKind::Colon),
            ',' => TokenCategory::SingleChar(TokenKind::Comma),
            't' => TokenCategory::FixedString(TokenKind::Boolean(true), "true"),
            'f' => TokenCategory::FixedString(TokenKind::Boolean(false), "false"),
            'n' => TokenCategory::FixedString(TokenKind::Null, "null"),
            '"' => TokenCategory::String,
            '-' => TokenCategory::Number,
            digit if digit.is_ascii_digit() => TokenCategory::Number,
            _ => TokenCategory::Invalid,
        };
        let kind = match category {
            TokenCategory::SingleChar(kind) => {
                self.advance_column(1);
                kind
            }
            TokenCategory::FixedString(kind, s) => self.read_fixed(s, kind),
            TokenCategory::String => self.read_string(),
            TokenCategory::Number => self.read_number(),
            TokenCategory::Invalid => {
                self.advance_column(1);
                TokenKind::Invalid
            }
        };
        let end = self.pos;
        let range = Range::<CodePos> { start, end };
        Some(Token { kind, range })
    }
}
