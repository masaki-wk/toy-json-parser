use std::iter::Peekable;

use crate::{CodeLocation, CodeSpan, Delimiter, IteratorLocationExt as _, LexicalError, LexicalErrorKind, Literal, LocatedIterator, Token, TokenKind};

/// Represents a JSON lexer.
///
/// [`Lexer`] is instantiated using the [`new`] method, which takes an iterator over [`char`]s (e.g. `string.chars()`).
/// [`Lexer`] implements the [`Iterator`] trait with `Item = Result<Token, LexicalError>`,
/// so you can call [`next()`] to retrieve tokens sequentially.
///
/// [`next()`] returns one of the following:
///
/// - `Some(Ok(Token))`: A [`Token`] was successfully produced.
/// - `Some(Err(LexicalError))`: A [`LexicalError`] occurred.
///   Iteration may continue, and subsequent calls may return more tokens or errors.
/// - `None`: Neither a token nor an error was produced because the underlying character iterator is exhausted.
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
/// assert_eq!(*token1.kind(), TokenKind::Delimiter(Delimiter::LeftBracket));
///
/// // Get the second token `]`
/// let token2 = lexer.next()?.ok()?;
/// assert_eq!(*token2.kind(), TokenKind::Delimiter(Delimiter::RightBracket));
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
            TokenCategory::Delimiter(delim) => Ok((TokenKind::Delimiter(delim), 1)),
            TokenCategory::UnquotedStringKnown(lit, s) => self.read_unquoted_string_known(lit, s, firstchar),
            TokenCategory::UnquotedStringUnknown => Err(self.read_unquoted_string_unknown(firstchar)),
            TokenCategory::Number => self.read_number(firstchar),
            TokenCategory::QuotedString => self.read_quoted_string(),
            TokenCategory::Invalid => Err((LexicalErrorKind::UnexpectedChar, firstchar.to_string())),
        };
        match result {
            Ok((kind, len)) => {
                let loc_end = CodeLocation::new(loc_start.line, loc_start.column + len);
                Some(Ok(Token::new(kind, CodeSpan::new(loc_start, loc_end))))
            }
            Err((kind, s)) => Some(Err(LexicalError::new(kind, s, loc_start))),
        }
    }

    // Reads a character if the character satisfies the predicate.
    fn read_char_if<F>(&mut self, pred: F) -> Option<char>
    where
        F: FnOnce(&char) -> bool,
    {
        self.chars.next_if(|(_, ch)| pred(ch)).map(|(_, ch)| ch)
    }

    // Reads an unquoted string.
    fn read_unquoted_string(&mut self, firstchar: char) -> String {
        let mut buf = firstchar.to_string();
        while let Some(ch) = self.read_char_if(|ch| ch.is_ascii_alphanumeric() || *ch == '_') {
            buf.push(ch);
        }
        buf
    }

    // Reads a known unquoted string.
    fn read_unquoted_string_known(
        &mut self,
        expected_literal: Literal,
        expected_str: &str,
        firstchar: char,
    ) -> Result<(TokenKind, usize), (LexicalErrorKind, String)> {
        let s = self.read_unquoted_string(firstchar);
        if s == expected_str {
            Ok((TokenKind::Literal(expected_literal), s.len()))
        } else {
            Err((LexicalErrorKind::UnquotedString, s))
        }
    }

    // Reads an unknown unquoted string.
    fn read_unquoted_string_unknown(&mut self, firstchar: char) -> (LexicalErrorKind, String) {
        let s = self.read_unquoted_string(firstchar);
        (LexicalErrorKind::UnquotedString, s)
    }

    // Reads digits.
    fn read_digits(&mut self, buf: &mut String) -> (usize, bool) {
        #[derive(PartialEq)]
        enum State {
            Initial,
            FirstCharIsZero,
            LeadingZeroDetected,
            LeadingZeroNotDetected,
        }
        let mut len: usize = 0;
        let mut state = State::Initial;
        while let Some(ch) = self.read_char_if(|ch| ch.is_ascii_digit()) {
            buf.push(ch);
            state = match state {
                State::Initial if ch == '0' => State::FirstCharIsZero,
                State::Initial => State::LeadingZeroNotDetected,
                State::FirstCharIsZero => State::LeadingZeroDetected,
                _ => state,
            };
            len += 1;
        }
        (len, state == State::LeadingZeroDetected)
    }

    // Reads a number.
    fn read_number(&mut self, firstchar: char) -> Result<(TokenKind, usize), (LexicalErrorKind, String)> {
        let mut buf = firstchar.to_string();
        let mut error = None;
        {
            let (len, leading_zero_detected) = self.read_digits(&mut buf);
            if firstchar == '-' && len == 0 {
                error = Some(LexicalErrorKind::NumberMissingIntegerDigits);
            } else if (firstchar == '0' && len > 0) || (firstchar == '-' && leading_zero_detected) {
                error = Some(LexicalErrorKind::NumberContainsLeadingZero);
            }
            // `firstchar == '-'` is equivalent to `!firstchar.is_ascii_digit()` in this method
        }
        if let Some(ch) = self.read_char_if(|ch| *ch == '.') {
            buf.push(ch);
            let (len, _) = self.read_digits(&mut buf);
            if len == 0 {
                error = Some(LexicalErrorKind::NumberMissingFractionDigits);
            }
        }
        if let Some(ch) = self.read_char_if(|ch| *ch == 'e' || *ch == 'E') {
            buf.push(ch);
            if let Some(ch) = self.read_char_if(|ch| *ch == '+' || *ch == '-') {
                buf.push(ch);
            }
            let (len, _) = self.read_digits(&mut buf);
            if len == 0 {
                error = Some(LexicalErrorKind::NumberMissingExponentDigits);
            }
        }
        match error {
            Some(kind) => Err((kind, buf)),
            None => {
                let len = buf.len();
                Ok((TokenKind::Literal(Literal::Number(buf)), len))
            }
        }
    }

    // Reads a quoted string.
    fn read_quoted_string(&mut self) -> Result<(TokenKind, usize), (LexicalErrorKind, String)> {
        let mut buf = String::new();
        let status = (|| {
            let mut error = None;
            loop {
                let (_, ch) = self.chars.next()?;
                match ch {
                    '"' => {
                        break;
                    }
                    '\\' => {
                        buf.push(ch);
                        let (_, ch) = self.chars.next()?;
                        buf.push(ch);
                        match ch {
                            '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' => {}
                            'u' => {
                                for _ in 0..4 {
                                    if let Some(ch) = self.read_char_if(|ch| ch.is_ascii_hexdigit()) {
                                        buf.push(ch);
                                    } else {
                                        error = Some(LexicalErrorKind::StringContainsInvalidUnicodeEscape);
                                        break;
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
            Some(None) => {
                let len = buf.len();
                Ok((TokenKind::Literal(Literal::String(buf)), len + 2))
            }
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
        assert_eq!(*token.kind(), expected_kind);
        assert_eq!(*token.span(), expected_span);
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
    fn tokenize_number_positive_integer_contains_zero() {
        let s = "103";
        take_single_valid_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_negative_integer() {
        let s = "-123";
        take_single_valid_token(s, TokenKind::Literal(Literal::Number(s.to_string())))
    }

    #[test]
    fn tokenize_number_negative_integer_contains_zero() {
        let s = "-103";
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
