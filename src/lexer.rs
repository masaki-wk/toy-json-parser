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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context as _, Result};

    fn do_tokenize(input: &str, expected_kind: TokenKind, expected_span: CodeSpan, check_finished: bool) -> Result<()> {
        let mut lexer = Lexer::new(input.chars());
        let token = lexer.next().with_context(|| "")?;
        assert_eq!(token.kind, expected_kind);
        assert_eq!(token.span, expected_span);
        if check_finished {
            assert_eq!(lexer.next(), None);
        }
        Ok(())
    }

    fn do_tokenize_single_token(input: &str, expected_kind: TokenKind) -> Result<()> {
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        do_tokenize(input, expected_kind, CodeSpan::new(start, end), true)
    }

    fn do_tokenize_single_token_with_whitespace_prefix(prefix: &str, input: &str, expected_kind: TokenKind, start: CodeLocation) -> Result<()> {
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        do_tokenize(&format!("{prefix}{input}"), expected_kind, CodeSpan::new(start, end), true)
    }

    fn do_tokenize_single_token_with_trailing_chars(body: &str, rest: &str, expected_kind: TokenKind) -> Result<()> {
        let input = &format!("{body}{rest}");
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + body.chars().count());
        do_tokenize(input, expected_kind, CodeSpan::new(start, end), false)
    }

    #[test]
    fn new() -> Result<()> {
        let _lexer = Lexer::new("".chars());
        Ok(())
    }

    #[test]
    fn tokenize_left_bracket() -> Result<()> {
        do_tokenize_single_token("[", TokenKind::Delimiter(Delimiter::LeftBracket))
    }

    #[test]
    fn tokenize_right_bracket() -> Result<()> {
        do_tokenize_single_token("]", TokenKind::Delimiter(Delimiter::RightBracket))
    }

    #[test]
    fn tokenize_left_brace() -> Result<()> {
        do_tokenize_single_token("{", TokenKind::Delimiter(Delimiter::LeftBrace))
    }

    #[test]
    fn tokenize_right_brace() -> Result<()> {
        do_tokenize_single_token("}", TokenKind::Delimiter(Delimiter::RightBrace))
    }

    #[test]
    fn tokenize_colon() -> Result<()> {
        do_tokenize_single_token(":", TokenKind::Delimiter(Delimiter::Colon))
    }

    #[test]
    fn tokenize_comma() -> Result<()> {
        do_tokenize_single_token(",", TokenKind::Delimiter(Delimiter::Comma))
    }

    #[test]
    fn tokenize_true() -> Result<()> {
        do_tokenize_single_token("true", TokenKind::Literal(Literal::Boolean(true)))
    }

    #[test]
    fn tokenize_false() -> Result<()> {
        do_tokenize_single_token("false", TokenKind::Literal(Literal::Boolean(false)))
    }

    #[test]
    fn tokenize_null() -> Result<()> {
        do_tokenize_single_token("null", TokenKind::Literal(Literal::Null))
    }

    #[test]
    fn tokenize_number_positive_zero() -> Result<()> {
        let s = "0";
        do_tokenize_single_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_negative_zero() -> Result<()> {
        let s = "-0";
        do_tokenize_single_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_positive_integer() -> Result<()> {
        let s = "123";
        do_tokenize_single_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_negative_integer() -> Result<()> {
        let s = "-123";
        do_tokenize_single_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_positive_decimal_fraction() -> Result<()> {
        let s = "12.3";
        do_tokenize_single_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_negative_decimal_fraction() -> Result<()> {
        let s = "-12.3";
        do_tokenize_single_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_positive_exponential_notation_small() -> Result<()> {
        let s = "1.23e-2";
        do_tokenize_single_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_positive_exponential_notation_large() -> Result<()> {
        let s = "1.23e+2";
        do_tokenize_single_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_negative_exponential_notation_small() -> Result<()> {
        let s = "-1.23e-2";
        do_tokenize_single_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_negative_exponential_notation_large() -> Result<()> {
        let s = "-1.23e+2";
        do_tokenize_single_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_string_without_escaped() -> Result<()> {
        let s = "foo";
        do_tokenize_single_token(&format!(r#""{s}""#), TokenKind::Literal(Literal::String(s.to_string())))
    }

    #[test]
    fn tokenize_string_with_escaped_char() -> Result<()> {
        let s = r#"\" \\ \/ \b \f \n \r \t"#;
        do_tokenize_single_token(&format!(r#""{s}""#), TokenKind::Literal(Literal::String(s.to_string())))
    }

    #[test]
    fn tokenize_string_with_escaped_unicode() -> Result<()> {
        let s = r#"\u048c"#;
        do_tokenize_single_token(&format!(r#""{s}""#), TokenKind::Literal(Literal::String(s.to_string())))
    }

    #[test]
    fn tokenize_colon_with_space_prefix() -> Result<()> {
        do_tokenize_single_token_with_whitespace_prefix(" ", ":", TokenKind::Delimiter(Delimiter::Colon), CodeLocation::new(1, 2))
    }

    #[test]
    fn tokenize_colon_with_tab_prefix() -> Result<()> {
        do_tokenize_single_token_with_whitespace_prefix("\t", ":", TokenKind::Delimiter(Delimiter::Colon), CodeLocation::new(1, 2))
    }

    #[test]
    fn tokenize_colon_with_line_feed_prefix() -> Result<()> {
        do_tokenize_single_token_with_whitespace_prefix("\n", ":", TokenKind::Delimiter(Delimiter::Colon), CodeLocation::new(2, 1))
    }

    #[test]
    fn tokenize_colon_with_carriage_return_prefix() -> Result<()> {
        do_tokenize_single_token_with_whitespace_prefix("\r", ":", TokenKind::Delimiter(Delimiter::Colon), CodeLocation::new(1, 2))
    }

    #[test]
    fn tokenize_colon_with_space_suffix() -> Result<()> {
        do_tokenize_single_token_with_trailing_chars(":", " ", TokenKind::Delimiter(Delimiter::Colon))
    }

    #[test]
    fn tokenize_true_with_space_prefix() -> Result<()> {
        do_tokenize_single_token_with_whitespace_prefix(" ", "true", TokenKind::Literal(Literal::Boolean(true)), CodeLocation::new(1, 2))
    }

    #[test]
    fn tokenize_true_with_space_suffix() -> Result<()> {
        do_tokenize_single_token_with_trailing_chars("true", " ", TokenKind::Literal(Literal::Boolean(true)))
    }

    #[test]
    fn tokenize_number_with_space_prefix() -> Result<()> {
        let s = "123";
        do_tokenize_single_token_with_whitespace_prefix(" ", s, TokenKind::Literal(Literal::Number(s.to_string())), CodeLocation::new(1, 2))
    }

    #[test]
    fn tokenize_number_with_space_suffix() -> Result<()> {
        let s = "123";
        do_tokenize_single_token_with_trailing_chars(s, " ", TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_string_with_space_prefix() -> Result<()> {
        let s = "foo";
        let input = &format!(r#""{s}""#);
        do_tokenize_single_token_with_whitespace_prefix(" ", input, TokenKind::Literal(Literal::String(s.to_string())), CodeLocation::new(1, 2))
    }

    #[test]
    fn tokenize_string_with_space_suffix() -> Result<()> {
        let s = "foo";
        let input = &format!(r#""{s}""#);
        do_tokenize_single_token_with_trailing_chars(input, " ", TokenKind::Literal(Literal::String(s.to_string())))
    }

    #[test]
    fn tokenize_invalid_char() -> Result<()> {
        let s = ".";
        do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
    }

    #[test]
    fn tokenize_invalid_raw_string() -> Result<()> {
        let s = "invalid";
        do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
    }

    #[test]
    fn tokenize_invalid_number_minus_only() -> Result<()> {
        let s = "-";
        do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
    }

    #[test]
    fn tokenize_invalid_number_leading_zero_without_minus() -> Result<()> {
        let s = "01";
        do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
    }

    #[test]
    fn tokenize_invalid_number_leading_zero_with_minus() -> Result<()> {
        let s = "-01";
        do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
    }

    #[test]
    fn tokenize_invalid_number_bad_char_in_fraction_component() -> Result<()> {
        let body = "0.";
        let rest = "a";
        do_tokenize_single_token_with_trailing_chars(body, rest, TokenKind::Invalid(body.to_string()))
    }

    #[test]
    fn tokenize_invalid_number_bad_char_in_exponent_component() -> Result<()> {
        let body = "0e";
        let rest = "a";
        do_tokenize_single_token_with_trailing_chars(body, rest, TokenKind::Invalid(body.to_string()))
    }

    #[test]
    fn tokenize_invalid_quoted_string_with_escaped_char() -> Result<()> {
        let s = r#""\c""#;
        do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
    }

    #[test]
    fn tokenize_invalid_quoted_string_with_escaped_unicode() -> Result<()> {
        let s = r#""\u000x""#;
        do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
    }

    #[test]
    fn tokenize_invalid_quoted_string_unterminated() -> Result<()> {
        let s = r#""foo"#;
        do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
    }
}
