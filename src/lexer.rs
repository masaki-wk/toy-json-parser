use std::iter::Peekable;
use std::ops::Range;

use crate::{CodePos, Token, TokenKind};

/// Represents a lexer.
///
/// # Examples
///
/// ```
/// # use toy_json_parser::{Lexer, TokenKind};
/// # fn test() -> Option<()> {
/// let mut lexer = Lexer::new("[]".chars());
/// let token1 = lexer.next()?;
/// assert_eq!(token1.kind, TokenKind::LeftBrace);
/// let token2 = lexer.next()?;
/// assert_eq!(token2.kind, TokenKind::RightBrace);
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

    // Reads a raw string.
    fn read_raw_string(&mut self, firstchar: char) -> String {
        let mut s = firstchar.to_string();
        while let Some(c) = self.chars.peek().copied() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.chars.next();
                self.pos.advance_column();
                s.push(c);
            } else {
                break;
            }
        }
        s
    }

    // Reads a known raw string.
    fn read_raw_string_known(&mut self, expected_tokenkind: TokenKind, expected_str: &str, firstchar: char) -> TokenKind {
        let s = self.read_raw_string(firstchar);
        if s == expected_str { expected_tokenkind } else { TokenKind::Invalid(s) }
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
        let mut s = firstchar.to_string();
        let mut has_integer_component = !is_negative;
        if !skip_integer_component {
            loop {
                match self.chars.peek() {
                    Some(c) if c.is_ascii_digit() => {
                        s.push(*c);
                        self.chars.next();
                        self.pos.advance_column();
                        has_integer_component = true;
                    }
                    _ => {
                        break;
                    }
                }
            }
        }
        if has_integer_component {
            let has_decimal_point = match self.chars.peek() {
                Some(c) if *c == '.' => {
                    s.push(*c);
                    self.chars.next();
                    self.pos.advance_column();
                    true
                }
                _ => false,
            };
            if has_decimal_point {
                let mut has_fraction_component = false;
                loop {
                    match self.chars.peek() {
                        Some(c) if c.is_ascii_digit() => {
                            s.push(*c);
                            self.chars.next();
                            self.pos.advance_column();
                            has_fraction_component = true;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                if !has_fraction_component {
                    return TokenKind::Invalid(s);
                }
            }
            let has_exponent_component = match self.chars.peek() {
                Some(c) if *c == 'e' || *c == 'E' => {
                    s.push(*c);
                    self.chars.next();
                    self.pos.advance_column();
                    true
                }
                _ => false,
            };
            if has_exponent_component {
                match self.chars.peek() {
                    Some(c) if *c == '+' || *c == '-' => {
                        s.push(*c);
                        self.chars.next();
                        self.pos.advance_column();
                    }
                    _ => {}
                }
                loop {
                    match self.chars.peek() {
                        Some(c) if c.is_ascii_digit() => {
                            s.push(*c);
                            self.chars.next();
                            self.pos.advance_column();
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            TokenKind::Number(s)
        } else {
            TokenKind::Invalid(s)
        }
    }

    // Reads a quoted string token.
    fn read_quoted_string(&mut self) -> TokenKind {
        let mut s = String::new();
        let mut failed = false;
        loop {
            if let Some(c) = self.chars.next() {
                self.pos.advance_column();
                match c {
                    '"' => {
                        break;
                    }
                    '\\' => {
                        s.push(c);
                        if let Some(c) = self.chars.next() {
                            self.pos.advance_column();
                            s.push(c);
                            match c {
                                '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' => {}
                                'u' => {
                                    for _ in 0..4 {
                                        if let Some(c) = self.chars.next() {
                                            self.pos.advance_column();
                                            s.push(c);
                                            if !c.is_ascii_hexdigit() {
                                                failed = true;
                                            }
                                        } else {
                                            failed = true;
                                            break;
                                        }
                                    }
                                }
                                _ => {
                                    failed = true;
                                }
                            }
                        } else {
                            failed = true;
                            break;
                        }
                    }
                    '\0'..'\x1f' => {
                        s.push(c);
                        failed = true;
                    }
                    _ => {
                        s.push(c);
                    }
                }
            } else {
                failed = true;
                break;
            }
        }
        if !failed {
            TokenKind::String(s)
        } else {
            TokenKind::Invalid(format!("\"{s}\""))
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
            SingleChar(TokenKind),
            KnownRawString(TokenKind, &'static str),
            QuotedString,
            Number,
            UnknownRawString,
            Invalid,
        }
        let (start, category, firstchar) = loop {
            enum CharCategory {
                WhitespaceColumn,
                WhitespaceLine,
                FirstCharOfToken(TokenCategory),
            }
            let start = self.pos;
            let ch = self.chars.next()?;
            let ch_category = match ch {
                ' ' | '\t' | '\r' => CharCategory::WhitespaceColumn,
                '\n' => CharCategory::WhitespaceLine,
                '[' => CharCategory::FirstCharOfToken(TokenCategory::SingleChar(TokenKind::LeftBracket)),
                ']' => CharCategory::FirstCharOfToken(TokenCategory::SingleChar(TokenKind::RightBracket)),
                '{' => CharCategory::FirstCharOfToken(TokenCategory::SingleChar(TokenKind::LeftBrace)),
                '}' => CharCategory::FirstCharOfToken(TokenCategory::SingleChar(TokenKind::RightBrace)),
                ':' => CharCategory::FirstCharOfToken(TokenCategory::SingleChar(TokenKind::Colon)),
                ',' => CharCategory::FirstCharOfToken(TokenCategory::SingleChar(TokenKind::Comma)),
                't' => CharCategory::FirstCharOfToken(TokenCategory::KnownRawString(TokenKind::Boolean(true), "true")),
                'f' => CharCategory::FirstCharOfToken(TokenCategory::KnownRawString(TokenKind::Boolean(false), "false")),
                'n' => CharCategory::FirstCharOfToken(TokenCategory::KnownRawString(TokenKind::Null, "null")),
                '"' => CharCategory::FirstCharOfToken(TokenCategory::QuotedString),
                '-' => CharCategory::FirstCharOfToken(TokenCategory::Number),
                c if c.is_ascii_digit() => CharCategory::FirstCharOfToken(TokenCategory::Number),
                c if c.is_ascii_alphabetic() || c == '_' => CharCategory::FirstCharOfToken(TokenCategory::UnknownRawString),
                _ => CharCategory::FirstCharOfToken(TokenCategory::Invalid),
            };
            match ch_category {
                CharCategory::WhitespaceColumn => self.pos.advance_column(),
                CharCategory::WhitespaceLine => self.pos.advance_line(),
                CharCategory::FirstCharOfToken(token_category) => {
                    self.pos.advance_column();
                    break (start, token_category, ch);
                }
            }
        };
        let kind = match category {
            TokenCategory::SingleChar(kind) => kind,
            TokenCategory::KnownRawString(kind, s) => self.read_raw_string_known(kind, s, firstchar),
            TokenCategory::QuotedString => self.read_quoted_string(),
            TokenCategory::Number => self.read_number(firstchar),
            TokenCategory::UnknownRawString => self.read_raw_string_unknown(firstchar),
            TokenCategory::Invalid => TokenKind::Invalid(firstchar.to_string()),
        };
        let end = self.pos;
        let range = Range::<CodePos> { start, end };
        Some(Token { kind, range })
    }
}
