// Tests for Lexer

use anyhow::{Context as _, Result};

use toy_json_parser::{CodeLocation, CodeSpan, Delimiter, Lexer, Literal, TokenKind};

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
