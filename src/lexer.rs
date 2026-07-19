use std::iter::Peekable;

use crate::{CodePos, Delimiter, Literal, Token, TokenKind};

/// Represents a lexer.
///
/// # Examples
///
/// ```
/// # use toy_json_parser::{Delimiter, Lexer, TokenKind};
/// # fn test() -> Option<()> {
/// let mut lexer = Lexer::new("[]".chars());
/// let token1 = lexer.next()?;
/// assert_eq!(token1.kind, TokenKind::Delimiter(Delimiter::LeftBrace));
/// let token2 = lexer.next()?;
/// assert_eq!(token2.kind, TokenKind::Delimiter(Delimiter::RightBrace));
/// # Some(())
/// # }
/// ```
///
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

    // The implementation of `chars_next_and_then_advance_column` and `chars_next_and_then_advance_auto`.
    fn chars_next_and_then_advance(&mut self, always_column: bool) -> Option<T::Item> {
        let c = self.chars.next()?;
        if always_column || c != '\n' {
            self.pos.advance_column();
        } else {
            self.pos.advance_line();
        }
        Some(c)
    }

    // Returns the result of `self.chars.next()`, and advances the column of the position.
    fn chars_next_and_then_advance_column(&mut self) -> Option<T::Item> {
        self.chars_next_and_then_advance(true)
    }

    // Returns the result of `self.chars.next()`, and advances the position automatically.
    fn chars_next_and_then_advance_auto(&mut self) -> Option<T::Item> {
        self.chars_next_and_then_advance(false)
    }

    // Reads a raw string.
    fn read_raw_string(&mut self, firstchar: char) -> String {
        let mut buf = firstchar.to_string();
        while let Some(c) = self.chars.peek().copied() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.chars_next_and_then_advance_column();
                buf.push(c);
            } else {
                break;
            }
        }
        buf
    }

    // Reads a known raw string.
    fn read_raw_string_known(&mut self, expected_literal: Literal, expected_str: &str, firstchar: char) -> TokenKind {
        let s = self.read_raw_string(firstchar);
        if s == expected_str {
            TokenKind::Literal(expected_literal)
        } else {
            TokenKind::Invalid(s)
        }
    }

    // Reads an unknown raw string.
    fn read_raw_string_unknown(&mut self, firstchar: char) -> TokenKind {
        TokenKind::Invalid(self.read_raw_string(firstchar))
    }

    // Reads a number token.
    fn read_number(&mut self, firstchar: char) -> TokenKind {
        let (is_negative, skip_integer_component) = match firstchar {
            '-' => (true, false),
            '0' => (false, true),
            _ => (false, false),
        };
        let mut buf = firstchar.to_string();
        if !skip_integer_component {
            let mut has_integer_component = !is_negative;
            loop {
                match self.chars.peek() {
                    Some(c) if c.is_ascii_digit() => {
                        buf.push(*c);
                        self.chars_next_and_then_advance_column();
                        has_integer_component = true;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if !has_integer_component {
                return TokenKind::Invalid(buf);
            }
        }
        let has_decimal_point = match self.chars.peek() {
            Some(c) if *c == '.' => {
                buf.push(*c);
                self.chars_next_and_then_advance_column();
                true
            }
            _ => false,
        };
        if has_decimal_point {
            let mut has_fraction_component = false;
            loop {
                match self.chars.peek() {
                    Some(c) if c.is_ascii_digit() => {
                        buf.push(*c);
                        self.chars_next_and_then_advance_column();
                        has_fraction_component = true;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if !has_fraction_component {
                return TokenKind::Invalid(buf);
            }
        }
        let has_exponent_char = match self.chars.peek() {
            Some(c) if *c == 'e' || *c == 'E' => {
                buf.push(*c);
                self.chars_next_and_then_advance_column();
                true
            }
            _ => false,
        };
        if has_exponent_char {
            match self.chars.peek() {
                Some(c) if *c == '+' || *c == '-' => {
                    buf.push(*c);
                    self.chars_next_and_then_advance_column();
                }
                _ => {}
            }
            let mut has_exponent_component = false;
            loop {
                match self.chars.peek() {
                    Some(c) if c.is_ascii_digit() => {
                        buf.push(*c);
                        self.chars_next_and_then_advance_column();
                        has_exponent_component = true;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if !has_exponent_component {
                return TokenKind::Invalid(buf);
            }
        }
        TokenKind::Literal(Literal::Number(buf))
    }

    // Reads a quoted string token.
    fn read_quoted_string(&mut self) -> TokenKind {
        let mut buf = String::new();
        let status = (|| {
            let mut failed = false;
            loop {
                let c = self.chars_next_and_then_advance_auto()?;
                match c {
                    '"' => {
                        break;
                    }
                    '\\' => {
                        buf.push(c);
                        let c = self.chars_next_and_then_advance_auto()?;
                        buf.push(c);
                        match c {
                            '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' => {}
                            'u' => {
                                for _ in 0..4 {
                                    let c = self.chars_next_and_then_advance_auto()?;
                                    buf.push(c);
                                    if !c.is_ascii_hexdigit() {
                                        failed = true;
                                    }
                                }
                            }
                            _ => {
                                failed = true;
                            }
                        }
                    }
                    '\0'..'\x1f' => {
                        buf.push(c);
                        failed = true;
                    }
                    _ => {
                        buf.push(c);
                    }
                }
            }
            Some(!failed)
        })();
        match status {
            Some(true) => TokenKind::Literal(Literal::String(buf)),
            Some(false) => TokenKind::Invalid(format!("\"{buf}\"")),
            None => TokenKind::Invalid('"'.to_string() + &buf),
        }
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        enum TokenCategory {
            Delimiter(Delimiter),
            RawStringKnown(Literal, &'static str),
            RawStringUnknown,
            Number,
            QuotedString,
            Invalid,
        }
        let (pos, category, firstchar) = loop {
            enum CharCategory {
                Whitespace,
                FirstCharOfToken(TokenCategory),
            }
            let pos = self.pos;
            let ch = self.chars_next_and_then_advance_auto()?;
            let ch_category = match ch {
                ' ' | '\t' | '\n' | '\r' => CharCategory::Whitespace,
                '[' => CharCategory::FirstCharOfToken(TokenCategory::Delimiter(Delimiter::LeftBracket)),
                ']' => CharCategory::FirstCharOfToken(TokenCategory::Delimiter(Delimiter::RightBracket)),
                '{' => CharCategory::FirstCharOfToken(TokenCategory::Delimiter(Delimiter::LeftBrace)),
                '}' => CharCategory::FirstCharOfToken(TokenCategory::Delimiter(Delimiter::RightBrace)),
                ':' => CharCategory::FirstCharOfToken(TokenCategory::Delimiter(Delimiter::Colon)),
                ',' => CharCategory::FirstCharOfToken(TokenCategory::Delimiter(Delimiter::Comma)),
                't' => CharCategory::FirstCharOfToken(TokenCategory::RawStringKnown(Literal::Boolean(true), "true")),
                'f' => CharCategory::FirstCharOfToken(TokenCategory::RawStringKnown(Literal::Boolean(false), "false")),
                'n' => CharCategory::FirstCharOfToken(TokenCategory::RawStringKnown(Literal::Null, "null")),
                '"' => CharCategory::FirstCharOfToken(TokenCategory::QuotedString),
                '-' => CharCategory::FirstCharOfToken(TokenCategory::Number),
                _ if ch.is_ascii_digit() => CharCategory::FirstCharOfToken(TokenCategory::Number),
                _ if ch.is_ascii_alphabetic() || ch == '_' => CharCategory::FirstCharOfToken(TokenCategory::RawStringUnknown),
                _ => CharCategory::FirstCharOfToken(TokenCategory::Invalid),
            };
            if let CharCategory::FirstCharOfToken(token_category) = ch_category {
                break (pos, token_category, ch);
            }
        };
        let kind = match category {
            TokenCategory::Delimiter(delim) => TokenKind::Delimiter(delim),
            TokenCategory::RawStringKnown(lit, s) => self.read_raw_string_known(lit, s, firstchar),
            TokenCategory::RawStringUnknown => self.read_raw_string_unknown(firstchar),
            TokenCategory::Number => self.read_number(firstchar),
            TokenCategory::QuotedString => self.read_quoted_string(),
            TokenCategory::Invalid => TokenKind::Invalid(firstchar.to_string()),
        };
        Some(Token { kind, pos })
    }
}
