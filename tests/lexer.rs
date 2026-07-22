// Tests for Lexer

use std::ops::Range;

use anyhow::{Context as _, Result};

use toy_json_parser::{CodePos, Delimiter, Lexer, Literal, TokenKind};

fn do_tokenize(input: &str, expected_kind: TokenKind, expected_range: Range<CodePos>, check_finished: bool) -> Result<()> {
    let mut lexer = Lexer::new(input.chars());
    let token = lexer.next().with_context(|| "")?;
    assert_eq!(token.kind, expected_kind);
    assert_eq!(token.range, expected_range);
    if check_finished {
        assert_eq!(lexer.next(), None);
    }
    Ok(())
}

fn do_tokenize_single_token(input: &str, expected_kind: TokenKind) -> Result<()> {
    let start = CodePos::new(1, 1);
    let end = CodePos::new(start.line, start.column + input.chars().count());
    do_tokenize(input, expected_kind, start..end, true)
}

fn do_tokenize_single_token_with_whitespace_prefix(prefix: &str, input: &str, expected_kind: TokenKind, start: (usize, usize)) -> Result<()> {
    let start = CodePos::new(start.0, start.1);
    let end = CodePos::new(start.line, start.column + input.chars().count());
    do_tokenize(&format!("{prefix}{input}"), expected_kind, start..end, true)
}

fn do_tokenize_single_token_with_trailing_chars(body: &str, rest: &str, expected_kind: TokenKind) -> Result<()> {
    let input = &format!("{body}{rest}");
    let start = CodePos::new(1, 1);
    let end = CodePos::new(start.line, start.column + body.chars().count());
    do_tokenize(input, expected_kind, start..end, false)
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
    do_tokenize_single_token(&format!("\"{s}\""), TokenKind::Literal(Literal::String(s.to_string())))
}

#[test]
fn tokenize_string_with_escaped_char() -> Result<()> {
    let s = "\\\" \\\\ \\/ \\b \\f \\n \\r \\t";
    do_tokenize_single_token(&format!("\"{s}\""), TokenKind::Literal(Literal::String(s.to_string())))
}

#[test]
fn tokenize_string_with_escaped_unicode() -> Result<()> {
    let s = "\\u048c";
    do_tokenize_single_token(&format!("\"{s}\""), TokenKind::Literal(Literal::String(s.to_string())))
}

#[test]
fn skip_space() -> Result<()> {
    do_tokenize_single_token_with_whitespace_prefix(" ", ":", TokenKind::Delimiter(Delimiter::Colon), (1, 2))
}

#[test]
fn skip_tab() -> Result<()> {
    do_tokenize_single_token_with_whitespace_prefix("\t", ":", TokenKind::Delimiter(Delimiter::Colon), (1, 2))
}

#[test]
fn skip_line_feed() -> Result<()> {
    do_tokenize_single_token_with_whitespace_prefix("\n", ":", TokenKind::Delimiter(Delimiter::Colon), (2, 1))
}

#[test]
fn skip_carrige_return() -> Result<()> {
    do_tokenize_single_token_with_whitespace_prefix("\r", ":", TokenKind::Delimiter(Delimiter::Colon), (1, 2))
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
    let s = "\"\\c\"";
    do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
}

#[test]
fn tokenize_invalid_quoted_string_with_escaped_unicode() -> Result<()> {
    let s = "\"\\u000x\"";
    do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
}

#[test]
fn tokenize_invalid_quoted_string_unterminated() -> Result<()> {
    let s = "\"foo";
    do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
}
