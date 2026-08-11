use std::iter::Peekable;

use crate::{CodeLocation, CodeSpan, Delimiter, IteratorLocationExt as _, Literal, LocatedIterator, Token, TokenKind};

/// Represents a JSON lexer.
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
    chars: Peekable<LocatedIterator<T>>,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// Creates a new [`Lexer`].
    pub fn new(chars: T) -> Self {
        Self {
            chars: chars.locate().peekable(),
        }
    }

    // Advances the iterator and returns the next token, like `iter.next()`.
    fn take_token(&mut self) -> Option<Token> {
        enum TokenCategory {
            Delimiter(Delimiter),
            RawStringKnown(Literal, &'static str),
            RawStringUnknown,
            Number,
            QuotedString,
            Invalid,
        }
        let (category, firstchar, loc_start) = loop {
            let (loc, ch) = self.chars.next()?;
            let category_candidate = match ch {
                ' ' | '\t' | '\n' | '\r' => None,
                '[' => Some(TokenCategory::Delimiter(Delimiter::LeftBracket)),
                ']' => Some(TokenCategory::Delimiter(Delimiter::RightBracket)),
                '{' => Some(TokenCategory::Delimiter(Delimiter::LeftBrace)),
                '}' => Some(TokenCategory::Delimiter(Delimiter::RightBrace)),
                ':' => Some(TokenCategory::Delimiter(Delimiter::Colon)),
                ',' => Some(TokenCategory::Delimiter(Delimiter::Comma)),
                't' => Some(TokenCategory::RawStringKnown(Literal::Boolean(true), "true")),
                'f' => Some(TokenCategory::RawStringKnown(Literal::Boolean(false), "false")),
                'n' => Some(TokenCategory::RawStringKnown(Literal::Null, "null")),
                '"' => Some(TokenCategory::QuotedString),
                '-' => Some(TokenCategory::Number),
                _ if ch.is_ascii_digit() => Some(TokenCategory::Number),
                _ if ch.is_ascii_alphabetic() || ch == '_' => Some(TokenCategory::RawStringUnknown),
                _ => Some(TokenCategory::Invalid),
            };
            if let Some(token_category) = category_candidate {
                break (token_category, ch, loc);
            }
        };
        let (kind, loc_last) = match category {
            TokenCategory::Delimiter(delim) => (TokenKind::Delimiter(delim), loc_start),
            TokenCategory::RawStringKnown(lit, s) => self.read_raw_string_known(lit, s, firstchar, loc_start),
            TokenCategory::RawStringUnknown => self.read_raw_string_unknown(firstchar, loc_start),
            TokenCategory::Number => self.read_number(firstchar, loc_start),
            TokenCategory::QuotedString => self.read_quoted_string(loc_start),
            TokenCategory::Invalid => (TokenKind::Invalid(firstchar.to_string()), loc_start),
        };
        let loc_end = CodeLocation::new(loc_last.line, loc_last.column + 1);
        Some(Token::new(kind, CodeSpan::new(loc_start, loc_end)))
    }

    // Reads a raw string.
    fn read_raw_string(&mut self, firstchar: char, loc_start: CodeLocation) -> (String, CodeLocation) {
        let mut buf = firstchar.to_string();
        let mut loc = loc_start;
        while let Some((ch_loc, ch)) = self.chars.peek().copied() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.chars.next();
                buf.push(ch);
                loc = ch_loc;
            } else {
                break;
            }
        }
        (buf, loc)
    }

    // Reads a known raw string.
    fn read_raw_string_known(&mut self, expected_literal: Literal, expected_str: &str, firstchar: char, loc_start: CodeLocation) -> (TokenKind, CodeLocation) {
        let (s, loc) = self.read_raw_string(firstchar, loc_start);
        let kind = if s == expected_str {
            TokenKind::Literal(expected_literal)
        } else {
            TokenKind::Invalid(s)
        };
        (kind, loc)
    }

    // Reads an unknown raw string.
    fn read_raw_string_unknown(&mut self, firstchar: char, loc_start: CodeLocation) -> (TokenKind, CodeLocation) {
        let (s, loc) = self.read_raw_string(firstchar, loc_start);
        (TokenKind::Invalid(s), loc)
    }

    // Reads a number.
    fn read_number(&mut self, firstchar: char, loc_start: CodeLocation) -> (TokenKind, CodeLocation) {
        let (is_negative, mut firstchar_is_zero) = match firstchar {
            '-' => (true, false),
            '0' => (false, true),
            _ => (false, false),
        };
        let mut buf = firstchar.to_string();
        let mut loc = loc_start;
        let mut failed = false;
        let mut has_integer_component = !is_negative;
        let mut firstchar_already_read = !is_negative;
        loop {
            match self.chars.peek() {
                Some((_, ch)) if ch.is_ascii_digit() => {
                    if !firstchar_already_read {
                        if *ch == '0' {
                            firstchar_is_zero = true;
                        }
                        firstchar_already_read = true;
                    } else {
                        if firstchar_is_zero {
                            // Leading zero detected
                            failed = true;
                        }
                    }
                    buf.push(*ch);
                    loc = self.chars.next().unwrap().0;
                    has_integer_component = true;
                }
                _ => {
                    break;
                }
            }
        }
        if !has_integer_component {
            failed = true;
        }
        let has_decimal_point = match self.chars.peek() {
            Some((_, ch)) if *ch == '.' => {
                buf.push(*ch);
                loc = self.chars.next().unwrap().0;
                true
            }
            _ => false,
        };
        if has_decimal_point {
            let mut has_fraction_component = false;
            loop {
                match self.chars.peek() {
                    Some((_, ch)) if ch.is_ascii_digit() => {
                        buf.push(*ch);
                        loc = self.chars.next().unwrap().0;
                        has_fraction_component = true;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if !has_fraction_component {
                failed = true;
            }
        }
        let has_exponent_char = match self.chars.peek() {
            Some((_, ch)) if *ch == 'e' || *ch == 'E' => {
                buf.push(*ch);
                loc = self.chars.next().unwrap().0;
                true
            }
            _ => false,
        };
        if has_exponent_char {
            match self.chars.peek() {
                Some((_, ch)) if *ch == '+' || *ch == '-' => {
                    buf.push(*ch);
                    loc = self.chars.next().unwrap().0;
                }
                _ => {}
            }
            let mut has_exponent_component = false;
            loop {
                match self.chars.peek() {
                    Some((_, ch)) if ch.is_ascii_digit() => {
                        buf.push(*ch);
                        loc = self.chars.next().unwrap().0;
                        has_exponent_component = true;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if !has_exponent_component {
                failed = true;
            }
        }
        if !failed {
            (TokenKind::Literal(Literal::Number(buf)), loc)
        } else {
            (TokenKind::Invalid(buf), loc)
        }
    }

    // Reads a quoted string.
    fn read_quoted_string(&mut self, loc_start: CodeLocation) -> (TokenKind, CodeLocation) {
        let mut buf = String::new();
        let mut loc = loc_start;
        let status = (|| {
            let mut failed = false;
            loop {
                let (ch_loc, ch) = self.chars.next()?;
                loc = ch_loc;
                match ch {
                    '"' => {
                        break;
                    }
                    '\\' => {
                        buf.push(ch);
                        let (ch_loc, ch) = self.chars.next()?;
                        loc = ch_loc;
                        buf.push(ch);
                        match ch {
                            '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' => {}
                            'u' => {
                                for _ in 0..4 {
                                    let (ch_loc, ch) = self.chars.next()?;
                                    loc = ch_loc;
                                    buf.push(ch);
                                    if !ch.is_ascii_hexdigit() {
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
                        buf.push(ch);
                        failed = true;
                    }
                    _ => {
                        buf.push(ch);
                    }
                }
            }
            Some(!failed)
        })();
        match status {
            Some(true) => (TokenKind::Literal(Literal::String(buf)), loc),
            Some(false) => (TokenKind::Invalid(format!(r#""{buf}""#)), loc),
            None => (TokenKind::Invalid('"'.to_string() + &buf), loc),
        }
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_token()
    }
}
