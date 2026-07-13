// Tests for Parser

use anyhow::Result;

use toy_json_parser::{Lexer, Parser, ValueKind};

#[test]
fn new() -> Result<()> {
    let lexer = Lexer::new("".chars());
    let _parser = Parser::new(lexer);
    Ok(())
}

fn do_parse_tokens(input: &str, expected_kind: ValueKind) -> Result<()> {
    let lexer = Lexer::new(input.chars());
    let mut parser = Parser::new(lexer);
    let value = parser.parse().unwrap();
    assert_eq!(value.kind, expected_kind);
    Ok(())
}

#[test]
fn parse_number() -> Result<()> {
    let s = "123";
    do_parse_tokens(s, ValueKind::Number(s.to_string()))
}

#[test]
fn parse_string() -> Result<()> {
    let s = "foo";
    let code = format!("\"{s}\"");
    do_parse_tokens(&code, ValueKind::String(s.to_string()))
}

#[test]
fn parse_true() -> Result<()> {
    do_parse_tokens("true", ValueKind::Boolean(true))
}

#[test]
fn parse_false() -> Result<()> {
    do_parse_tokens("false", ValueKind::Boolean(false))
}

#[test]
fn parse_null() -> Result<()> {
    do_parse_tokens("null", ValueKind::Null)
}

#[test]
fn parse_array_empty() -> Result<()> {
    do_parse_tokens("[]", ValueKind::Array(Vec::new()))
}

#[test]
fn parse_array_single_item() -> Result<()> {
    let mut buf = Vec::new();
    buf.push(Box::new(ValueKind::Number("0".to_string())));
    do_parse_tokens("[0]", ValueKind::Array(buf))
}

#[test]
fn parse_array_multiple_item() -> Result<()> {
    let mut buf = Vec::new();
    buf.push(Box::new(ValueKind::Number("0".to_string())));
    buf.push(Box::new(ValueKind::Number("1".to_string())));
    do_parse_tokens("[0, 1]", ValueKind::Array(buf))
}

#[test]
fn parse_object_empty() -> Result<()> {
    do_parse_tokens("{}", ValueKind::Object(Vec::new()))
}

#[test]
fn parse_object_single_pair() -> Result<()> {
    let mut buf = Vec::new();
    buf.push(("a".to_string(), Box::new(ValueKind::Number("0".to_string()))));
    do_parse_tokens("{\"a\": 0}", ValueKind::Object(buf))
}

#[test]
fn parse_object_multiple_pair() -> Result<()> {
    let mut buf = Vec::new();
    buf.push(("a".to_string(), Box::new(ValueKind::Number("0".to_string()))));
    buf.push(("b".to_string(), Box::new(ValueKind::Number("1".to_string()))));
    do_parse_tokens("{\"a\": 0, \"b\": 1}", ValueKind::Object(buf))
}
