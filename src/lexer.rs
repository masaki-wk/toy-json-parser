use std::ops::Range;

use crate::{CodePos, Token, TokenKind};

/// Represents a lexer.
#[derive(Debug, Clone)]
pub struct Lexer<T>
where
    T: Iterator<Item = char>,
{
    chars: T,
    pos: CodePos,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// Creates a new lexer.
    pub fn new(chars: T) -> Self {
        Self {
            chars,
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
        enum TokenCategory {
            SingleChar(TokenKind),
            FixedString(TokenKind, &'static str),
            String,
            Number,
            Invalid,
        }
        let (start, category) = loop {
            enum CharCategory {
                WhitespaceColumn,
                WhitespaceLine,
                FirstCharOfToken(TokenCategory),
            }
            let start = self.pos;
            let c = self.chars.next()?;
            let char_category = match c {
                ' ' | '\t' | '\r' => CharCategory::WhitespaceColumn,
                '\n' => CharCategory::WhitespaceLine,
                '[' => CharCategory::FirstCharOfToken(TokenCategory::SingleChar(TokenKind::LeftSquareBracket)),
                ']' => CharCategory::FirstCharOfToken(TokenCategory::SingleChar(TokenKind::RightSquareBracket)),
                '{' => CharCategory::FirstCharOfToken(TokenCategory::SingleChar(TokenKind::LeftCurlyBracket)),
                '}' => CharCategory::FirstCharOfToken(TokenCategory::SingleChar(TokenKind::RightCurlyBracket)),
                ':' => CharCategory::FirstCharOfToken(TokenCategory::SingleChar(TokenKind::Colon)),
                ',' => CharCategory::FirstCharOfToken(TokenCategory::SingleChar(TokenKind::Comma)),
                't' => CharCategory::FirstCharOfToken(TokenCategory::FixedString(TokenKind::Boolean(true), "true")),
                'f' => CharCategory::FirstCharOfToken(TokenCategory::FixedString(TokenKind::Boolean(false), "false")),
                'n' => CharCategory::FirstCharOfToken(TokenCategory::FixedString(TokenKind::Null, "null")),
                '"' => CharCategory::FirstCharOfToken(TokenCategory::String),
                '-' => CharCategory::FirstCharOfToken(TokenCategory::Number),
                digit if digit.is_ascii_digit() => CharCategory::FirstCharOfToken(TokenCategory::Number),
                _ => CharCategory::FirstCharOfToken(TokenCategory::Invalid),
            };
            match char_category {
                CharCategory::WhitespaceColumn => self.advance_column(1),
                CharCategory::WhitespaceLine => self.advance_line(),
                CharCategory::FirstCharOfToken(token_category) => {
                    break (start, token_category);
                }
            }
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
