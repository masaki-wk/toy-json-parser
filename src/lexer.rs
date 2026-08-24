use std::fmt;
use std::iter::Peekable;

use crate::{CodeLocation, CodeSpan, Delimiter, IteratorLocationExt as _, Literal, LocatedIterator, Token, TokenKind};

/// Represents a kind of lexical error by [`Lexer`].
#[derive(Debug, PartialEq, Clone)]
pub enum LexicalErrorKind {
    /// Unexpected character was found.
    UnexpectedChar,

    /// Unquoted string was found.
    UnquotedString,

    /// String was not terminated by a closing `"`.
    UnterminatedString,

    /// Number has a leading zero before integer component digits.
    NumberContainsLeadingZero,

    /// Number was missing integer digits.
    NumberMissingIntegerDigits,

    /// Number has a decimal point but no fraction digits.
    NumberMissingFractionDigits,

    /// Number has an exponent indicator but no exponent digits.
    NumberMissingExponentDigits,

    /// String contains an unescaped control character.
    StringContainsUnescapedControlChar,

    /// String contains an invalid escape sequence.
    StringContainsInvalidEscapeSequence,

    /// String contains a `\u` escape that is not followed by four hexadecimal digits.
    StringContainsInvalidUnicodeEscape,
}

/// Represents a lexical error by [`Lexer`].
#[derive(Debug, PartialEq, Clone)]
#[allow(missing_docs)]
pub struct LexicalError {
    pub kind: LexicalErrorKind,
    pub string: String,
    pub location: CodeLocation,
}

