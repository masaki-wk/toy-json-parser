use anyhow::{Context as _, Result};

use toy_json_parser::{CodePos, Lexer, TokenKind};

fn do_tokenize_single(input: &str, expected_kind: TokenKind, expected_line: usize, expected_column: usize) -> Result<()> {
    let mut lexer = Lexer::new(input.chars());
    let token = lexer.next().with_context(|| "")?;
    let expected_pos = CodePos {
        line: expected_line,
        column: expected_column,
    };
    assert_eq!(token.kind, expected_kind);
    assert_eq!(token.range.start, expected_pos);
    assert_eq!(lexer.next(), None);
    Ok(())
}

#[test]
fn new() -> Result<()> {
    let _lexer = Lexer::new("".chars());
    Ok(())
}

#[test]
fn tokenize_left_bracket() -> Result<()> {
    do_tokenize_single("[", TokenKind::LeftBracket, 1, 1)
}

#[test]
fn tokenize_right_bracket() -> Result<()> {
    do_tokenize_single("]", TokenKind::RightBracket, 1, 1)
}

#[test]
fn tokenize_left_brace() -> Result<()> {
    do_tokenize_single("{", TokenKind::LeftBrace, 1, 1)
}

#[test]
fn tokenize_right_brace() -> Result<()> {
    do_tokenize_single("}", TokenKind::RightBrace, 1, 1)
}

#[test]
fn tokenize_colon() -> Result<()> {
    do_tokenize_single(":", TokenKind::Colon, 1, 1)
}

#[test]
fn tokenize_comma() -> Result<()> {
    do_tokenize_single(",", TokenKind::Comma, 1, 1)
}

#[test]
fn tokenize_true() -> Result<()> {
    do_tokenize_single("true", TokenKind::Boolean(true), 1, 1)
}

#[test]
fn tokenize_false() -> Result<()> {
    do_tokenize_single("false", TokenKind::Boolean(false), 1, 1)
}

#[test]
fn tokenize_null() -> Result<()> {
    do_tokenize_single("null", TokenKind::Null, 1, 1)
}

#[test]
fn skip_space() -> Result<()> {
    do_tokenize_single(" :", TokenKind::Colon, 1, 2)
}

#[test]
fn skip_tab() -> Result<()> {
    do_tokenize_single("\t:", TokenKind::Colon, 1, 2)
}

#[test]
fn skip_line_feed() -> Result<()> {
    do_tokenize_single("\n:", TokenKind::Colon, 2, 1)
}

#[test]
fn skip_carrige_return() -> Result<()> {
    do_tokenize_single("\r:", TokenKind::Colon, 1, 2)
}

#[test]
fn tokenize_invalid_char() -> Result<()> {
    let s = ".";
    do_tokenize_single(s, TokenKind::Invalid(s.to_string()), 1, 1)
}

#[test]
fn tokenize_invalid_raw_string() -> Result<()> {
    let s = "invalid";
    do_tokenize_single(s, TokenKind::Invalid(s.to_string()), 1, 1)
}
