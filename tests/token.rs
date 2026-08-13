// Tests for Token

use toy_json_parser::{CodeLocation, CodeSpan, Delimiter, Literal, Token, TokenKind};

fn do_display_token_test(kind: TokenKind, expected: &str) {
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + expected.chars().count());
    let span = CodeSpan::new(start, end);
    let target = Token::new(kind, span);
    assert_eq!(&target.to_string(), expected);
}

#[test]
fn display_token_delimiter_left_bracket() {
    let expected = "[";
    let kind = TokenKind::Delimiter(Delimiter::LeftBracket);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_delimiter_right_bracket() {
    let expected = "]";
    let kind = TokenKind::Delimiter(Delimiter::RightBracket);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_delimiter_left_brace() {
    let expected = "{";
    let kind = TokenKind::Delimiter(Delimiter::LeftBrace);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_delimiter_right_brace() {
    let expected = "}";
    let kind = TokenKind::Delimiter(Delimiter::RightBrace);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_delimiter_colon() {
    let expected = ":";
    let kind = TokenKind::Delimiter(Delimiter::Colon);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_delimiter_comma() {
    let expected = ",";
    let kind = TokenKind::Delimiter(Delimiter::Comma);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_literal_number() {
    let s = "123";
    let kind = TokenKind::Literal(Literal::Number(s.to_string()));
    do_display_token_test(kind, s)
}

#[test]
fn display_token_literal_string() {
    let s = "foo";
    let expected = format!(r#""{s}""#);
    let kind = TokenKind::Literal(Literal::String(s.to_string()));
    do_display_token_test(kind, &expected)
}

#[test]
fn display_token_literal_boolean_false() {
    let expected = "false";
    let kind = TokenKind::Literal(Literal::Boolean(false));
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_literal_boolean_true() {
    let expected = "true";
    let kind = TokenKind::Literal(Literal::Boolean(true));
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_literal_null() {
    let expected = "null";
    let kind = TokenKind::Literal(Literal::Null);
    do_display_token_test(kind, expected)
}

#[test]
fn display_token_invalid() {
    let s = "_";
    let kind = TokenKind::Invalid(s.to_string());
    do_display_token_test(kind, s)
}
