// Tests for Parser

use anyhow::{Result, bail};

use toy_json_parser::{CodePos, Lexer, Literal, Parser, ParserError, Value};

#[test]
fn new() -> Result<()> {
    let lexer = Lexer::new("".chars());
    let _parser = Parser::new(lexer);
    Ok(())
}

fn do_parse_legal_code(input: &str, expected: Value) -> Result<()> {
    let lexer = Lexer::new(input.chars());
    let mut parser = Parser::new(lexer);
    let value = parser.parse()?;
    assert_eq!(value, expected);
    Ok(())
}

fn do_parse_illegal_code(input: &str, expected: ParserError) -> Result<()> {
    let lexer = Lexer::new(input.chars());
    let mut parser = Parser::new(lexer);
    match parser.parse() {
        Ok(_) => {
            bail!("");
        }
        Err(e) => {
            assert_eq!(e, expected);
        }
    }
    Ok(())
}

#[test]
fn parse_number() -> Result<()> {
    let s = "123";
    do_parse_legal_code(s, Value::Literal(Literal::Number(s.to_string())))
}

#[test]
fn parse_string() -> Result<()> {
    let s = "foo";
    let code = format!("\"{s}\"");
    do_parse_legal_code(&code, Value::Literal(Literal::String(s.to_string())))
}

#[test]
fn parse_true() -> Result<()> {
    do_parse_legal_code("true", Value::Literal(Literal::Boolean(true)))
}

#[test]
fn parse_false() -> Result<()> {
    do_parse_legal_code("false", Value::Literal(Literal::Boolean(false)))
}

#[test]
fn parse_null() -> Result<()> {
    do_parse_legal_code("null", Value::Literal(Literal::Null))
}

#[test]
fn parse_array_empty() -> Result<()> {
    let input = "[]";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: 1,
        column: start.column + input.chars().count(),
    };
    let buf = Vec::new();
    do_parse_legal_code(input, Value::Array((buf, start..end)))
}

#[test]
fn parse_array_single_item() -> Result<()> {
    let input = "[0]";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: 1,
        column: start.column + input.chars().count(),
    };
    let mut buf = Vec::new();
    buf.push(Box::new(Value::Literal(Literal::Number("0".to_string()))));
    do_parse_legal_code(input, Value::Array((buf, start..end)))
}

#[test]
fn parse_array_multiple_item() -> Result<()> {
    let input = "[0, 1]";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: 1,
        column: start.column + input.chars().count(),
    };
    let mut buf = Vec::new();
    buf.push(Box::new(Value::Literal(Literal::Number("0".to_string()))));
    buf.push(Box::new(Value::Literal(Literal::Number("1".to_string()))));
    do_parse_legal_code(input, Value::Array((buf, start..end)))
}

#[test]
fn parse_object_empty() -> Result<()> {
    let input = "{}";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: 1,
        column: start.column + input.chars().count(),
    };
    let buf = Vec::new();
    do_parse_legal_code(input, Value::Object((buf, start..end)))
}

#[test]
fn parse_object_single_pair() -> Result<()> {
    let input = "{\"a\": 0}";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: 1,
        column: start.column + input.chars().count(),
    };
    let mut buf = Vec::new();
    buf.push(("a".to_string(), Box::new(Value::Literal(Literal::Number("0".to_string())))));
    do_parse_legal_code(input, Value::Object((buf, start..end)))
}

#[test]
fn parse_object_multiple_pair() -> Result<()> {
    let input = "{\"a\": 0, \"b\": 1}";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: 1,
        column: start.column + input.chars().count(),
    };
    let mut buf = Vec::new();
    buf.push(("a".to_string(), Box::new(Value::Literal(Literal::Number("0".to_string())))));
    buf.push(("b".to_string(), Box::new(Value::Literal(Literal::Number("1".to_string())))));
    do_parse_legal_code(input, Value::Object((buf, start..end)))
}

#[test]
fn parse_illegal_no_token() -> Result<()> {
    let input = "";
    do_parse_illegal_code(input, ParserError::NoToken)
}

#[test]
fn parse_illegal_extra_token_at_the_end() -> Result<()> {
    let body = "0 ";
    let extra = "1";
    let input = &format!("{body}{extra}");
    let pos = CodePos {
        line: 1,
        column: 1 + body.chars().count(),
    };
    do_parse_illegal_code(input, ParserError::ExtraTokenAtTheEnd(pos))
}
