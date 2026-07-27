// Tests for Token

use anyhow::Result;

use toy_json_parser::{CodeLocation, CodeSpan, Delimiter, Literal, Token, TokenKind};

fn do_display_token_test(kind: TokenKind, expected: &str) -> Result<()> {
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + expected.chars().count());
    let span = CodeSpan::new(start, end);
    let target = Token::new(kind, span);
    assert_eq!(&target.to_string(), expected);
    Ok(())
}

#[test]
fn display_token_delimiter_left_bracket() -> Result<()> {
    let expected = "[";
    let kind = TokenKind::Delimiter(Delimiter::LeftBracket);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_delimiter_right_bracket() -> Result<()> {
    let expected = "]";
    let kind = TokenKind::Delimiter(Delimiter::RightBracket);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_delimiter_left_brace() -> Result<()> {
    let expected = "{";
    let kind = TokenKind::Delimiter(Delimiter::LeftBrace);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_delimiter_right_brace() -> Result<()> {
    let expected = "}";
    let kind = TokenKind::Delimiter(Delimiter::RightBrace);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_delimiter_colon() -> Result<()> {
    let expected = ":";
    let kind = TokenKind::Delimiter(Delimiter::Colon);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_delimiter_comma() -> Result<()> {
    let expected = ",";
    let kind = TokenKind::Delimiter(Delimiter::Comma);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_literal_number() -> Result<()> {
    let s = "123";
    let kind = TokenKind::Literal(Literal::Number(s.to_string()));
    do_display_token_test(kind, s)
}

#[test]
fn display_token_literal_string() -> Result<()> {
    let s = "foo";
    let expected = format!("\"{s}\"");
    let kind = TokenKind::Literal(Literal::String(s.to_string()));
    do_display_token_test(kind, &expected)
}

#[test]
fn display_token_literal_boolean_false() -> Result<()> {
    let expected = "false";
    let kind = TokenKind::Literal(Literal::Boolean(false));
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_literal_boolean_true() -> Result<()> {
    let expected = "true";
    let kind = TokenKind::Literal(Literal::Boolean(true));
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_literal_null() -> Result<()> {
    let expected = "null";
    let kind = TokenKind::Literal(Literal::Null);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_invalid() -> Result<()> {
    let s = "_";
    let kind = TokenKind::Invalid(s.to_string());
    do_display_token_test(kind, s)
}
