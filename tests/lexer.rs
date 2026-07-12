// Tests for Lexer

use std::ops::Range;

use anyhow::{Context as _, Result};

use toy_json_parser::{CodePos, Lexer, TokenKind};

fn do_tokenize(input: &str, expected_kind: TokenKind, expected_range: Range<(usize, usize)>) -> Result<()> {
    let expected_pos_start = CodePos {
        line: expected_range.start.0,
        column: expected_range.start.1,
    };
    let expected_pos_end = CodePos {
        line: expected_range.end.0,
        column: expected_range.end.1,
    };
    let mut lexer = Lexer::new(input.chars());
    let token = lexer.next().with_context(|| "")?;
    assert_eq!(token.kind, expected_kind);
    assert_eq!(token.range.start, expected_pos_start);
    assert_eq!(token.range.end, expected_pos_end);
    assert_eq!(lexer.next(), None);
    Ok(())
}

fn do_tokenize_single_token(input: &str, expected_kind: TokenKind) -> Result<()> {
    let start = (1, 1);
    let end = (start.0, start.1 + input.chars().count());
    do_tokenize(input, expected_kind, start..end)
}

fn do_tokenize_single_token_with_whitespace_prefix(prefix: &str, input: &str, expected_kind: TokenKind, start: (usize, usize)) -> Result<()> {
    let end = (start.0, start.1 + input.chars().count());
    do_tokenize(&format!("{prefix}{input}"), expected_kind, start..end)
}

#[test]
fn new() -> Result<()> {
    let _lexer = Lexer::new("".chars());
    Ok(())
}

#[test]
fn tokenize_left_bracket() -> Result<()> {
    do_tokenize_single_token("[", TokenKind::LeftBracket)
}

#[test]
fn tokenize_right_bracket() -> Result<()> {
    do_tokenize_single_token("]", TokenKind::RightBracket)
}

#[test]
fn tokenize_left_brace() -> Result<()> {
    do_tokenize_single_token("{", TokenKind::LeftBrace)
}

#[test]
fn tokenize_right_brace() -> Result<()> {
    do_tokenize_single_token("}", TokenKind::RightBrace)
}

#[test]
fn tokenize_colon() -> Result<()> {
    do_tokenize_single_token(":", TokenKind::Colon)
}

#[test]
fn tokenize_comma() -> Result<()> {
    do_tokenize_single_token(",", TokenKind::Comma)
}

#[test]
fn tokenize_true() -> Result<()> {
    do_tokenize_single_token("true", TokenKind::Boolean(true))
}

#[test]
fn tokenize_false() -> Result<()> {
    do_tokenize_single_token("false", TokenKind::Boolean(false))
}

#[test]
fn tokenize_null() -> Result<()> {
    do_tokenize_single_token("null", TokenKind::Null)
}

#[test]
fn tokenize_number_positive_zero() -> Result<()> {
    let s = "0";
    do_tokenize_single_token(s, TokenKind::Number(s.to_string()))
}

#[test]
fn tokenize_number_negative_zero() -> Result<()> {
    let s = "-0";
    do_tokenize_single_token(s, TokenKind::Number(s.to_string()))
}

#[test]
fn tokenize_number_positive_integer() -> Result<()> {
    let s = "123";
    do_tokenize_single_token(s, TokenKind::Number(s.to_string()))
}

#[test]
fn tokenize_number_negative_integer() -> Result<()> {
    let s = "-123";
    do_tokenize_single_token(s, TokenKind::Number(s.to_string()))
}

#[test]
fn tokenize_string_without_escaped() -> Result<()> {
    let s = "foo";
    do_tokenize_single_token(&format!("\"{s}\""), TokenKind::String(s.to_string()))
}

#[test]
fn tokenize_string_with_escaped_char() -> Result<()> {
    let s = "\\\" \\\\ \\/ \\b \\f \\n \\r \\t";
    do_tokenize_single_token(&format!("\"{s}\""), TokenKind::String(s.to_string()))
}

#[test]
fn tokenize_string_with_escaped_unicode() -> Result<()> {
    let s = "\\u048c";
    do_tokenize_single_token(&format!("\"{s}\""), TokenKind::String(s.to_string()))
}

#[test]
fn skip_space() -> Result<()> {
    do_tokenize_single_token_with_whitespace_prefix(" ", ":", TokenKind::Colon, (1, 2))
}

#[test]
fn skip_tab() -> Result<()> {
    do_tokenize_single_token_with_whitespace_prefix("\t", ":", TokenKind::Colon, (1, 2))
}

#[test]
fn skip_line_feed() -> Result<()> {
    do_tokenize_single_token_with_whitespace_prefix("\n", ":", TokenKind::Colon, (2, 1))
}

#[test]
fn skip_carrige_return() -> Result<()> {
    do_tokenize_single_token_with_whitespace_prefix("\r", ":", TokenKind::Colon, (1, 2))
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
fn tokenize_invalid_quoted_string_with_escaped_char() -> Result<()> {
    let s = "\"\\c\"";
    do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
}

#[test]
fn tokenize_invalid_quoted_string_with_escaped_unicode() -> Result<()> {
    let s = "\"\\u000x\"";
    do_tokenize_single_token(s, TokenKind::Invalid(s.to_string()))
}
