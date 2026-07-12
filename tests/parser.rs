// Tests for Parser

use anyhow::Result;

use toy_json_parser::{Lexer, Parser, ValueKind};

#[test]
fn new() -> Result<()> {
    let lexer = Lexer::new("".chars());
    let _parser = Parser::new(lexer);
    Ok(())
}

fn do_parse_single_token(input: &str, expected_kind: ValueKind) -> Result<()> {
    let lexer = Lexer::new(input.chars());
    let mut parser = Parser::new(lexer);
    let value = parser.parse().unwrap();
    assert_eq!(value.kind, expected_kind);
    Ok(())
}

#[test]
fn parse_string() -> Result<()> {
    let s = "foo";
    let code = format!("\"{s}\"");
    do_parse_single_token(&code, ValueKind::String(s.to_string()))
}
