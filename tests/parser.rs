// Tests for Parser

use anyhow::Result;

use toy_json_parser::{Lexer, Parser, ValueKind};

#[test]
fn new() -> Result<()> {
    let lexer = Lexer::new("".chars());
    let _parser = Parser::new(lexer);
    Ok(())
}

#[test]
fn parse_string() -> Result<()> {
    let expected = "foo";
    let code = format!("\"{expected}\"");
    let lexer = Lexer::new(code.chars());
    let mut parser = Parser::new(lexer);
    let value = parser.parse().unwrap();
    assert!(matches!(value.kind, ValueKind::String(_)));
    Ok(())
}