impl LexicalError {
    /// Creates a new [`LexicalError`].
    pub const fn new(kind: LexicalErrorKind, string: String, location: CodeLocation) -> Self {
        Self { kind, string, location }
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for LexicalError {}

/// Represents a JSON lexer.
///
/// [`Lexer`] is instantiated using the [`new`] method, which takes an iterator of characters (e.g. `string.chars()`).
/// [`Lexer`] implements the [`Iterator`] trait for [`Result`] of [`Token`] or [`LexicalError`],
/// so you can call [`next()`] to retrieve tokens sequentially.
///
/// [`new`]: Lexer::new
/// [`next()`]: Iterator::next
///
/// # Examples
///
/// ```
/// # use toy_json_parser::{Delimiter, Lexer, TokenKind};
/// # fn test() -> Option<()> {
/// let mut lexer = Lexer::new("[]".chars());
///
/// // Get the first token `[`
/// let token1 = lexer.next()?.ok()?;
/// assert_eq!(token1.kind, TokenKind::Delimiter(Delimiter::LeftBracket));
///
/// // Get the second token `]`
/// let token2 = lexer.next()?.ok()?;
/// assert_eq!(token2.kind, TokenKind::Delimiter(Delimiter::RightBracket));
///
/// // No more tokens
/// assert_eq!(lexer.next(), None);
///
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

    // Advances the iterator and returns the next result, like `iter.next()`.
    fn take_token(&mut self) -> Option<Result<Token, LexicalError>> {
        enum TokenCategory {
            Delimiter(Delimiter),
            UnquotedStringKnown(Literal, &'static str),
            UnquotedStringUnknown,
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
                't' => Some(TokenCategory::UnquotedStringKnown(Literal::Boolean(true), "true")),
                'f' => Some(TokenCategory::UnquotedStringKnown(Literal::Boolean(false), "false")),
                'n' => Some(TokenCategory::UnquotedStringKnown(Literal::Null, "null")),
                '"' => Some(TokenCategory::QuotedString),
                '-' => Some(TokenCategory::Number),
                _ if ch.is_ascii_digit() => Some(TokenCategory::Number),
                _ if ch.is_ascii_alphabetic() || ch == '_' => Some(TokenCategory::UnquotedStringUnknown),
                _ => Some(TokenCategory::Invalid),
            };
            if let Some(token_category) = category_candidate {
                break (token_category, ch, loc);
            }
        };
        let result = match category {
            TokenCategory::Delimiter(delim) => Ok((TokenKind::Delimiter(delim), loc_start)),
            TokenCategory::UnquotedStringKnown(lit, s) => self.read_unquoted_string_known(lit, s, firstchar, loc_start),
            TokenCategory::UnquotedStringUnknown => Err(self.read_unquoted_string_unknown(firstchar, loc_start)),
            TokenCategory::Number => self.read_number(firstchar, loc_start),
            TokenCategory::QuotedString => self.read_quoted_string(loc_start),
            TokenCategory::Invalid => Err((LexicalErrorKind::UnexpectedChar, firstchar.to_string())),
        };
        match result {
            Ok((kind, loc_last)) => {
                let loc_end = CodeLocation::new(loc_last.line, loc_last.column + 1);
                Some(Ok(Token::new(kind, CodeSpan::new(loc_start, loc_end))))
            }
            Err((kind, s)) => Some(Err(LexicalError::new(kind, s, loc_start))),
        }
    }

    // Reads an unquoted string.
    fn read_unquoted_string(&mut self, firstchar: char, loc_start: CodeLocation) -> (String, CodeLocation) {
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

    // Reads a known unquoted string.
    fn read_unquoted_string_known(
        &mut self,
        expected_literal: Literal,
        expected_str: &str,
        firstchar: char,
        loc_start: CodeLocation,
    ) -> Result<(TokenKind, CodeLocation), (LexicalErrorKind, String)> {
        let (s, loc) = self.read_unquoted_string(firstchar, loc_start);
        if s == expected_str {
            Ok((TokenKind::Literal(expected_literal), loc))
        } else {
            Err((LexicalErrorKind::UnquotedString, s))
        }
    }

    // Reads an unknown unquoted string.
    fn read_unquoted_string_unknown(&mut self, firstchar: char, loc_start: CodeLocation) -> (LexicalErrorKind, String) {
        let (s, _) = self.read_unquoted_string(firstchar, loc_start);
        (LexicalErrorKind::UnquotedString, s)
    }

    // Reads a number.
    fn read_number(&mut self, firstchar: char, loc_start: CodeLocation) -> Result<(TokenKind, CodeLocation), (LexicalErrorKind, String)> {
        let (is_negative, mut firstchar_is_zero) = match firstchar {
            '-' => (true, false),
            '0' => (false, true),
            _ => (false, false),
        };
        let mut buf = firstchar.to_string();
        let mut loc = loc_start;
        let mut error = None;
        let mut has_integer_digits = !is_negative;
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
                            error = Some(LexicalErrorKind::NumberContainsLeadingZero);
                        }
                    }
                    buf.push(*ch);
                    loc = self.chars.next().unwrap().0;
                    has_integer_digits = true;
                }
                _ => {
                    break;
                }
            }
        }
        if !has_integer_digits {
            error = Some(LexicalErrorKind::NumberMissingIntegerDigits);
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
            let mut has_fraction_digits = false;
            loop {
                match self.chars.peek() {
                    Some((_, ch)) if ch.is_ascii_digit() => {
                        buf.push(*ch);
                        loc = self.chars.next().unwrap().0;
                        has_fraction_digits = true;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if !has_fraction_digits {
                error = Some(LexicalErrorKind::NumberMissingFractionDigits);
            }
        }
        let has_exponent_letter = match self.chars.peek() {
            Some((_, ch)) if *ch == 'e' || *ch == 'E' => {
                buf.push(*ch);
                loc = self.chars.next().unwrap().0;
                true
            }
            _ => false,
        };
        if has_exponent_letter {
            match self.chars.peek() {
                Some((_, ch)) if *ch == '+' || *ch == '-' => {
                    buf.push(*ch);
                    loc = self.chars.next().unwrap().0;
                }
                _ => {}
            }
            let mut has_exponent_digits = false;
            loop {
                match self.chars.peek() {
                    Some((_, ch)) if ch.is_ascii_digit() => {
                        buf.push(*ch);
                        loc = self.chars.next().unwrap().0;
                        has_exponent_digits = true;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if !has_exponent_digits {
                error = Some(LexicalErrorKind::NumberMissingExponentDigits);
            }
        }
        match error {
            Some(kind) => Err((kind, buf)),
            None => Ok((TokenKind::Literal(Literal::Number(buf)), loc)),
        }
    }

    // Reads a quoted string.
    fn read_quoted_string(&mut self, loc_start: CodeLocation) -> Result<(TokenKind, CodeLocation), (LexicalErrorKind, String)> {
        let mut buf = String::new();
        let mut loc = loc_start;
        let status = (|| {
            let mut error = None;
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
                                        error = Some(LexicalErrorKind::StringContainsInvalidUnicodeEscape);
                                    }
                                }
                            }
                            _ => {
                                error = Some(LexicalErrorKind::StringContainsInvalidEscapeSequence);
                            }
                        }
                    }
                    '\0'..'\x1f' => {
                        buf.push(ch);
                        error = Some(LexicalErrorKind::StringContainsUnescapedControlChar);
                    }
                    _ => {
                        buf.push(ch);
                    }
                }
            }
            Some(error)
        })();
        match status {
            Some(None) => Ok((TokenKind::Literal(Literal::String(buf)), loc)),
            Some(Some(kind)) => Err((kind, format!(r#""{buf}""#))),
            None => Err((LexicalErrorKind::UnterminatedString, '"'.to_string() + &buf)),
        }
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = Result<Token, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_take_single_valid_token(input: &str, expected_kind: TokenKind, expected_span: CodeSpan) {
        let mut lexer = Lexer::new(input.chars());
        let token = lexer.next().unwrap().unwrap();
        assert_eq!(token.kind, expected_kind);
        assert_eq!(token.span, expected_span);
        assert_eq!(lexer.next(), None);
    }

    fn do_take_single_invalid_token(input: &str, expected_kind: LexicalErrorKind, expected_string: &str, to_be_finished: bool) {
        let start = CodeLocation::new(1, 1);
        let mut lexer = Lexer::new(input.chars());
        let error = lexer.next().unwrap().unwrap_err();
        assert_eq!(error.kind, expected_kind);
        assert_eq!(error.string, expected_string);
        assert_eq!(error.location, start);
        assert_eq!(lexer.next().is_none(), to_be_finished);
    }

    fn take_single_valid_token(input: &str, expected_kind: TokenKind) {
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        do_take_single_valid_token(input, expected_kind, CodeSpan::new(start, end))
    }

    fn take_single_valid_token_with_whitespace_prefix(prefix: &str, input: &str, expected_kind: TokenKind, start: CodeLocation) {
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        do_take_single_valid_token(&format!("{prefix}{input}"), expected_kind, CodeSpan::new(start, end))
    }

    fn take_single_valid_token_with_whitespace_suffix(input: &str, suffix: &str, expected_kind: TokenKind) {
        let joined = &format!("{input}{suffix}");
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        do_take_single_valid_token(joined, expected_kind, CodeSpan::new(start, end))
    }

    fn take_single_invalid_token(input: &str, expected_kind: LexicalErrorKind) {
        do_take_single_invalid_token(input, expected_kind, input, true)
    }

    fn take_single_invalid_token_with_trailing_chars(input: &str, rest: &str, expected_kind: LexicalErrorKind) {
        let joined = &format!("{input}{rest}");
        do_take_single_invalid_token(joined, expected_kind, input, false)
    }

    #[test]
    fn new() {
        let _lexer = Lexer::new("".chars());
    }

    #[test]
    fn tokenize_left_bracket() {
        take_single_valid_token("[", TokenKind::Delimiter(Delimiter::LeftBracket))
    }

    #[test]
    fn tokenize_right_bracket() {
        take_single_valid_token("]", TokenKind::Delimiter(Delimiter::RightBracket))
    }

    #[test]
    fn tokenize_left_brace() {
        take_single_valid_token("{", TokenKind::Delimiter(Delimiter::LeftBrace))
    }

    #[test]
    fn tokenize_right_brace() {
        take_single_valid_token("}", TokenKind::Delimiter(Delimiter::RightBrace))
    }

    #[test]
    fn tokenize_colon() {
        take_single_valid_token(":", TokenKind::Delimiter(Delimiter::Colon))
    }

    #[test]
    fn tokenize_comma() {
        take_single_valid_token(",", TokenKind::Delimiter(Delimiter::Comma))
    }

    #[test]
    fn tokenize_true() {
        take_single_valid_token("true", TokenKind::Literal(Literal::Boolean(true)))
    }

    #[test]
    fn tokenize_false() {
        take_single_valid_token("false", TokenKind::Literal(Literal::Boolean(false)))
    }

    #[test]
    fn tokenize_null() {
        take_single_valid_token("null", TokenKind::Literal(Literal::Null))
    }

    #[test]
    fn tokenize_number_positive_zero() {
        let s = "0";
        take_single_valid_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_negative_zero() {
        let s = "-0";
        take_single_valid_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_positive_integer() {
        let s = "123";
        take_single_valid_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_negative_integer() {
        let s = "-123";
        take_single_valid_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_positive_decimal_fraction() {
        let s = "12.3";
        take_single_valid_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_negative_decimal_fraction() {
        let s = "-12.3";
        take_single_valid_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_positive_exponential_notation_small() {
        let s = "1.23e-2";
        take_single_valid_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_positive_exponential_notation_large() {
        let s = "1.23e+2";
        take_single_valid_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_negative_exponential_notation_small() {
        let s = "-1.23e-2";
        take_single_valid_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_negative_exponential_notation_large() {
        let s = "-1.23e+2";
        take_single_valid_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_string_without_escaped() {
        let s = "foo";
        take_single_valid_token(&format!(r#""{s}""#), TokenKind::Literal(Literal::String(s.to_string())))
    }

    #[test]
    fn tokenize_string_with_escaped_char() {
        let s = r#"\" \\ \/ \b \f \n \r \t"#;
        take_single_valid_token(&format!(r#""{s}""#), TokenKind::Literal(Literal::String(s.to_string())))
    }

    #[test]
    fn tokenize_string_with_escaped_unicode() {
        let s = r#"\u048c"#;
        take_single_valid_token(&format!(r#""{s}""#), TokenKind::Literal(Literal::String(s.to_string())))
    }

    #[test]
    fn tokenize_colon_with_space_prefix() {
        take_single_valid_token_with_whitespace_prefix(" ", ":", TokenKind::Delimiter(Delimiter::Colon), CodeLocation::new(1, 2))
    }

    #[test]
    fn tokenize_colon_with_tab_prefix() {
        take_single_valid_token_with_whitespace_prefix("\t", ":", TokenKind::Delimiter(Delimiter::Colon), CodeLocation::new(1, 2))
    }

    #[test]
    fn tokenize_colon_with_line_feed_prefix() {
        take_single_valid_token_with_whitespace_prefix("\n", ":", TokenKind::Delimiter(Delimiter::Colon), CodeLocation::new(2, 1))
    }

    #[test]
    fn tokenize_colon_with_carriage_return_prefix() {
        take_single_valid_token_with_whitespace_prefix("\r", ":", TokenKind::Delimiter(Delimiter::Colon), CodeLocation::new(1, 2))
    }

    #[test]
    fn tokenize_colon_with_space_suffix() {
        take_single_valid_token_with_whitespace_suffix(":", " ", TokenKind::Delimiter(Delimiter::Colon))
    }

    #[test]
    fn tokenize_true_with_space_prefix() {
        take_single_valid_token_with_whitespace_prefix(" ", "true", TokenKind::Literal(Literal::Boolean(true)), CodeLocation::new(1, 2))
    }

    #[test]
    fn tokenize_true_with_space_suffix() {
        take_single_valid_token_with_whitespace_suffix("true", " ", TokenKind::Literal(Literal::Boolean(true)))
    }

    #[test]
    fn tokenize_number_with_space_prefix() {
        let s = "123";
        take_single_valid_token_with_whitespace_prefix(" ", s, TokenKind::Literal(Literal::Number(s.to_string())), CodeLocation::new(1, 2))
    }

    #[test]
    fn tokenize_number_with_space_suffix() {
        let s = "123";
        take_single_valid_token_with_whitespace_suffix(s, " ", TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_string_with_space_prefix() {
        let s = "foo";
        let input = &format!(r#""{s}""#);
        take_single_valid_token_with_whitespace_prefix(" ", input, TokenKind::Literal(Literal::String(s.to_string())), CodeLocation::new(1, 2))
    }

    #[test]
    fn tokenize_string_with_space_suffix() {
        let s = "foo";
        let input = &format!(r#""{s}""#);
        take_single_valid_token_with_whitespace_suffix(input, " ", TokenKind::Literal(Literal::String(s.to_string())))
    }

    #[test]
    fn tokenize_invalid_char() {
        let s = ".";
        take_single_invalid_token(s, LexicalErrorKind::UnexpectedChar)
    }

    #[test]
    fn tokenize_invalid_unquoted_string() {
        let s = "invalid";
        take_single_invalid_token(s, LexicalErrorKind::UnquotedString)
    }

    #[test]
    fn tokenize_invalid_unterminated_string() {
        let s = r#""foo"#;
        take_single_invalid_token(s, LexicalErrorKind::UnterminatedString)
    }

    #[test]
    fn tokenize_invalid_number_minus_only() {
        let s = "-";
        take_single_invalid_token(s, LexicalErrorKind::NumberMissingIntegerDigits)
    }

    #[test]
    fn tokenize_invalid_number_leading_zero_without_minus() {
        let s = "01";
        take_single_invalid_token(s, LexicalErrorKind::NumberContainsLeadingZero)
    }

    #[test]
    fn tokenize_invalid_number_leading_zero_with_minus() {
        let s = "-01";
        take_single_invalid_token(s, LexicalErrorKind::NumberContainsLeadingZero)
    }

    #[test]
    fn tokenize_invalid_number_bad_char_in_fraction_part() {
        let body = "0.";
        let rest = "a";
        take_single_invalid_token_with_trailing_chars(body, rest, LexicalErrorKind::NumberMissingFractionDigits)
    }

    #[test]
    fn tokenize_invalid_number_bad_char_in_exponent_part() {
        let body = "0e";
        let rest = "a";
        take_single_invalid_token_with_trailing_chars(body, rest, LexicalErrorKind::NumberMissingExponentDigits)
    }

    #[test]
    fn tokenize_invalid_quoted_string_with_bad_escape_sequence() {
        let s = r#""\c""#;
        take_single_invalid_token(s, LexicalErrorKind::StringContainsInvalidEscapeSequence)
    }

    #[test]
    fn tokenize_invalid_quoted_string_with_bad_unicode_escape() {
        let s = r#""\u000x""#;
        take_single_invalid_token(s, LexicalErrorKind::StringContainsInvalidUnicodeEscape)
    }

    #[test]
    fn tokenize_invalid_quoted_string_with_unescaped_control_char() {
        let s = "\"\n\"";
        take_single_invalid_token(s, LexicalErrorKind::StringContainsUnescapedControlChar)
    }
}
